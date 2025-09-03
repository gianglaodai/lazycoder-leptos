CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE IF NOT EXISTS attributes (
    id SERIAL CONSTRAINT PK_attributes PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_attributes_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name VARCHAR(255) NOT NULL,
    entity_type VARCHAR(255) NOT NULL,
    data_type VARCHAR(255) NOT NULL,
    CONSTRAINT UN_attributes_entity_type_name UNIQUE (entity_type, name)
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

CREATE TABLE IF NOT EXISTS users (
    id SERIAL CONSTRAINT PK_users PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_users_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username CITEXT NOT NULL CONSTRAINT UN_users_username UNIQUE,
    email CITEXT NOT NULL CONSTRAINT UN_users_email UNIQUE,
    password VARCHAR(255) NOT NULL,
    role INTEGER NOT NULL DEFAULT 0
    );

CREATE TABLE IF NOT EXISTS post_types (
    id SERIAL CONSTRAINT PK_post_types PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_post_types_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    code TEXT NOT NULL CONSTRAINT UN_post_types_code UNIQUE,
    name TEXT NOT NULL,
    CONSTRAINT CK_post_types_code CHECK (code ~ '^[a-z0-9]+([a-z0-9_\-]*[a-z0-9])?$')
    );

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL CONSTRAINT PK_posts PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_posts_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    summary TEXT,
    content TEXT,
    status INTEGER NOT NULL DEFAULT 0,
    visibility TEXT NOT NULL DEFAULT 'public',
    format TEXT NOT NULL DEFAULT 'markdown',
    published_at TIMESTAMPTZ,
    user_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    CONSTRAINT UN_posts_type_slug UNIQUE (type_id, slug),
    CONSTRAINT CK_posts_status CHECK (status IN (0,1,2)),
    CONSTRAINT CK_posts_slug CHECK (slug ~ '^[a-z0-9]+([a-z0-9-]*[a-z0-9])?$'),
    CONSTRAINT CK_posts_visibility CHECK (visibility IN ('public','private','unlisted','members')),
    CONSTRAINT CK_posts_format CHECK (format IN ('markdown','html','mdx','plaintext')),
    CONSTRAINT FK_posts_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT FK_posts_type_id FOREIGN KEY (type_id) REFERENCES post_types(id) ON DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS IDX_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS IDX_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS IDX_posts_published_at ON posts(published_at DESC);
CREATE INDEX IF NOT EXISTS IDX_posts_user_id_status ON posts(user_id, status);
CREATE INDEX IF NOT EXISTS IDX_posts_type_id ON posts(type_id);
CREATE INDEX IF NOT EXISTS IDX_posts_type_slug ON posts(type_id, slug);
CREATE INDEX IF NOT EXISTS IDX_posts_published_visibility ON posts(published_at DESC, visibility);

CREATE TABLE IF NOT EXISTS post_collections (
    id SERIAL CONSTRAINT PK_posts_collections PRIMARY KEY,
    uid UUID NOT NULL CONSTRAINT UN_posts_collections_uid UNIQUE,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    slug TEXT NOT NULL CONSTRAINT UN_posts_collections_slug UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    visibility TEXT NOT NULL DEFAULT 'public',
    CONSTRAINT CK_post_collections_visibility CHECK (visibility IN ('public','private','unlisted')),
    CONSTRAINT CK_post_collections_slug CHECK (slug ~ '^[a-z0-9]+([a-z0-9-]*[a-z0-9])?$')
);

CREATE TABLE IF NOT EXISTS post_taxonomies (
                                               id SERIAL CONSTRAINT PK_post_taxonomies PRIMARY KEY,
                                               uid UUID NOT NULL CONSTRAINT UN_post_taxonomies_uid UNIQUE,
                                               version INTEGER NOT NULL DEFAULT 0,
                                               created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    code TEXT NOT NULL CONSTRAINT UN_post_taxonomies_code UNIQUE, /*'category', 'tag', 'topic', 'tech', 'tool'...*/
    name TEXT NOT NULL,
    CONSTRAINT CK_post_taxonomies_code CHECK (code ~ '^[a-z0-9]+([a-z0-9_\-]*[a-z0-9])?$')
    );

CREATE TABLE IF NOT EXISTS terms (
                                     id SERIAL CONSTRAINT PK_terms PRIMARY KEY,
                                     uid UUID NOT NULL CONSTRAINT UN_terms_uid UNIQUE,
                                     version INTEGER NOT NULL DEFAULT 0,
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    taxonomy_id INTEGER NOT NULL,
    parent_id INTEGER NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    CONSTRAINT UN_terms_taxonomy_id_slug UNIQUE (taxonomy_id, slug),
    CONSTRAINT CK_terms_slug CHECK (slug ~ '^[a-z0-9]+([a-z0-9-]*[a-z0-9])?$'),
    CONSTRAINT FK_terms_taxonomy_id FOREIGN KEY (taxonomy_id) REFERENCES post_taxonomies(id) ON DELETE CASCADE,
    CONSTRAINT FK_terms_parent_id FOREIGN KEY (parent_id) REFERENCES terms(id) ON DELETE SET NULL
    );

CREATE TABLE IF NOT EXISTS post_collection_items (
    post_collection_id INTEGER NOT NULL ,
    post_id INTEGER NOT NULL ,
    position INTEGER NOT NULL, -- 1,2,3...
    headline TEXT, -- optional custom title for TOC display
    CONSTRAINT PK_post_collection_items PRIMARY KEY (post_collection_id, post_id),
    CONSTRAINT UN_post_collection_items_position UNIQUE (post_collection_id, position),
    CONSTRAINT CK_post_collection_items_position CHECK (position > 0),
    CONSTRAINT FK_post_collection_items_post_collection_id FOREIGN KEY (post_collection_id) REFERENCES post_collections(id) ON DELETE CASCADE,
    CONSTRAINT FK_post_collection_items_post_id FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS IDX_post_collection_items_post_collection_id_position ON post_collection_items(post_collection_id, position);
CREATE INDEX IF NOT EXISTS IDX_terms_taxonomy_parent ON terms(taxonomy_id, parent_id);


CREATE TABLE IF NOT EXISTS post_terms (
    post_id INTEGER NOT NULL,
    term_id INTEGER NOT NULL,
    CONSTRAINT PK_post_terms PRIMARY KEY (post_id, term_id),
    CONSTRAINT FK_post_terms_post_id FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT FK_post_terms_term_id FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS IDX_post_terms_term_id ON post_terms(term_id);
CREATE INDEX IF NOT EXISTS IDX_post_terms_post_id ON post_terms(post_id);

CREATE TABLE IF NOT EXISTS post_relations (
    from_post INTEGER NOT NULL,
    to_post INTEGER NOT NULL,
    rel_type TEXT NOT NULL, /*'related', 'next', 'prev', 'see_also'*/
    CONSTRAINT PK_post_relations PRIMARY KEY (from_post, to_post, rel_type),
    CONSTRAINT CK_post_relations_type CHECK (rel_type IN ('related','next','prev','see_also')),
    CONSTRAINT FK_post_relations_from_post FOREIGN KEY (from_post) REFERENCES posts(id) ON DELETE CASCADE,
    CONSTRAINT FK_post_relations_to_post FOREIGN KEY (to_post) REFERENCES posts(id) ON DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS IDX_post_relations_from_post ON post_relations(from_post);
CREATE INDEX IF NOT EXISTS IDX_post_relations_to_post ON post_relations(to_post);