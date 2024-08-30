UPDATE
    posts
SET
    title = $2, description = $3, content = $4, content_html = $5
WHERE
    id = $1