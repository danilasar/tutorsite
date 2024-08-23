DELETE FROM
    public.sessions
WHERE
    user_id = $1;