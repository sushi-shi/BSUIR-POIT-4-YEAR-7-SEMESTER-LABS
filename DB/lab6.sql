/*
  1. Создать хранимую функцию, получающую на вход идентификатор читателя и возвращающую
    список идентификаторов книг, которые он уже прочитал и вернул в библиотеку.
  NOTE. Не можем в MariaDB вернуть таблицу из функции, увы!
  NOTE. NULL, если id не в таблице.
*/

DELIMITER //

CREATE OR REPLACE FUNCTION RETURNED_BOOKS(subscriber_id INT)
RETURNS TEXT
DETERMINISTIC
BEGIN
  DECLARE returned TEXT;

  /* NULL if not in the table */
  IF NOT EXISTS (SELECT `s_id` FROM `subscribers` WHERE s_id = subscriber_id) THEN
    RETURN NULL;
  END IF;

  SET returned := (
    SELECT GROUP_CONCAT(`sb_book` SEPARATOR ', ')
    FROM `subscribers`
    JOIN `subscriptions` ON (`s_id` = `sb_subscriber`)
    WHERE `sb_is_active` = 'N'
      AND `s_id` = subscriber_id
  );

  /* Empty String if didn't return anything */
  RETURN (IF(returned IS NULL, '', returned));
END;

//
DELIMITER ;

/*
  3. Создать хранимую функцию, получающую на вход идентификатор читателя и возвращающую 1,
  если у читателя на руках сейчас менее десяти книг, и 0 в противном случае.
*/

DELIMITER //

CREATE OR REPLACE FUNCTION HAS_SOME_BOOKS(subsriber_id INT)
RETURNS BOOLEAN
DETERMINISTIC
BEGIN
  IF NOT EXISTS (SELECT `s_id` FROM `subscribers` WHERE s_id = subsriber_id) THEN
    RETURN NULL;
  END IF;

  RETURN (
    SELECT COUNT(*)
    FROM `subscribers`
    JOIN `subscriptions` ON (`s_id` = `sb_subscriber`)
    WHERE `sb_is_active` = 'Y'
      AND `s_id` = subsriber_id
  ) < 10;
END;

//
DELIMITER ;


/*
  4. Создать хранимую функцию, получающую на вход год издания книги и возвращающую 1,
  если книга издана менее ста лет назад, и 0 в противном случае.
*/
DELIMITER  //

CREATE OR REPLACE FUNCTION CONTEMPORARY(year INT)
RETURNS BOOLEAN
DETERMINISTIC
BEGIN
    RETURN year + 100 > YEAR(NOW());
END;

//
DELIMITER ;


/*
  5. Создать хранимую процедуру, обновляющую все поля типа DATE (если такие есть)
  всех записей указанной таблицы на значение текущей даты.
*/
DELIMITER //

CREATE OR REPLACE PROCEDURE SET_CURRENT_DATE(timetable TEXT)
BEGIN
  DECLARE done BOOL DEFAULT FALSE;
  DECLARE date_column TEXT;
  DECLARE date_columns CURSOR FOR
    SELECT column_name FROM INFORMATION_SCHEMA.COLUMNS
    WHERE TABLE_NAME = timetable
      AND DATA_TYPE = 'date';
  DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

  IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = timetable) THEN
    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'The table does not exists', MYSQL_ERRNO = 1000;
  END IF;

  OPEN date_columns;

  while_loop: LOOP
    FETCH date_columns INTO date_column;
    IF done THEN
      LEAVE while_loop;
    END IF;

    SET @update_query = CONCAT('UPDATE ', timetable, ' SET ', date_column, ' = NOW()');
    PREPARE query FROM @update_query;
    EXECUTE query;
    DEALLOCATE PREPARE query;
  END LOOP while_loop;

  CLOSE date_columns;

END;

//
DELIMITER ;

/*
  9. Создать хранимую процедуру, автоматически создающую и наполняющую данными таблицу «arrears»,
  в которой должны быть представлены идентификаторы и имена читателей, у которых до сих пор находится на руках хотя бы одна книга,
  по которой дата возврата установлена в прошлом относительно текущей даты.
*/

DELIMITER //

CREATE OR REPLACE PROCEDURE ARREARS()
BEGIN
  DROP TABLE IF EXISTS `arrears`;
  CREATE TABLE `arrears` (
    `ar_id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `ar_subscriber` INT UNSIGNED NOT NULL,
    `ar_name` VARCHAR(150) NOT NULL,
    `ar_book` INT UNSIGNED NOT NULL,
    CONSTRAINT `PK_arrears` PRIMARY KEY (`ar_id` ASC)
  );
  INSERT INTO `arrears` (`ar_subscriber`, `ar_name`, `ar_book`)
  SELECT `s_id`, `s_name`, `sb_book`
  FROM `subscribers`
  JOIN `subscriptions` ON (`s_id` = `sb_subscriber`)
  WHERE `sb_finish` < CURDATE();
END;

//
DELIMITER ;

