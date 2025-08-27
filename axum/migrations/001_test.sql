
CREATE TABLE users (
    id SERIAL PRIMARY KEY,           -- auto-incrementing id
    name TEXT NOT NULL,              -- user’s name
    email TEXT UNIQUE NOT NULL   -- must be unique
);
