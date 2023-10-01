DROP INDEX IF EXISTS posts.idx_user_id;

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS posts;

CREATE TABLE users (
    id         CHAR(36)    NOT NULL,
    username   VARCHAR(32) NOT NULL UNIQUE,
    created_at DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

CREATE TABLE posts (
    id         CHAR(36)     NOT NULL,
    user_id    CHAR(36)     NOT NULL,
    message    VARCHAR(256) NOT NULL,
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX idx_user_id ON posts (user_id);