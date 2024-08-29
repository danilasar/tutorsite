INSERT INTO
    posts(title, description, content)
VALUES
    ($1, $2, $3)
RETURNING
    id