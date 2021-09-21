-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER NOT NULL PRIMARY KEY,
    text TEXT NOT NULL,
    completed INTEGER NOT NULL CHECK (completed IN (0, 1))
)