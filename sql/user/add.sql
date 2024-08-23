INSERT INTO
    public.users (login, name, password_hash)
VALUES
    ($1, $2, $3)
RETURNING id;