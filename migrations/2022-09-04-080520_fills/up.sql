CREATE TABLE fills (
    id INTEGER NOT NULL,
    amount REAL NOT NULL,
    date DATETIME NOT NULL,
    bucket_id INTEGER NOT NULL,
    PRIMARY KEY(id AUTOINCREMENT),
    FOREIGN KEY(bucket_id) REFERENCES buckets(id)
);
