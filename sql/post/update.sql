UPDATE
    posts
SET
    title = $2, description = $3, content = $4
WHERE
    id = $1