-- Recreate all _info views to provide convenient, join-enriched read models.
-- Keep definitions simple (no triggers/procs/functions) and only use SELECT/JOIN.

-- 1) users_info
DROP VIEW IF EXISTS users_info;
CREATE VIEW users_info AS
SELECT u.id,
       u.uid,
       u.version,
       u.created_at,
       u.updated_at,
       u.username,
       u.email,
       u.role
FROM users u;

-- 2) post_types_info
DROP VIEW IF EXISTS post_types_info;
CREATE VIEW post_types_info AS
SELECT pt.id,
       pt.uid,
       pt.version,
       pt.created_at,
       pt.updated_at,
       pt.code,
       pt.name
FROM post_types pt;

-- 3) posts_info
DROP VIEW IF EXISTS posts_info;
CREATE VIEW posts_info AS
SELECT p.id,
       p.uid,
       p.version,
       p.created_at,
       p.updated_at,
       p.slug,
       p.title,
       p.summary,
       p.content,
       p.status,
       p.user_id,
       u.username,
       u.email
FROM posts AS p
         LEFT JOIN users AS u ON p.user_id = u.id;

-- 4) post_collections_info
DROP VIEW IF EXISTS post_collections_info;
CREATE VIEW post_collections_info AS
SELECT pc.id,
       pc.uid,
       pc.version,
       pc.created_at,
       pc.updated_at,
       pc.slug,
       pc.title,
       pc.description,
       pc.visibility
FROM post_collections pc;


-- 6) post_taxonomies_info
DROP VIEW IF EXISTS post_taxonomies_info;
CREATE VIEW post_taxonomies_info AS
SELECT t.id,
       t.uid,
       t.version,
       t.created_at,
       t.updated_at,
       t.code,
       t.name
FROM post_taxonomies t;

-- 7) terms_info
DROP VIEW IF EXISTS terms_info;
CREATE VIEW terms_info AS
SELECT te.id,
       te.uid,
       te.version,
       te.created_at,
       te.updated_at,
       te.taxonomy_id,
       tx.code AS taxonomy_code,
       tx.name AS taxonomy_name,
       te.parent_id,
       tp.slug AS parent_slug,
       tp.name AS parent_name,
       te.slug,
       te.name,
       te.description
FROM terms te
         LEFT JOIN post_taxonomies tx ON tx.id = te.taxonomy_id
         LEFT JOIN terms tp ON tp.id = te.parent_id;



-- 10) attributes_info
DROP VIEW IF EXISTS attributes_info;
CREATE VIEW attributes_info AS
SELECT a.id,
       a.uid,
       a.version,
       a.created_at,
       a.updated_at,
       a.name,
       a.entity_type,
       a.data_type
FROM attributes a;

-- 11) attribute_values_info
DROP VIEW IF EXISTS attribute_values_info;
CREATE VIEW attribute_values_info AS
SELECT av.id,
       av.uid,
       av.version,
       av.created_at,
       av.updated_at,
       av.int_value,
       av.double_value,
       av.string_value,
       av.boolean_value,
       av.date_value,
       av.datetime_value,
       av.time_value,
       av.attribute_id,
       a.name        AS attribute_name,
       a.entity_type AS attribute_entity_type,
       a.data_type   AS attribute_data_type,
       av.entity_id,
       av.entity_type
FROM attribute_values av
         LEFT JOIN attributes a ON a.id = av.attribute_id;