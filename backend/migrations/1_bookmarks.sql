CREATE TABLE IF NOT EXISTS "bookmarks"
(
    recipeId TEXT,
    user     TEXT,
    FOREIGN KEY (recipeId)
        REFERENCES recipes (id)
        ON DELETE CASCADE,
    FOREIGN KEY (user)
        REFERENCES users (name)
        ON DELETE CASCADE
);
