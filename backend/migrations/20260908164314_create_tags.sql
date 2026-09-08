CREATE TABLE tags (
    id TEXT NOT NULL PRIMARY KEY,
    list_id TEXT NOT NULL,
    name TEXT NOT NULL,
    color_key TEXT NOT NULL
);

CREATE TABLE task_tags (
    task_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    
    PRIMARY KEY (task_id, tag_id),
    
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
