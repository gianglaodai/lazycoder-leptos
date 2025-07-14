#![cfg(feature = "ssr")]

use leptos::ev::close;
use leptos::tachys::html::property::Property;
use crate::business::error::CoreError;
use crate::business::repository::{Repository, SortCriterion};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, QueryBuilder};
use uuid::Uuid;
use crate::business::filter::{Filter, FilterOperator, FilterValue};

enum BindValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Uuid(Uuid),
    Date(time::Date),
    DateTime(time::OffsetDateTime),
    Time(time::Time),
}
pub trait SqlxRepository: Repository<Self::Entity> {
    type Entity;
    type Orm: for<'r> FromRow<'r, PgRow> + Send + Unpin;

    fn get_table_name(&self) -> &str;
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
            QueryBuilder::new(format!("SELECT * FROM {}", self.get_table_name()));
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

        query_builder.push(format!(" OFFSET {}", first_result.unwrap_or(0)));
        match max_results {
            Some(limit) => query_builder.push(format!(" LIMIT {}", limit)),
            None => query_builder.push(" LIMIT ALL"),
        };

        let result = query_builder
            .build_query_as::<Self::Orm>()
            .fetch_all(self.get_pool())
            .await?;

        Ok(result.into_iter().map(|orm| Self::from_orm(orm)).collect())
    }

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

    fn build_order_by(sort_criteria: Vec<SortCriterion>) -> Option<String> {
        if sort_criteria.is_empty() {
            return None;
        }
        let parts: Vec<String> = sort_criteria
            .iter()
            .map(|criterion| format!(
                "{} {}",
                criterion.field,
                if criterion.ascending { "ASC" } else { "DESC" }
            ))
            .collect();
        Some(format!("ORDER BY {}", parts.join(", ")))
    }
    fn build_property_filter(filter: &Filter) -> Result<(String, Vec<BindValue>), CoreError> {
        match filter {
            Filter::Property { property_name, operator, value } => {
                let column = property_name.clone();
                let mut binds = vec![];
                let clause = match operator {
                    FilterOperator::Equal => {
                        match value {
                            FilterValue::String(v) => {
                                binds.push(BindValue::String(v.clone()));
                                format!("{} = '{}'", column, binds.len())
                            }
                            FilterValue::Int(v) => {
                                binds.push(BindValue::Int(v.clone()));
                                format!("{} = {}", column, binds.len())
                            }
                            FilterValue::Float(v) => {
                                binds.push(BindValue::Float(v.clone()));
                                format!("{} = {}", column, binds.len())
                            }
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!(),
                };
                Ok((clause, binds))
            }
            _ => Err(CoreError::ValidationError("Invalid filter".to_string())),
        }
    }
}
