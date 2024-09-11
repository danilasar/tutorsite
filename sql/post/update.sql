UPDATE
    posts
SET
    title = $2, description = $3, md_file = $4
WHERE
    id = $1