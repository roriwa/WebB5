CREATE TABLE IF NOT EXISTS "users"
(
    name          TEXT PRIMARY KEY UNIQUE,
    password_hash TEXT
);
CREATE TABLE IF NOT EXISTS "recipes"
(
    id            TEXT PRIMARY KEY UNIQUE,
    name          TEXT,
    author        TEXT,
    stars         INT,
    timeRequired  TEXT,
    summary       TEXT,
    description   TEXT,
    imageLocation TEXT,
    FOREIGN KEY (author)
        REFERENCES users (name)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "comments"
(
    author   TEXT,
    comment  TEXT,
    posted   INT,
    recipeId TEXT,
    FOREIGN KEY (author)
        REFERENCES "users" (name)
        ON DELETE CASCADE,
    FOREIGN KEY (recipeId)
        REFERENCES "recipes" (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "tags"
(
    tag      TEXT PRIMARY KEY,
    recipeId TEXT,
    FOREIGN KEY (recipeId)
        REFERENCES "recipes" (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "ingredients"
(
    amount   TEXT,
    type     TEXT,
    recipeId TEXT,
    FOREIGN KEY (recipeId)
        REFERENCES "recipes" (id)
        ON DELETE CASCADE
);

