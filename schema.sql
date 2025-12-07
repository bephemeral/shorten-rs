DROP TABLE IF EXISTS links;

CREATE OR REPLACE FUNCTION random_alphanumeric(length INT) RETURNS TEXT AS $$
DECLARE
    chars TEXT := 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    result TEXT := '';
    i INT;
BEGIN
    FOR i IN 1..length LOOP
        result := result || substr(chars, floor(random() * length(chars) + 1)::int, 1);
    END LOOP;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS links (
    id CHAR(4) PRIMARY KEY DEFAULT random_alphanumeric(4),
    url TEXT NOT NULL
);
