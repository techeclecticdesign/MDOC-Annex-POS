 CREATE TABLE categories (
   id       INTEGER PRIMARY KEY AUTOINCREMENT,
   name     TEXT NOT NULL UNIQUE,
   deleted  DATETIME
 );