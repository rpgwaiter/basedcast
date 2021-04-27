-- Your SQL goes here
CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    track INT NULL,
    game TEXT NOT NULL,
    artist TEXT NULL,
    year INT NOT NULL,
    system TEXT NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT 't',
    bitrate INT NOT NULL,
    duration INT NOT NULL,
    filesize INT NOT NULL,
    filename TEXT NOT NULL,
    fullpath TEXT NOT NULL,
    hash TEXT NOT NULL
);
ALTER TABLE songs ADD CONSTRAINT song_hash UNIQUE (hash)