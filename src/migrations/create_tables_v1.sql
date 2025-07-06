CREATE TABLE IF NOT EXISTS users (
    id SERIAL CONSTRAINT PK_users PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_users_uid UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username VARCHAR(255) NOT NULL CONSTRAINT UN_users_username UNIQUE,
    email VARCHAR(255) NOT NULL CONSTRAINT UN_users_email UNIQUE,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL CONSTRAINT PK_posts PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_posts_uid UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    slug VARCHAR(255) NOT NULL CONSTRAINT UN_posts_slug UNIQUE,
    title VARCHAR(255) NOT NULL,
    summary TEXT,
    content TEXT,
    status INTEGER NOT NULL DEFAULT 0,
    published_at TIMESTAMPTZ,
    author_id INTEGER,
    CONSTRAINT FK_posts_author_id FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

-- CREATE INDEX IDX_posts_author_id ON posts(author_id);
-- CREATE INDEX IDX_posts_status ON posts(status);
-- CREATE INDEX IDX_posts_published_at ON posts(published_at DESC);
-- CREATE INDEX IDX_posts_author_id_status ON posts(author_id, status);
