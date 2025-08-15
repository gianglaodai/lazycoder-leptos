CREATE TABLE IF NOT EXISTS users (
    id SERIAL CONSTRAINT PK_users PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_users_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username VARCHAR(255) NOT NULL CONSTRAINT UN_users_username UNIQUE,
    email VARCHAR(255) NOT NULL CONSTRAINT UN_users_email UNIQUE,
    password VARCHAR(255) NOT NULL,
    role INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT UN_users_username UNIQUE (username),
    CONSTRAINT UN_users_email UNIQUE (email),
    CONSTRAINT UN_users_uid UNIQUE (uid)
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL CONSTRAINT PK_posts PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_posts_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    slug VARCHAR(255) NOT NULL CONSTRAINT UN_posts_slug UNIQUE,
    title VARCHAR(255) NOT NULL,
    summary TEXT,
    content TEXT,
    status INTEGER NOT NULL DEFAULT 0,
    published_at TIMESTAMPTZ,
    user_id INTEGER NOT NULL,
    CONSTRAINT FK_posts_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT UN_posts_slug UNIQUE (slug),
    CONSTRAINT UN_posts_uid UNIQUE (uid)
);

CREATE INDEX IF NOT EXISTS IDX_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS IDX_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS IDX_posts_published_at ON posts(published_at DESC);
CREATE INDEX IF NOT EXISTS IDX_posts_user_id_status ON posts(user_id, status);

CREATE TABLE IF NOT EXISTS attributes (
    id SERIAL CONSTRAINT PK_attributes PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_attributes_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name VARCHAR(255) NOT NULL CONSTRAINT UN_attributes_name UNIQUE,
    entity_type VARCHAR(255) NOT NULL,
    data_type VARCHAR(255) NOT NULL,
    CONSTRAINT UN_attributes_entity_type_name UNIQUE (entity_type, name),
    CONSTRAINT UN_attributes_uid UNIQUE (uid)
);

CREATE TABLE IF NOT EXISTS attribute_values (
    id SERIAL CONSTRAINT PK_attributes_values PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_attribute_values_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    int_value INTEGER,
    double_value DOUBLE PRECISION,
    string_value TEXT,
    boolean_value BOOLEAN,
    date_value DATE,
    datetime_value TIMESTAMPTZ,
    time_value TIME,
    attribute_id INTEGER NOT NULL,
    entity_id INTEGER NOT NULL,
    entity_type VARCHAR(255) NOT NULL,
    CONSTRAINT FK_attribute_values_attributes_id FOREIGN KEY (attribute_id) REFERENCES attributes(id) ON DELETE CASCADE,
    CONSTRAINT UN_attribute_values_entity_type_entity_id_attribute_id UNIQUE (entity_type, entity_id, attribute_id)
);
