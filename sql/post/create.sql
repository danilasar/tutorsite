INSERT INTO
    posts(title, description, content, content_html)
VALUES
    ($1, $2, $3, $4)
RETURNING
    id