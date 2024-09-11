INSERT INTO
    posts(title, description, md_file)
VALUES
    ($1, $2, $3)
RETURNING
    id