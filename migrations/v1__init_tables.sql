CREATE TABLE profiles (
    id INTEGER PRIMARY KEY
);

CREATE TABLE dotfiles (
    id INTEGER PRIMARY KEY,
    source TEXT NOT NULL,
    description TEXT
);
