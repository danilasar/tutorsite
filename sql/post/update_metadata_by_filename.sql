UPDATE
    public.posts
SET
    title = $2, description = $3
WHERE
    md_file = $1