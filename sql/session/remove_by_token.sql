DELETE FROM
    public.sessions
WHERE
    token = $1;