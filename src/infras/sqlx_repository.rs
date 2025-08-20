#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::{Filter, FilterOperator, FilterValue};
use crate::business::repository::{Creatable, Repository, ViewRepository};
use crate::business::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attribute_datatype", rename_all = "lowercase")]
pub enum AttributeDataType {
    Int,
    Double,
    String,
    Boolean,
    Date,
    DateTime,
    Time,
}

define_orm_with_common_fields!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: AttributeDataType,
});

define_orm_with_common_fields!(AttributeValue {
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
    pub attribute_id: i32,
    pub entity_id: i32,
    pub entity_type: String,
});

pub trait SqlxViewRepository: ViewRepository<Self::Entity> {
    type Entity;
    type Orm: for<'r> FromRow<'r, PgRow> + Send + Unpin;
    fn get_table_name(&self) -> &str;
    fn get_columns(&self) -> Vec<&str>;
    fn get_searchable_columns(&self) -> Vec<&str>;
    fn get_pool(&self) -> &PgPool;

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
            self.get_table_name()
        ))
        .bind(id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(result.map(|orm| Self::from_orm(orm)))
    }

    async fn find_by_uid(&self, uid: Uuid) -> Result<Option<Self::Entity>, CoreError> {
        let result = sqlx::query_as::<_, Self::Orm>(&format!(
            "SELECT * FROM {} WHERE uid=$1",
            self.get_table_name()
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
            self.get_table_name()
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
                self.get_table_name()
            ));
            query_builder.push_bind(self.get_table_name());

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

                let searchable_columns = self.get_searchable_columns();
                query_builder.push("(to_tsvector('simple', unaccent(");
                for (i, col) in searchable_columns.iter().enumerate() {
                    if i > 0 {
                        query_builder.push(" || ' ' || ");
                    }
                    query_builder.push("coalesce(").push(col).push(", '')");
                }
                query_builder
                    .push(")) @@ to_tsquery('simple', unaccent(")
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
                        .push(self.get_table_name()).push(".id")
                        .push(" AND av.entity_type = ")
                        .push_bind(self.get_table_name())
                        .push(" AND to_tsvector('simple', unaccent(coalesce(av.string_value, ''))) @@ to_tsquery('simple', unaccent(")
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
        builder.push(format!("{} ", field));

        Self::handle_operator(builder, operator, value);
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

        let value_column = match value {
            FilterValue::Int(_) | FilterValue::IntRange(_, _) | FilterValue::ListInt(_) => {
                "int_value"
            }
            FilterValue::Float(_) | FilterValue::FloatRange(_, _) | FilterValue::ListFloat(_) => {
                "double_value"
            }
            FilterValue::String(_) | FilterValue::ListString(_) => "string_value",
            FilterValue::Bool(_) => "boolean_value",
            FilterValue::Date(_) | FilterValue::DateRange(_, _) => "date_value",
            FilterValue::DateTime(_) | FilterValue::DateTimeRange(_, _) => "datetime_value",
            FilterValue::Time(_) | FilterValue::TimeRange(_, _) => "time_value",
        };

        builder.push(format!(" AND av.{} ", value_column));
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
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::String(v) => builder.push_bind(v),
                    FilterValue::Bool(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("Equal not supported for this value type"),
                }
            }

            FilterOperator::NotEqual => {
                builder.push("!= ");
                match value {
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::String(v) => builder.push_bind(v),
                    FilterValue::Bool(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("NotEqual not supported for this value type"),
                }
            }

            FilterOperator::Like => match value {
                FilterValue::String(v) => builder.push("LIKE ").push_bind(v),
                _ => unimplemented!("Like only supports string"),
            },

            FilterOperator::NotLike => match value {
                FilterValue::String(v) => builder.push("NOT LIKE ").push_bind(v),
                _ => unimplemented!("NotLike only supports string"),
            },

            FilterOperator::GreaterThan => {
                builder.push("> ");
                match value {
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("GreaterThan not supported for this value type"),
                }
            }

            FilterOperator::GreaterThanOrEqual => {
                builder.push(">= ");
                match value {
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("GreaterThanOrEqual not supported for this value type"),
                }
            }

            FilterOperator::LessThan => {
                builder.push("< ");
                match value {
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("LessThan not supported for this value type"),
                }
            }

            FilterOperator::LessThanOrEqual => {
                builder.push("<= ");
                match value {
                    FilterValue::Int(v) => builder.push_bind(v),
                    FilterValue::Float(v) => builder.push_bind(v),
                    FilterValue::Date(v) => builder.push_bind(v),
                    FilterValue::DateTime(v) => builder.push_bind(v),
                    FilterValue::Time(v) => builder.push_bind(v),
                    _ => unimplemented!("LessThanOrEqual not supported for this value type"),
                }
            }

            FilterOperator::Is => match value {
                FilterValue::Bool(v) => builder.push("= ").push_bind(v),
                _ => unimplemented!("Is not supported for this value type"),
            },

            FilterOperator::In => match value {
                FilterValue::ListInt(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| {
                    b.push_bind(v);
                }),
                FilterValue::ListFloat(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| {
                    b.push_bind(v);
                }),
                FilterValue::ListString(vs) => builder.push("IN ").push_tuples(vs, |mut b, v| {
                    b.push_bind(v);
                }),
                _ => unimplemented!("In only supports list values"),
            },

            FilterOperator::NotIn => match value {
                FilterValue::ListInt(vs) => builder.push("NOT IN ").push_tuples(vs, |mut b, v| {
                    b.push_bind(v);
                }),
                FilterValue::ListFloat(vs) => {
                    builder.push("NOT IN ").push_tuples(vs, |mut b, v| {
                        b.push_bind(v);
                    })
                }
                FilterValue::ListString(vs) => {
                    builder.push("NOT IN ").push_tuples(vs, |mut b, v| {
                        b.push_bind(v);
                    })
                }
                _ => unimplemented!("NotIn only supports list values"),
            },

            FilterOperator::IsNull => builder.push("IS NULL"),

            FilterOperator::NotNull => builder.push("IS NOT NULL"),

            FilterOperator::Between => {
                builder.push("BETWEEN ");
                match value {
                    FilterValue::IntRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::FloatRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::DateRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::DateTimeRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::TimeRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    _ => unimplemented!("Between only supports range types"),
                }
            }

            FilterOperator::NotBetween => {
                builder.push("NOT BETWEEN ");
                match value {
                    FilterValue::IntRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::FloatRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::DateRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::DateTimeRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    FilterValue::TimeRange(from, to) => {
                        builder.push_bind(from).push(" AND ").push_bind(to)
                    }
                    _ => unimplemented!("NotBetween only supports range types"),
                }
            }
        };
    }
}
pub trait SqlxRepository:
    SqlxViewRepository + Repository<<Self as SqlxViewRepository>::Entity, Self::EntityCreate>
where
    Self::EntityCreate: Creatable<Entity = Self::Entity>,
{
    type EntityCreate;

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        let result = sqlx::query(&format!(
            "DELETE FROM {} WHERE id = $1",
            self.get_table_name()
        ))
        .bind(id)
        .execute(self.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    async fn delete_by_uid(&self, uid: Uuid) -> Result<u64, CoreError> {
        let result = sqlx::query(&format!(
            "DELETE FROM {} WHERE uid = $1",
            self.get_table_name()
        ))
        .bind(uid)
        .execute(self.get_pool())
        .await?;

        Ok(result.rows_affected())
    }
}
