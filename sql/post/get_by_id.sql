SELECT
    id, title, description, md_file
FROM
    posts
WHERE
    id = $1