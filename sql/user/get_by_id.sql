SELECT
    id, login, name, password_hash
FROM
    public.users
WHERE
    id = $1;