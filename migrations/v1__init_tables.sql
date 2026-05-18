CREATE TABLE profiles (
    id INTEGER PRIMARY KEY
    name TEXT NOT NULL
);

CREATE TABLE dotfiles (
    id INTEGER PRIMARY KEY,
    source TEXT NOT NULL,
    description TEXT
);

CREATE TABLE dotfile_profiles (
    profile_id INTEGER,
    dotfile_id INTEGER
)
