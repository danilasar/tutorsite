SELECT
    id, login, name, password_hash
FROM
    public.sessions AS sess
LEFT JOIN
    public.users AS users
ON
    sess.user_id = users.id
WHERE
    sess.user_id = users.id  AND sess.token = $1;