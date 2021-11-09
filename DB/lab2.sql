/*
  3. Показать без повторений идентификаторы книг, которые были взяты читателями.
*/
SELECT DISTINCT `sb_book`
FROM `subscriptions`;

/*
  5. Показать, сколько всего читателей зарегистрировано в библиотеке.
*/
SELECT COUNT(`s_id`) AS `subscribers_count`
FROM `subscribers`;

/*
  7.  Показать, сколько читателей брало книги в библиотеке.
*/
SELECT COUNT(DISTINCT `sb_subscriber`) AS `subscribers_count`
FROM `subscriptions`;

/****************************************************************/

/*
  12. Показать идентификатор одного (любого) читателя,
    взявшего в библиотеке больше всего книг.
*/
SELECT `sb_subscriber`, COUNT(`sb_subscriber`) AS `books_amount`
FROM `subscriptions`
GROUP BY `sb_subscriber`
ORDER BY `books_amount` DESC
LIMIT 1;

/*
  15. Показать, сколько в среднем экземпляров книг есть в библиотеке.
  NOTE. Не уверен, что именно подразумевается под средним, поэтому сделаю еще одно задание.
*/
SELECT SUM(`b_quantity`) / COUNT(`b_id`) AS `books_average`
FROM `books`;

/*
  16. Показать в днях, сколько в среднем времени читатели уже зарегистрированы в библиотеке
  (временем регистрации считать диапазон от первой даты получения читателем книги до текущей даты).
  NOTE. Читатели без книг считаются незарегестрированными (один тут есть такой)
*/
SELECT AVG(`sb_registred`) as `sb_registred_avg`
FROM (
  SELECT DATEDIFF(CURDATE(), MIN(`sb_start`)) AS `sb_registred`
  FROM `subscriptions`
  GROUP BY `sb_subscriber`
) AS `useless`;

/*
  17. Показать, сколько книг было возвращено и не возвращено в библиотеку
    (СУБД должна оперировать исходными значениями поля sb_is_active (т.е. «Y» и «N»),
    а после подсчёта значения «Y» и «N» должны быть преобразованы в «Returned» и «Not returned»).
*/
SELECT IF(`sb_is_active` = 'Y', 'Not Returned', 'Returned') AS `sb_returned`,
  COUNT(*) as `sb_count`
FROM `subscriptions`
GROUP BY `sb_returned`
ORDER BY `sb_book` ASC;
