DROP VIEW IF EXISTS posts_info;
CREATE VIEW posts_info AS
SELECT p.id           AS id,
       p.uid          AS uid,
       p.version      AS version,
       p.created_at   AS created_at,
       p.updated_at   AS updated_at,
       p.slug         AS slug,
       p.title        AS title,
       p.summary      AS summary,
       p.content      AS content,
       p.status       AS status,
       p.published_at AS published_at,
       p.user_id      AS user_id,
       u.username     AS username,
       u.email        AS email
FROM posts AS p
         LEFT JOIN users AS u ON p.user_id = u.id;