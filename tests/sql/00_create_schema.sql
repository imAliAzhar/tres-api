DROP TABLE IF EXISTS records;

create table records (
    date TIMESTAMP WITH TIME ZONE NOT NULL PRIMARY KEY,
    duration_str VARCHAR(255),
    duration_ms INTEGER
);
