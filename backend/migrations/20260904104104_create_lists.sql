CREATE TABLE lists (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY NOT NULL,
    list_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL,

    FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE
);

CREATE INDEX idx_tasks_list_id
    ON tasks(list_id);
