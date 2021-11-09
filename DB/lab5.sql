/*
  1. Создать представление, позволяющее получать список читателей с количеством находящихся у каждого читателя на руках книг,
  но отображающее только таких читателей, по которым имеются задолженности, т.е. на руках у читателя есть хотя бы одна книга,
  которую он должен был вернуть до наступления текущей даты.
*/
CREATE SQL SECURITY INVOKER VIEW Indebted_User AS
SELECT `s_id`, `s_name`, COUNT(*) as `indebted_books`
FROM (
  SELECT `s_id`, `s_name`, `sb_finish`, `sb_is_active`
  FROM `subscribers`
  JOIN `subscriptions` ON (`sb_subscriber` = `s_id`)
) AS `useless`
WHERE CURDATE() > `sb_finish`
  AND `sb_is_active` = 'Y'
GROUP BY `s_id`;

/*
  7. Создать представление, извлекающее информацию о датах выдачи и возврата книг и состоянии выдачи книги в виде единой строки
    в формате «ГГГГ-ММ-ДД - ГГГГ-ММ-ДД - Возвращена» и при этом допускающее обновление информации в таблице subscriptions.
*/
CREATE SQL SECURITY INVOKER VIEW Indebty_Users AS
SELECT `sb_id`, `sb_start`, `sb_finish`, `sb_is_active`,
  CONCAT(
    DATE_FORMAT(`sb_start`, '%Y-%m-%d'), ' - ',
    DATE_FORMAT(`sb_finish`, '%Y-%m-%d'), ' - ',
    IF(`sb_is_active` = 'Y', 'Не Возвращена', 'Возвращена')
  ) AS `sb_info`
FROM `subscriptions`;


/*
  12. Модифицировать схему базы данных таким образом, чтобы таблица «subscribers» хранила информацию о том,
    сколько раз читатель брал в библиотеке книги (этот счётчик должен инкрементироваться каждый раз,
    когда читателю выдаётся книга; уменьшение значения этого счётчика не предусмотрено).
*/
ALTER TABLE `subscribers`
ADD `s_taken_no` INT NOT NULL;

DROP TRIGGER `tr_subscriptions_after_insert`;

DELIMITER //

CREATE TRIGGER `tr_subscriptions_after_insert` AFTER INSERT ON `subscriptions`
FOR EACH ROW
BEGIN
  UPDATE `subscribers`
    SET `s_taken_no` = `s_taken_no` + 1
    WHERE `s_id` = NEW.sb_subscriber;
END;

//
DELIMITER ;


/*
  13. Создать триггер, не позволяющий добавить в базу данных информацию о выдаче книги, если выполняется хотя бы одно из условий:
  * дата выдачи или возврата приходится на воскресенье;
  * читатель брал за последние полгода более 100 книг;
  * промежуток времени между датами выдачи и возврата менее трёх дней.
*/
DROP TRIGGER `tr_subscriptions_before_insert`;

DELIMITER //

CREATE TRIGGER `tr_subscriptions_before_insert` BEFORE INSERT on `subscriptions`
FOR EACH ROW BEGIN
  DECLARE exceeded_limit BOOLEAN;

  IF DAYOFWEEK(NEW.`sb_finish`) = 1 OR DAYOFWEEK(NEW.`sb_start`) = 1 THEN
    SET @msg = CONCAT('Sunday ', NEW.`sb_start`, ' is a weekend.');
    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = @msg, MYSQL_ERRNO = 1000;
  END IF;

  SET exceeded_limit := (
    SELECT COUNT(*)
    FROM `subscriptions`
    WHERE
      `sb_start` >= DATE_SUB(CURDATE(), INTERVAL 6 MONTH)
      AND `sb_subscriber` = NEW.`sb_subscriber`
  ) > 100;

  IF exceeded_limit THEN
    SET @msg = 'Cannot take more than a hundred books per half-a-year';
    SIGNAL SQLSTATE '45001' SET MESSAGE_TEXT = @msg, MYSQL_ERRNO = 1001;
  END IF;

  IF NEW.`sb_start` > DATE_SUB(NEW.`sb_finish`, INTERVAL 3 DAY) THEN
    SET @msg = 'Cannot take a book for less then 3 days';
    SIGNAL SQLSTATE '45001' SET MESSAGE_TEXT = @msg, MYSQL_ERRNO = 1002;
  END IF;
END;

//
DELIMITER ;


/*
  17. Создать триггер, меняющий дату выдачи книги на текущую, если указанная
    в INSERT- или UPDATE-запросе дата выдачи книги меньше текущей на полгода и более.
*/
DROP TRIGGER `tr_subscriptions_before_insert`;
DROP TRIGGER `tr_subscriptions_before_update`;

DELIMITER //

CREATE TRIGGER `tr_subscriptions_before_insert` BEFORE INSERT ON `subscriptions`
FOR EACH ROW BEGIN
  IF NEW.`sb_start` <= DATE_SUB(CURDATE(), INTERVAL 6 MONTH) THEN
    SET NEW.`sb_start` = CURDATE();
  END IF;
END;

//

CREATE TRIGGER `tr_subscriptions_before_update` BEFORE UPDATE ON `subscriptions`
FOR EACH ROW BEGIN
  IF NEW.`sb_start` <= DATE_SUB(CURDATE(), INTERVAL 6 MONTH) THEN
    SET NEW.`sb_start` = CURDATE();
  END IF;
END;

//

DELIMITER ;
