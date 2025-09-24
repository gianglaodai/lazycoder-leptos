#![cfg(feature = "ssr")]

use crate::common::error::CoreError;
use crate::common::filter::{Filter, FilterOperator, FilterValue, ScalarValue};
use crate::common::repository::{Creatable, Repository, ViewRepository};
use crate::common::service::Entity as BizEntity;
use crate::common::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attribute_datatype", rename_all = "lowercase")]
pub enum AttributeDataType {
    Int,
    Float,
    String,
    Boolean,
    Date,
    DateTime,
    Time,
}

define_orm_with_common_fields!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

define_orm_with_common_fields!(AttributeValue {
    pub attribute_id: i32,
    pub entity_id: i32,
    pub entity_type: String,
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
});

pub trait OrmMeta {
    fn columns() -> Vec<&'static str>;
    fn auto_columns() -> Vec<&'static str> {
        vec!["id"]
    }
    fn insertable_columns() -> Vec<&'static str> {
        let auto = Self::auto_columns();
        Self::columns()
            .into_iter()
            .filter(|c| !auto.contains(c))
            .collect()
    }
    fn updatable_columns() -> Vec<&'static str> {
        let exclude = vec!["id", "uid", "created_at"];
        Self::columns()
            .into_iter()
            .filter(|c| !exclude.contains(c))
            .collect()
    }
}

pub trait OrmBind {
    fn bind_column<'q>(&'q self, col: &str, qb: &mut QueryBuilder<'q, Postgres>);
    fn bind_update_pairs<'q>(&'q self, cols: &[&str], qb: &mut QueryBuilder<'q, Postgres>);
}

pub trait SqlxEntityMapper {
    type Entity;
    type EntityCreate;
    type Orm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm;
    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm;
}

pub trait SqlxViewMeta {
    fn get_table_name(&self) -> &str;
    fn get_columns(&self) -> Vec<&str>;
    fn get_searchable_columns(&self) -> Vec<&str>;
}

pub trait SqlxViewRepository: SqlxViewMeta {
    type Entity;
    type Orm: for<'r> FromRow<'r, PgRow> + Send + Unpin;

    fn get_pool(&self) -> &PgPool;

    /// List of columns that are stored as textual types (e.g., text/varchar)
    /// This is used to avoid type-mismatch when a numeric-looking literal is provided for a string column.
    /// Default: empty list.
    fn get_string_properties(&self) -> Vec<&str> {
        vec![]
    }

    async fn get_column_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        // Repository: fetch directly from DB; caching is handled at the service layer
        let table = SqlxViewMeta::get_table_name(self).to_string();
        let pool = self.get_pool();
        let cols = SqlxViewMeta::get_columns(self);

        let rows = sqlx::query(
            "SELECT column_name, data_type, udt_name FROM information_schema.columns WHERE table_schema = 'public' AND table_name = $1"
        )
        .bind(&table)
        .fetch_all(pool)
        .await?;

        let wanted_set: std::collections::HashSet<&str> = cols.into_iter().collect();

