
/* 
  6. Создать нового пользователя СУБД и предоставить ему права на базу данных «Библиотека», 
  допускающие только выборку данных (в т.ч. через представления и с вызовом хранимы функций).
*/
DROP USER ''@'localhost';
CREATE USER 'helpme';
GRANT EXECUTE ON db_labs_library.* TO 'helpme';
GRANT SELECT ON db_labs_library.* TO 'helpme';


/* 
  7. Написать хранимую процедуру, получающую на вход имя таблицы, имя поля, исходную и целевую кодировку, 
  и конвертирующую данные в этом поле этой таблицы из исходной кодировки в целевую.
*/
DELIMITER //

CREATE OR REPLACE PROCEDURE CHANGE_ENCODING(
  table_name TEXT,
  field_name TEXT,
  to_enc TEXT
)
BEGIN
  SET @query := CONCAT(
    'ALTER TABLE ', table_name, 
    ' MODIFY ', field_name,
    ' TEXT CHARACTER SET ', to_enc
  );

  PREPARE stmt FROM @query;
  EXECUTE stmt;
  DEALLOCATE PREPARE stmt;

END;

//
DELIMITER ;

/*
  8. Написать хранимую функцию, формирующую набор из указанного количества случайных 
  гарантированно неповторяющихся идентификаторов читателей.
  NOTE. 
    Опять-таки, если необходимо решать по книге, то MySQL не поддежривает возвращение 
    таблиц, что подразумевает, что мы можем вернуть строку.
    А для случайных значений нам не нужны никакие CROSS JOIN'ы, достаточно простой случайной сортировки
  NOTE.
    Хранимые функции не поддерживают dynamic sql.
    А обычный SELECT не получилось, потому что LIMIT не поддерживает индексы внутри.
*/
DELIMITER //
CREATE OR REPLACE PROCEDURE RANDOM_SUBSCRIBERS(
  no INT, 
  OUT res TEXT
)
BEGIN
  CREATE TEMPORARY TABLE result (ids TEXT); 

  SET @query := CONCAT(
    'INSERT INTO result ',
    'SELECT GROUP_CONCAT(`s_id`) ',
    'FROM ( ',
      'SELECT `s_id` ',
      'FROM `subscribers` ',
      'ORDER BY RAND() ',
      'LIMIT ? ',
    ') as `useless` '
  );
  PREPARE stmt FROM @query;
  EXECUTE stmt USING no;
  DEALLOCATE PREPARE stmt;

  SET res := (SELECT * FROM result);
  DROP TABLE result;
END;
//
DELIMITER ;

CALL RANDOM_SUBSCRIBERS(5, @msg);
SELECT @msg;



/*
  10. Написать хранимую функцию, получающую на вход идентификатор читателя и возвращающую 
  в формате JSON список всех прочитанных им книг (с указанием авторов и жанров по каждой книге).
*/

DELIMITER //

CREATE OR REPLACE FUNCTION RETURNED_BOOKS(subscriber_id INT)
RETURNS JSON
DETERMINISTIC
BEGIN
  DECLARE returned JSON;

  /* NULL if not in the table */
  IF NOT EXISTS (SELECT `s_id` FROM `subscribers` WHERE s_id = subscriber_id) THEN
    RETURN NULL;
  END IF;

  SET returned := (
    SELECT JSON_ARRAYAGG(
      JSON_OBJECT('a_name', authors.a_name, 'g_name', genres.g_name)
    )
    FROM (
      SELECT sb_book
      FROM subscriptions
      WHERE sb_subscriber = subscriber_id
        AND sb_is_active = 'N'
    ) `useless`
    JOIN books ON (b_id = sb_book)
    JOIN m2m_books_genres USING (b_id)
    JOIN genres USING (g_id)
    JOIN m2m_books_authors USING (b_id)
    JOIN authors USING (a_id)
  );

  RETURN returned;
END;

//
DELIMITER ;

