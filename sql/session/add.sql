INSERT INTO
    public.sessions (token, user_id)
VALUES
    ($1, $2)
RETURNING
    expires;