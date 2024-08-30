SELECT
    id, title, description, content, content_html
FROM
    posts
WHERE
    id = $1