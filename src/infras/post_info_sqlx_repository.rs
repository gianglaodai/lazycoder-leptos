#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_service::{PostInfo, PostInfoRepository, PostStatus};
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use sqlx::{postgres::PgRow, FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostInfoSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub user_id: i32,
    pub username: String,
    pub email: String,
});

impl From<PostInfoOrm> for PostInfo {
    fn from(orm: PostInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            slug: orm.slug,
            title: orm.title,
            summary: orm.summary,
            content: orm.content,
            status: PostStatus::from(orm.status),
            user_id: orm.user_id,
            username: orm.username,
            email: orm.email,
        }
    }
}

impl PostInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn table_name(&self) -> &str {
        "posts_info"
    }

    fn searchable_columns(&self) -> Vec<&str> {
        vec!["slug", "title", "summary", "content", "username", "email"]
    }

    fn build_find_many_query(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
        count: bool,
    ) -> QueryBuilder<'_, Postgres> {
        let mut query_builder = QueryBuilder::new(format!(
            "SELECT {} FROM {}",
            if count { "COUNT(*)" } else { "*" },
            self.table_name()
        ));

        use crate::business::filter::{Filter as F, FilterOperator, FilterValue};

        let (property_filters, attribute_filters, search_filters): (Vec<_>, Vec<_>, Vec<_>) =
            filters
                .into_iter()
                .fold((vec![], vec![], vec![]), |mut acc, f| {
                    match &f {
                        F::Property { .. } => acc.0.push(f),
                        F::Attribute { .. } => acc.1.push(f),
                        F::Search { .. } => acc.2.push(f),
                    }
                    acc
                });

        let mut has_where = false;

        for filter in property_filters {
            if !has_where {
                query_builder.push(" WHERE ");
                has_where = true;
            } else {
                query_builder.push(" AND ");
            }
            if let F::Property { property_name, operator, value } = filter {
                // reuse operator builder from sqlx_repository.rs by inlining minimal logic
                Self::push_property(&mut query_builder, &property_name, operator, value);
            }
        }

        for filter in attribute_filters {
            if !has_where {
                query_builder.push(" WHERE ");
                has_where = true;
            } else {
                query_builder.push(" AND ");
            }

            // For attributes, the entity_type is the base table name "posts"
            query_builder.push("EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = ");
            query_builder.push(self.table_name()).push(".id AND av.entity_type = ");
            query_builder.push_bind("posts");

            if let F::Attribute { attr_name, operator, value } = filter {
                Self::push_attribute(&mut query_builder, attr_name, operator, value);
            }

            query_builder.push(")");
        }

        for filter in search_filters {
            if let F::Search { value } = filter {
                let keyword = value.trim();
                if keyword.is_empty() {
                    continue;
                }
                if !has_where {
                    query_builder.push(" WHERE ");
                    has_where = true;
                } else {
                    query_builder.push(" AND ");
                }
                query_builder.push("(");

                // tsvector on searchable columns
                let searchable_columns = self.searchable_columns();
                query_builder.push("(to_tsvector('simple', unaccent(");
                for (i, col) in searchable_columns.iter().enumerate() {
                    if i > 0 { query_builder.push(" || ' ' || "); }
                    query_builder.push("coalesce(").push(col).push(", '')");
                }
                query_builder
                    .push(")) @@ to_tsquery('simple', unaccent(")
                    .push_bind(keyword.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>().join(" | "))
                    .push(")))");

                query_builder.push(" OR ");

                // attribute string search
                query_builder.push(" (EXISTS ( SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = ")
                    .push(self.table_name()).push(".id")
                    .push(" AND av.entity_type = ")
                    .push_bind("posts")
                    .push(" AND to_tsvector('simple', unaccent(coalesce(av.string_value, ''))) @@ to_tsquery('simple', unaccent(")
                    .push_bind(keyword.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>().join(" | "))
                    .push("))))");

                query_builder.push(")");
            }
        }

        if !sort_criteria.is_empty() {
            query_builder.push(" ORDER BY ");
            for (i, criterion) in sort_criteria.iter().enumerate() {
                if i > 0 { query_builder.push(", "); }
                query_builder.push(format!(
                    "{} {}",
                    criterion.field,
                    if criterion.ascending { "ASC" } else { "DESC" }
                ));
            }
        }

        if !count {
            query_builder.push(format!(" OFFSET {}", first_result.unwrap_or(0)));
            match max_results {
                Some(limit) => query_builder.push(format!(" LIMIT {}", limit)),
                None => query_builder.push(" LIMIT ALL"),
            };
        }

        query_builder
    }

    fn push_property(
        builder: &mut QueryBuilder<'_, Postgres>,
        field: &str,
        operator: crate::business::filter::FilterOperator,
        value: crate::business::filter::FilterValue,
    ) {
        use crate::business::filter::{FilterOperator as Op, FilterValue as Val};
        builder.push(format!("{} ", field));
        match operator {
            Op::Equal => {
                builder.push("= ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::String(v) => builder.push_bind(v),
                    Val::Bool(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("Equal not supported for this value type"),
                }
            }
            Op::NotEqual => {
                builder.push("!= ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::String(v) => builder.push_bind(v),
                    Val::Bool(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("NotEqual not supported for this value type"),
                }
            }
            Op::Like => match value {
                Val::String(v) => builder.push("LIKE ").push_bind(v),
                _ => unimplemented!("Like only supports string"),
            },
            Op::NotLike => match value {
                Val::String(v) => builder.push("NOT LIKE ").push_bind(v),
                _ => unimplemented!("NotLike only supports string"),
            },
            Op::GreaterThan => {
                builder.push("> ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("GreaterThan not supported for this value type"),
                }
            }
            Op::GreaterThanOrEqual => {
                builder.push(">= ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("GreaterThanOrEqual not supported for this value type"),
                }
            }
            Op::LessThan => {
                builder.push("< ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("LessThan not supported for this value type"),
                }
            }
            Op::LessThanOrEqual => {
                builder.push("<= ");
                match value {
                    Val::Int(v) => builder.push_bind(v),
                    Val::Float(v) => builder.push_bind(v),
                    Val::Date(v) => builder.push_bind(v),
                    Val::DateTime(v) => builder.push_bind(v),
                    Val::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("LessThanOrEqual not supported for this value type"),
                }
            }
            Op::Is => match value {
                Val::Bool(v) => builder.push("= ").push_bind(v),
                _ => unimplemented!("Is not supported for this value type"),
            },
            Op::In => match value {
                Val::ListInt(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                Val::ListFloat(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                Val::ListString(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                _ => unimplemented!("In only supports list values"),
            },
            Op::NotIn => match value {
                Val::ListInt(vs) => builder.push("NOT IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                Val::ListFloat(vs) => builder.push("NOT IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                Val::ListString(vs) => builder.push("NOT IN ").push_tuples(vs, |mut b, v| { b.push_bind(v); }),
                _ => unimplemented!("NotIn only supports list values"),
            },
            Op::IsNull => builder.push("IS NULL"),
            Op::NotNull => builder.push("IS NOT NULL"),
            Op::Between => match value {
                Val::IntRange(from, to) => { builder.push("BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::FloatRange(from, to) => { builder.push("BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::DateRange(from, to) => { builder.push("BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::DateTimeRange(from, to) => { builder.push("BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::TimeRange(from, to) => { builder.push("BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                _ => unimplemented!("Between only supports range types"),
            },
            Op::NotBetween => match value {
                Val::IntRange(from, to) => { builder.push("NOT BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::FloatRange(from, to) => { builder.push("NOT BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::DateRange(from, to) => { builder.push("NOT BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::DateTimeRange(from, to) => { builder.push("NOT BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                Val::TimeRange(from, to) => { builder.push("NOT BETWEEN ").push_bind(from).push(" AND ").push_bind(to); }
                _ => unimplemented!("NotBetween only supports range types"),
            },
        }
    }

    fn push_attribute(
        builder: &mut QueryBuilder<'_, Postgres>,
        attr_name: String,
        operator: crate::business::filter::FilterOperator,
        value: crate::business::filter::FilterValue,
    ) {
        use crate::business::filter::{FilterOperator as Op, FilterValue as Val};
        builder.push(" AND a.name = ");
        builder.push_bind(attr_name);

        let value_column = match value {
            Val::Int(_) | Val::IntRange(_, _) | Val::ListInt(_) => "int_value",
            Val::Float(_) | Val::FloatRange(_, _) | Val::ListFloat(_) => "double_value",
            Val::String(_) | Val::ListString(_) => "string_value",
            Val::Bool(_) => "boolean_value",
            Val::Date(_) | Val::DateRange(_, _) => "date_value",
            Val::DateTime(_) | Val::DateTimeRange(_, _) => "datetime_value",
            Val::Time(_) | Val::TimeRange(_, _) => "time_value",
        };

        builder.push(format!(" AND av.{} ", value_column));
        Self::push_property(builder, "", operator, value); // operator and binding only; field already placed
    }
}

impl ViewRepository<PostInfo> for PostInfoSqlxRepository {
    fn find_all(&self, filters: Vec<Filter>) -> impl std::future::Future<Output = Result<Vec<PostInfo>, CoreError>> {
        self.find_many(vec![], None, None, filters)
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        let mut query_builder = self.build_find_many_query(vec![], None, None, filters, true);
        let result: i64 = query_builder
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await?;
        Ok(result)
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostInfo>, CoreError> {
        let mut query_builder =
            self.build_find_many_query(sort_criteria, first_result, max_results, filters, false);
        let result = query_builder
            .build_query_as::<PostInfoOrm>()
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(PostInfo::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PostInfo>, CoreError> {
        let result = sqlx::query_as::<_, PostInfoOrm>(&format!(
            "SELECT * FROM {} WHERE id=$1",
            self.table_name()
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.map(PostInfo::from))
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<PostInfo>, CoreError> {
        let result = sqlx::query_as::<_, PostInfoOrm>(&format!(
            "SELECT * FROM {} WHERE uid=$1",
            self.table_name()
        ))
        .bind(Uuid::parse_str(&uid).unwrap())
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.map(PostInfo::from))
    }
}

impl PostInfoRepository for PostInfoSqlxRepository {}
