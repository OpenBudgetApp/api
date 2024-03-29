CREATE TABLE transactions (
    id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    amount REAL NOT NULL,
    date DATETIME NOT NULL,
    account_id INTEGER NOT NULL,
    bucket_id INTEGER,
    PRIMARY KEY(id AUTOINCREMENT),
    FOREIGN KEY(account_id) REFERENCES accounts(id)
    FOREIGN KEY(bucket_id) REFERENCES buckets(id)
);
