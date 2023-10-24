-- Add up migration script here

CREATE TABLE IF NOT EXISTS records
(
    date TIMESTAMP WITH TIME ZONE NOT NULL PRIMARY KEY,
    duration_str VARCHAR(255),
    duration_ms INTEGER
);