        let mut map: HashMap<String, ScalarValue> = HashMap::new();
        for row in rows.into_iter() {
            let col: String = row.get("column_name");
            if !wanted_set.is_empty() && !wanted_set.contains(col.as_str()) {
                continue;
            }
            let data_type: String = row.get("data_type");
            let udt_name: String = row.get("udt_name");
            let scalar = match data_type.as_str() {
                "integer" | "smallint" | "bigint" => ScalarValue::Int(0),
                "double precision" | "real" | "numeric" => ScalarValue::Float(0.0),
                "boolean" => ScalarValue::Bool(false),
                "date" => ScalarValue::Date(
                    time::Date::from_calendar_date(1970, time::Month::January, 1).unwrap(),
                ),
                "timestamp with time zone" | "timestamp without time zone" => {
                    ScalarValue::DateTime(time::OffsetDateTime::UNIX_EPOCH)
                }
                "time without time zone" | "time with time zone" => {
                    ScalarValue::Time(time::Time::from_hms(0, 0, 0).unwrap())
                }
                "character varying" | "character" | "text" => ScalarValue::String(String::new()),
                _ => match udt_name.as_str() {
                    "uuid" => ScalarValue::String(String::new()),
                    _ => ScalarValue::String(String::new()),
                },
            };
            map.insert(col, scalar);
        }
        Ok(map)
    }

    fn from_orm(orm: Self::Orm) -> Self::Entity;
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Self::Entity>, CoreError> {
        let mut query_builder =
            self.build_find_many_query(sort_criteria, first_result, max_results, filters, false);
        log::info!("{}", query_builder.sql());
        let result = query_builder
            .build_query_as::<Self::Orm>()
            .fetch_all(self.get_pool())
            .await?;

        Ok(result.into_iter().map(|orm| Self::from_orm(orm)).collect())
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        let mut query_builder = self.build_find_many_query(vec![], None, None, filters, true);
        let result = query_builder
            .build_query_scalar()
            .fetch_one(self.get_pool())
            .await?;
        Ok(result)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Self::Entity>, CoreError> {
        let result = sqlx::query_as::<_, Self::Orm>(&format!(
            "SELECT * FROM {} WHERE id=$1",
            SqlxViewMeta::get_table_name(self)
        ))
        .bind(id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(result.map(|orm| Self::from_orm(orm)))
    }

    async fn find_by_uid(&self, uid: Uuid) -> Result<Option<Self::Entity>, CoreError> {
        let result = sqlx::query_as::<_, Self::Orm>(&format!(
            "SELECT * FROM {} WHERE uid=$1",
            SqlxViewMeta::get_table_name(self)
        ))
        .bind(uid)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(result.map(|orm| Self::from_orm(orm)))
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
            SqlxViewMeta::get_table_name(self)
        ));

        let (property_filters, attribute_filters, search_filters): (Vec<_>, Vec<_>, Vec<_>) =
            filters
                .into_iter()
                .fold((vec![], vec![], vec![]), |mut acc, f| {
                    match &f {
                        Filter::Property { .. } => acc.0.push(f),
                        Filter::Attribute { .. } => acc.1.push(f),
                        Filter::Search { .. } => acc.2.push(f),
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

            if let Filter::Property {
                property_name,
                operator,
                value,
            } = filter
            {
                self.build_property_filter(&mut query_builder, &property_name, operator, value);
            }
        }

        for filter in attribute_filters {
            if !has_where {
                query_builder.push(" WHERE ");
                has_where = true;
            } else {
                query_builder.push(" AND ");
            }

            query_builder.push(format!(
                "EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = {}.id AND av.entity_type = ",
                SqlxViewMeta::get_table_name(self)
            ));
            query_builder.push_bind(SqlxViewMeta::get_table_name(self));

            if let Filter::Attribute {
                attr_name,
                operator,
                value,
            } = filter
            {
                self.build_attribute_filter(&mut query_builder, attr_name, operator, value);
            }

            query_builder.push(")");
        }

        for filter in search_filters {
            if let Filter::Search { value } = filter {
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

                let searchable_columns = SqlxViewMeta::get_searchable_columns(self);
                query_builder.push("(to_tsvector('simple', unaccent(");
                for (i, col) in searchable_columns.iter().enumerate() {
                    if i > 0 {
                        query_builder.push(" || ' ' || ");
                    }
                    query_builder.push("coalesce(").push(col).push(", '')");
                }
                query_builder
                    .push(")) @@ plainto_tsquery('simple', unaccent(")
                    .push_bind(
                        keyword
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(" | "),
                    )
                    .push(")))");

                query_builder.push(" OR ");

                {
                    query_builder.push(" (EXISTS ( SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = ")
                        .push(SqlxViewMeta::get_table_name(self)).push(".id")
                        .push(" AND av.entity_type = ")
                        .push_bind(SqlxViewMeta::get_table_name(self))
                        .push(" AND to_tsvector('simple', unaccent(coalesce(av.string_value, ''))) @@ plainto_tsquery('simple', unaccent(")
                        .push_bind(keyword.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>().join(" | "))
                        .push("))))");
                }

                query_builder.push(")");
            }
        }

        if !sort_criteria.is_empty() {
            query_builder.push(" ORDER BY ");
            for (i, criterion) in sort_criteria.iter().enumerate() {
                if i > 0 {
                    query_builder.push(", ");
                }
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
    fn build_property_filter(
        &self,
        builder: &mut QueryBuilder<'_, sqlx::Postgres>,
        field: &str,
        operator: FilterOperator,
        value: FilterValue,
    ) {
        let is_string_col = self.get_string_properties().contains(&field);
        if is_string_col {
            // Compare on the left side as text to avoid type mismatch
            builder.push(format!("{}::text ", field));
            // Coerce value to string-based representations
            let coerced = match value {
                FilterValue::Single(s) => FilterValue::Single(ScalarValue::String(match s {
                    ScalarValue::String(v) => v,
                    ScalarValue::Int(v) => v.to_string(),
                    ScalarValue::Float(v) => v.to_string(),
                    ScalarValue::Bool(v) => v.to_string(),
                    ScalarValue::Date(v) => v.to_string(),
                    ScalarValue::DateTime(v) => v.to_string(),
                    ScalarValue::Time(v) => v.to_string(),
                })),
                FilterValue::List(vs) => FilterValue::List(
                    vs.into_iter()
                        .map(|s| {
                            ScalarValue::String(match s {
                                ScalarValue::String(v) => v,
                                ScalarValue::Int(v) => v.to_string(),
                                ScalarValue::Float(v) => v.to_string(),
                                ScalarValue::Bool(v) => v.to_string(),
                                ScalarValue::Date(v) => v.to_string(),
                                ScalarValue::DateTime(v) => v.to_string(),
                                ScalarValue::Time(v) => v.to_string(),
                            })
                        })
                        .collect(),
                ),
                FilterValue::Range((a, b)) => FilterValue::Range((
                    ScalarValue::String(match a {
                        ScalarValue::String(v) => v,
                        ScalarValue::Int(v) => v.to_string(),
                        ScalarValue::Float(v) => v.to_string(),
                        ScalarValue::Bool(v) => v.to_string(),
                        ScalarValue::Date(v) => v.to_string(),
                        ScalarValue::DateTime(v) => v.to_string(),
                        ScalarValue::Time(v) => v.to_string(),
                    }),
                    ScalarValue::String(match b {
                        ScalarValue::String(v) => v,
                        ScalarValue::Int(v) => v.to_string(),
                        ScalarValue::Float(v) => v.to_string(),
                        ScalarValue::Bool(v) => v.to_string(),
                        ScalarValue::Date(v) => v.to_string(),
                        ScalarValue::DateTime(v) => v.to_string(),
                        ScalarValue::Time(v) => v.to_string(),
                    }),
                )),
                FilterValue::None => FilterValue::None,
            };
            Self::handle_operator(builder, operator, coerced);
        } else {
            // Default behavior for non-text columns
            let use_trunc = matches!(operator, FilterOperator::Equal | FilterOperator::NotEqual)
                && matches!(value, FilterValue::Single(ScalarValue::DateTime(_)));
            if use_trunc {
                builder.push(format!("date_trunc('second', {}::timestamptz) ", field));
            } else {
                builder.push(format!("{} ", field));
            }
            Self::handle_operator(builder, operator, value);
        }
    }

    fn build_attribute_filter(
        &self,
        builder: &mut QueryBuilder<'_, sqlx::Postgres>,
        attr_name: String,
        operator: FilterOperator,
        value: FilterValue,
    ) {
        builder.push(" AND a.name = ");
        builder.push_bind(attr_name);

        let value_column = match &value {
            FilterValue::Single(s) => match s {
                ScalarValue::Int(_) => "int_value",
                ScalarValue::Float(_) => "double_value",
                ScalarValue::String(_) => "string_value",
                ScalarValue::Bool(_) => "boolean_value",
                ScalarValue::Date(_) => "date_value",
                ScalarValue::DateTime(_) => "datetime_value",
                ScalarValue::Time(_) => "time_value",
            },
            FilterValue::List(vs) => {
                if let Some(first) = vs.first() {
                    match first {
                        ScalarValue::Int(_) => "int_value",
                        ScalarValue::Float(_) => "double_value",
                        ScalarValue::String(_) => "string_value",
                        // lists of bool/date/time aren't supported by parser, but default safely
                        ScalarValue::Bool(_) => "boolean_value",
                        ScalarValue::Date(_) => "date_value",
                        ScalarValue::DateTime(_) => "datetime_value",
                        ScalarValue::Time(_) => "time_value",
                    }
                } else {
                    "string_value"
                }
            }
            FilterValue::Range((a, _)) => match a {
                ScalarValue::Int(_) => "int_value",
                ScalarValue::Float(_) => "double_value",
                ScalarValue::String(_) => "string_value",
                ScalarValue::Bool(_) => "boolean_value",
                ScalarValue::Date(_) => "date_value",
                ScalarValue::DateTime(_) => "datetime_value",
                ScalarValue::Time(_) => "time_value",
            },
            FilterValue::None => "string_value",
        };

        // If comparing datetime equality on attribute value, align to seconds on the column side
        let use_trunc_attr = matches!(operator, FilterOperator::Equal | FilterOperator::NotEqual)
            && matches!(value, FilterValue::Single(ScalarValue::DateTime(_)));
        if use_trunc_attr && value_column == "datetime_value" {
            builder.push(" AND date_trunc('second', av.datetime_value::timestamptz) ");
        } else {
            builder.push(format!(" AND av.{} ", value_column));
        }
        Self::handle_operator(builder, operator, value);
    }

    fn handle_operator(
        builder: &mut QueryBuilder<Postgres>,
        operator: FilterOperator,
        value: FilterValue,
    ) {
        match operator {
            FilterOperator::Equal => {
                builder.push("= ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::String(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Bool(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push("date_trunc('second', ");
                                builder.push_bind(v);
                                builder.push("::timestamptz)");
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                        };
                    }
                    _ => unimplemented!("Equal expects a single scalar value"),
                }
            }

            FilterOperator::NotEqual => {
                builder.push("!= ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::String(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Bool(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push("date_trunc('second', ");
                                builder.push_bind(v);
                                builder.push("::timestamptz)");
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                        };
                    }
                    _ => unimplemented!("NotEqual expects a single scalar value"),
                }
            }

            FilterOperator::Like => match value {
                FilterValue::Single(ScalarValue::String(v)) => {
                    builder.push("LIKE ");
                    builder.push_bind(format!("%{}%", v));
                }
                _ => unimplemented!("Like only supports string single value"),
            },

            FilterOperator::NotLike => match value {
                FilterValue::Single(ScalarValue::String(v)) => {
                    builder.push("NOT LIKE ");
                    builder.push_bind(format!("%{}%", v));
                }
                _ => unimplemented!("NotLike only supports string single value"),
            },

            FilterOperator::GreaterThan => {
                builder.push("> ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                            _ => unimplemented!("GreaterThan unsupported scalar"),
                        };
                    }
                    _ => unimplemented!("GreaterThan expects a single scalar"),
                }
            }

            FilterOperator::GreaterThanOrEqual => {
                builder.push(">= ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                            _ => unimplemented!("GreaterThanOrEqual unsupported scalar"),
                        };
                    }
                    _ => unimplemented!("GreaterThanOrEqual expects a single scalar"),
                }
            }

            FilterOperator::LessThan => {
                builder.push("< ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                            _ => unimplemented!("LessThan unsupported scalar"),
                        };
                    }
                    _ => unimplemented!("LessThan expects a single scalar"),
                }
            }

            FilterOperator::LessThanOrEqual => {
                builder.push("<= ");
                match value {
                    FilterValue::Single(s) => {
                        match s {
                            ScalarValue::Int(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Float(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Date(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::DateTime(v) => {
                                builder.push_bind(v);
                            }
                            ScalarValue::Time(v) => {
                                builder.push_bind(v);
                            }
                            _ => unimplemented!("LessThanOrEqual unsupported scalar"),
                        };
                    }
                    _ => unimplemented!("LessThanOrEqual expects a single scalar"),
                }
            }

            FilterOperator::In => match value {
                FilterValue::List(vs) => {
                    builder.push("IN ");
                    builder.push_tuples(vs, |mut b, v| {
                        match v {
                            ScalarValue::Int(i) => b.push_bind(i),
                            ScalarValue::Float(f) => b.push_bind(f),
                            ScalarValue::String(s) => b.push_bind(s),
                            _ => unimplemented!("Unsupported element type in IN list"),
                        };
                    });
                }
                _ => unimplemented!("In expects a list"),
            },

            FilterOperator::NotIn => match value {
                FilterValue::List(vs) => {
                    builder.push("NOT IN ");
                    builder.push_tuples(vs, |mut b, v| {
                        match v {
                            ScalarValue::Int(i) => b.push_bind(i),
                            ScalarValue::Float(f) => b.push_bind(f),
                            ScalarValue::String(s) => b.push_bind(s),
                            _ => unimplemented!("Unsupported element type in NOT IN list"),
                        };
                    });
                }
                _ => unimplemented!("NotIn expects a list"),
            },

            FilterOperator::IsNull => {
                builder.push("IS NULL");
            }

            FilterOperator::NotNull => {
                builder.push("IS NOT NULL");
            }

            FilterOperator::Between => {
                builder.push("BETWEEN ");
                match value {
                    FilterValue::Range((a, b)) => {
                        match a {
                            // types are guaranteed comparable and same kind by construction
                            ScalarValue::Int(v1) => builder.push_bind(v1),
                            ScalarValue::Float(v1) => builder.push_bind(v1),
                            ScalarValue::Date(v1) => builder.push_bind(v1),
                            ScalarValue::DateTime(v1) => builder.push_bind(v1),
                            ScalarValue::Time(v1) => builder.push_bind(v1),
                            _ => unimplemented!("Unsupported start type for BETWEEN"),
                        };
                        builder.push(" AND ");
                        match b {
                            ScalarValue::Int(v2) => builder.push_bind(v2),
                            ScalarValue::Float(v2) => builder.push_bind(v2),
                            ScalarValue::Date(v2) => builder.push_bind(v2),
                            ScalarValue::DateTime(v2) => builder.push_bind(v2),
                            ScalarValue::Time(v2) => builder.push_bind(v2),
                            _ => unimplemented!("Unsupported end type for BETWEEN"),
                        };
                    }
                    _ => unimplemented!("Between expects a range"),
                }
            }

            FilterOperator::NotBetween => {
                builder.push("NOT BETWEEN ");
                match value {
                    FilterValue::Range((a, b)) => {
                        match a {
                            ScalarValue::Int(v1) => builder.push_bind(v1),
                            ScalarValue::Float(v1) => builder.push_bind(v1),
                            ScalarValue::Date(v1) => builder.push_bind(v1),
                            ScalarValue::DateTime(v1) => builder.push_bind(v1),
                            ScalarValue::Time(v1) => builder.push_bind(v1),
                            _ => unimplemented!("Unsupported start type for NOT BETWEEN"),
                        };
                        builder.push(" AND ");
                        match b {
                            ScalarValue::Int(v2) => builder.push_bind(v2),
                            ScalarValue::Float(v2) => builder.push_bind(v2),
                            ScalarValue::Date(v2) => builder.push_bind(v2),
                            ScalarValue::DateTime(v2) => builder.push_bind(v2),
                            ScalarValue::Time(v2) => builder.push_bind(v2),
                            _ => unimplemented!("Unsupported end type for NOT BETWEEN"),
                        };
                    }
                    _ => unimplemented!("NotBetween expects a range"),
                }
            }
        };
    }
}
pub trait SqlxRepository: SqlxViewRepository
where
    Self::EntityCreate: Creatable<Entity = Self::Entity>,
{
    type EntityCreate;

    // Default create/update delegate to generic_* when mapper and meta are available.
    async fn create(
        &self,
        entity_create: &<Self as SqlxRepository>::EntityCreate,
    ) -> Result<<Self as SqlxViewRepository>::Entity, CoreError>
    where
        Self: SqlxEntityMapper<
            Entity = <Self as SqlxViewRepository>::Entity,
            EntityCreate = <Self as SqlxRepository>::EntityCreate,
            Orm = <Self as SqlxViewRepository>::Orm,
        >,
        <Self as SqlxViewRepository>::Orm: OrmMeta + OrmBind,
    {
        self.generic_create(entity_create).await
    }
    async fn update(
        &self,
        entity: &<Self as SqlxViewRepository>::Entity,
    ) -> Result<<Self as SqlxViewRepository>::Entity, CoreError>
    where
        Self: SqlxEntityMapper<
            Entity = <Self as SqlxViewRepository>::Entity,
            EntityCreate = <Self as SqlxRepository>::EntityCreate,
            Orm = <Self as SqlxViewRepository>::Orm,
        >,
        <Self as SqlxViewRepository>::Orm: OrmMeta + OrmBind,
        <Self as SqlxViewRepository>::Entity: BizEntity,
    {
        self.generic_update(entity).await
    }

    // Generic helpers using OrmMeta + OrmBind + SqlxEntityMapper. Repos can call these.
    async fn generic_create(
        &self,
        entity_create: &<Self as SqlxRepository>::EntityCreate,
    ) -> Result<<Self as SqlxViewRepository>::Entity, CoreError>
    where
        Self: SqlxEntityMapper<
            Entity = <Self as SqlxViewRepository>::Entity,
            EntityCreate = <Self as SqlxRepository>::EntityCreate,
            Orm = <Self as SqlxViewRepository>::Orm,
        >,
        <Self as SqlxViewRepository>::Orm: OrmMeta + OrmBind,
    {
        let orm = <Self as SqlxEntityMapper>::to_orm_from_create(self, entity_create);
        let table = SqlxViewMeta::get_table_name(self);
        let insert_cols = <<Self as SqlxViewRepository>::Orm as OrmMeta>::insertable_columns();

        let mut qb = QueryBuilder::<Postgres>::new("INSERT INTO ");
        qb.push(table).push(" (");
        for (i, col) in insert_cols.iter().enumerate() {
            if i > 0 {
                qb.push(", ");
            }
            qb.push(*col);
        }
        qb.push(") VALUES (");
        for (i, col) in insert_cols.iter().enumerate() {
            if i > 0 {
                qb.push(", ");
            }
            orm.bind_column(col, &mut qb);
        }
        qb.push(") RETURNING *");

        let orm_row: <Self as SqlxViewRepository>::Orm =
            qb.build_query_as().fetch_one(self.get_pool()).await?;
        Ok(<Self as SqlxViewRepository>::from_orm(orm_row))
    }

    async fn generic_update(
        &self,
        entity: &<Self as SqlxViewRepository>::Entity,
    ) -> Result<<Self as SqlxViewRepository>::Entity, CoreError>
    where
        Self: SqlxEntityMapper<
            Entity = <Self as SqlxViewRepository>::Entity,
            EntityCreate = <Self as SqlxRepository>::EntityCreate,
            Orm = <Self as SqlxViewRepository>::Orm,
        >,
        <Self as SqlxViewRepository>::Orm: OrmMeta + OrmBind,
        <Self as SqlxViewRepository>::Entity: BizEntity,
    {
        let orm = <Self as SqlxEntityMapper>::to_orm_from_entity(self, entity);
        let table = SqlxViewMeta::get_table_name(self);
        let set_cols = <<Self as SqlxViewRepository>::Orm as OrmMeta>::updatable_columns();

        let mut qb = QueryBuilder::<Postgres>::new("UPDATE ");
        qb.push(table).push(" SET ");
        orm.bind_update_pairs(&set_cols, &mut qb);
        qb.push(" WHERE id = ");
        qb.push_bind(entity.id());
        qb.push(" RETURNING *");

        let orm_row: <Self as SqlxViewRepository>::Orm =
            qb.build_query_as().fetch_one(self.get_pool()).await?;
        Ok(<Self as SqlxViewRepository>::from_orm(orm_row))
    }

    // Provided defaults: common attribute type map and delete helpers
    async fn get_attribute_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        // Repository: fetch directly from DB; caching is handled at the service layer
        let entity_type = SqlxViewMeta::get_table_name(self).to_string();
        let pool = self.get_pool();

        // Select attribute name and textual data type for the owner entity_type
        let rows = sqlx::query(
            "SELECT name, data_type::text AS data_type FROM attributes WHERE entity_type = $1",
        )
        .bind(&entity_type)
        .fetch_all(pool)
        .await?;

        let mut map: HashMap<String, ScalarValue> = HashMap::new();
        for row in rows.into_iter() {
            let name: String = row.get("name");
            let dt: String = row.get("data_type");
            if map.contains_key(&name) {
                return Err(CoreError::UnprocessableEntity(
                    "error.attribute.duplicate.name".into(),
                    std::collections::HashMap::from([
                        ("name".into(), name),
                        ("entity_type".into(), entity_type.clone()),
                    ]),
                ));
            }
            let scalar = match dt.as_str() {
                "int" | "integer" => ScalarValue::Int(0),
                "double" | "float" | "double precision" | "real" | "numeric" => {
                    ScalarValue::Float(0.0)
                }
                "string" | "text" | "character varying" | "character" => {
                    ScalarValue::String(String::new())
                }
                "boolean" | "bool" => ScalarValue::Bool(false),
                "date" => ScalarValue::Date(
                    time::Date::from_calendar_date(1970, time::Month::January, 1).unwrap(),
                ),
                "datetime"
                | "timestamp"
                | "timestamp with time zone"
                | "timestamp without time zone" => {
                    ScalarValue::DateTime(time::OffsetDateTime::UNIX_EPOCH)
                }
                "time" => ScalarValue::Time(time::Time::from_hms(0, 0, 0).unwrap()),
                other => {
                    return Err(CoreError::UnprocessableEntity(
                        "error.attribute.unsupported.datatype".into(),
                        std::collections::HashMap::from([
                            ("datatype".into(), other.to_string()),
                            ("name".into(), name),
                        ]),
                    ));
                }
            };
            map.insert(name, scalar);
        }
        Ok(map)
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        let result = sqlx::query(&format!(
            "DELETE FROM {} WHERE id = $1",
            SqlxViewMeta::get_table_name(self)
        ))
        .bind(id)
        .execute(self.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut builder = QueryBuilder::<Postgres>::new(format!(
            "DELETE FROM {} WHERE id IN ",
            SqlxViewMeta::get_table_name(self)
        ));

        // Follow the same convention used in filter IN/NOT IN: use tuples for values
        builder.push_tuples(ids, |mut b, id| {
            b.push_bind(id);
        });

        let query = builder.build();
        let result = query.execute(self.get_pool()).await?;
        Ok(result.rows_affected())
    }

    async fn delete_by_uid(&self, uid: Uuid) -> Result<u64, CoreError> {
        let result = sqlx::query(&format!(
            "DELETE FROM {} WHERE uid = $1",
            SqlxViewMeta::get_table_name(self)
        ))
        .bind(uid)
        .execute(self.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    async fn delete_by_uids(&self, uids: Vec<Uuid>) -> Result<u64, CoreError> {
        if uids.is_empty() {
            return Ok(0);
        }

        let mut builder = QueryBuilder::<Postgres>::new(format!(
            "DELETE FROM {} WHERE uid IN ",
            SqlxViewMeta::get_table_name(self)
        ));
        builder.push_tuples(uids, |mut b, id| {
            b.push_bind(id);
        });
        let query = builder.build();
        let result = query.execute(self.get_pool()).await?;
        Ok(result.rows_affected())
    }
}

// Blanket implementation: any SqlxViewRepository automatically implements ViewRepository for its Entity
impl<R> ViewRepository<<R as SqlxViewRepository>::Entity> for R
where
    R: SqlxViewRepository + Send + Sync,
{
    fn get_table_name(&self) -> &str {
        SqlxViewMeta::get_table_name(self)
    }
    fn get_columns(&self) -> Vec<&str> {
        SqlxViewMeta::get_columns(self)
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        SqlxViewMeta::get_searchable_columns(self)
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<<R as SqlxViewRepository>::Entity>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<Option<<R as SqlxViewRepository>::Entity>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(
        &self,
        uid: String,
    ) -> Result<Option<<R as SqlxViewRepository>::Entity>, CoreError> {
        let uuid = match Uuid::parse_str(&uid) {
            Ok(u) => u,
            Err(_) => return Err(CoreError::bad_request("error.invalid.uid")),
        };
        SqlxViewRepository::find_by_uid(self, uuid).await
    }

    async fn get_column_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

// Blanket implementation: any SqlxRepository automatically implements Repository for its Entity/Create
impl<R> Repository<<R as SqlxViewRepository>::Entity, <R as SqlxRepository>::EntityCreate> for R
where
    R: SqlxRepository
        + Send
        + Sync
        + SqlxEntityMapper<
            Entity = <R as SqlxViewRepository>::Entity,
            EntityCreate = <R as SqlxRepository>::EntityCreate,
            Orm = <R as SqlxViewRepository>::Orm,
        >,
    <R as SqlxViewRepository>::Orm: OrmMeta + OrmBind,
    <R as SqlxViewRepository>::Entity: BizEntity,
{
    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids).await
    }

    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        let uuid = match Uuid::parse_str(&uid) {
            Ok(u) => u,
            Err(_) => return Err(CoreError::bad_request("error.invalid.uid")),
        };
        SqlxRepository::delete_by_uid(self, uuid).await
    }

    async fn delete_by_uids(&self, uids: Vec<String>) -> Result<u64, CoreError> {
        if uids.is_empty() {
            return Ok(0);
        }
        // Try parsing all; fail fast on first invalid
        let mut parsed = Vec::with_capacity(uids.len());
        for s in uids.into_iter() {
            match Uuid::parse_str(&s) {
                Ok(u) => parsed.push(u),
                Err(_) => return Err(CoreError::bad_request("error.invalid.uid")),
            }
        }
        SqlxRepository::delete_by_uids(self, parsed).await
    }

    async fn create(
        &self,
        entity_create: &<R as SqlxRepository>::EntityCreate,
    ) -> Result<<R as SqlxViewRepository>::Entity, CoreError> {
        <Self as SqlxRepository>::create(self, entity_create).await
    }

    async fn update(
        &self,
        entity: &<R as SqlxViewRepository>::Entity,
    ) -> Result<<R as SqlxViewRepository>::Entity, CoreError> {
        <Self as SqlxRepository>::update(self, entity).await
    }

    async fn get_attribute_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxRepository::get_attribute_type_map(self).await
    }
}
