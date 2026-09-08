CREATE TABLE invitations (
    id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    list_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,

    PRIMARY KEY (id),

    FOREIGN KEY (account_id)
        REFERENCES accounts(id)
        ON DELETE CASCADE,

    FOREIGN KEY (list_id)
        REFERENCES lists(id)
        ON DELETE CASCADE
);
