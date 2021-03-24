-- Your SQL goes here
CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    track INT NULL,
    game TEXT NULL,
    artist TEXT NULL,
    year INT NOT NULL,
    system TEXT NULL,
    is_public BOOLEAN NOT NULL DEFAULT 't',
    bitrate INT NOT NULL,
    duration INT NOT NULL, -- Will need to be converted to int from mediainfo's decimal
    filesize INT NOT NULL,
    filename TEXT NOT NULL,
    fullpath TEXT NOT NULL,
    hash UUID NOT NULL
)