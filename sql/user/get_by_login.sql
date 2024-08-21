SELECT
    id, login, name, password_hash
FROM
    public.users
WHERE
    login = $1;