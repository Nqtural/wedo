CREATE TABLE list_membership (
    list_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,

    PRIMARY KEY (list_id, user_id),

    FOREIGN KEY (list_id)
        REFERENCES lists(id)
        ON DELETE CASCADE,

    FOREIGN KEY (user_id)
        REFERENCES accounts(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_list_membership_user_id
    ON list_membership(user_id);

CREATE TRIGGER delete_orphaned_list
AFTER DELETE ON list_membership
WHEN NOT EXISTS (
    SELECT 1
    FROM list_membership
    WHERE list_id = OLD.list_id
)
BEGIN
    DELETE FROM lists
    WHERE id = OLD.list_id;
END;
