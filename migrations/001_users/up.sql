
-- Your SQL goes here

DROP TYPE IF EXISTS Role;
CREATE TYPE Role AS ENUM (
    'admin', 'user'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    name varchar(20) NOT NULL,
    tel_number  varchar(11) NOT NULL,
    role  Role NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

INSERT INTO users (name, role, tel_number)
VALUES ('test1', 'admin', '12345678900');

INSERT INTO users (name, role, tel_number)
VALUES ('test1', 'user', '12345678901');