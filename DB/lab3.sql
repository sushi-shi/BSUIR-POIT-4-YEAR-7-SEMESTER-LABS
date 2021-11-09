/*
  1. Показать список книг, у которых более одного автора.
*/
SELECT `b_name`
FROM `books`
JOIN `m2m_books_authors` USING (`b_id`)
GROUP BY `b_id`
HAVING COUNT(*) > 1;

/*
  3.  Показать все книги с их жанрами (дублирование названий книг не допускается).
  NOTE. Я правильно понимаю, что под "не допускается" имеется ввиду "должны быть в разных рядах"?
*/
SELECT `b_name`, GROUP_CONCAT(`g_name`) AS `genres`
FROM `books`
JOIN `m2m_books_genres` USING (`b_id`)
JOIN `genres` USING (`g_id`)
GROUP BY `b_id`;

/*
  5. Показать список книг, которые когда-либо были взяты читателями.
*/
SELECT `b_name`
FROM `books`
JOIN `subscriptions` ON (`b_id` = `sb_book`)
GROUP BY `b_id`;

/*
  15. Показать всех авторов и количество книг (не экземпляров книг,
    а «книг как изданий») по каждому автору.
*/
SELECT `a_id`, `a_name`, COUNT(`b_id`) AS `books_quantity`
FROM `authors`
JOIN `m2m_books_authors` USING (`a_id`)
GROUP BY `a_id`;

/*
  17. Показать читаемость жанров, т.е. все жанры и то количество раз,
    которое книги этих жанров были взяты читателями.
*/
SELECT `g_name`, COUNT(*) as `g_read`
FROM `genres`
JOIN `m2m_books_genres` USING (`g_id`)
JOIN `books` USING (`b_id`)
JOIN `subscriptions` ON (`b_id` = `sb_book`)
GROUP BY `g_name`
ORDER BY `g_read` DESC;

/*
  22. Показать читателей наибольшего количества жанров
    (не важно, брали ли они книги, каждая из которых относится одновременно к многим жанрам,
    или же просто много книг из разных жанров,
    каждая из которых относится к небольшому количеству жанров).
*/
SELECT `s_id`, `s_name`, COUNT(*) as `g_read`
FROM (
  SELECT `s_id`, `s_name`, `g_id`
  FROM `subscribers`
  JOIN `subscriptions` ON (`s_id` = `sb_subscriber`)
  JOIN `books` ON (`sb_book` = `b_id`)
  JOIN `m2m_books_genres` USING (`b_id`)
  GROUP BY `s_id`, `g_id`
) AS `useless`
GROUP BY `s_id`;
