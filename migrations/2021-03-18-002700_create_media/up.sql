-- Your SQL goes here
CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    system VARCHAR NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT 't',
    bitrate INT DEFAULT NULL,
    duration INT DEFAULT NULL, -- Will need to be converted to int from mediainfo's decimal
    filesize INT NOT NULL,
    filename VARCHAR NOT NULL,
    fullpath VARCHAR NOT NULL
)