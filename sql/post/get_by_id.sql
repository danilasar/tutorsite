SELECT
    id, title, description, content
FROM
    posts
WHERE
    id = $1