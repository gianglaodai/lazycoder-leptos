use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;
use std::collections::HashMap;
use std::future::Future;

pub trait Creatable {
    type Entity;
}

pub trait ViewRepository<T> {
    fn get_table_name(&self) -> &str;
    fn get_columns(&self) -> Vec<&str>;
    fn get_searchable_columns(&self) -> Vec<&str>;
    fn find_all(&self, filters: Vec<Filter>) -> impl Future<Output = Result<Vec<T>, CoreError>> {
        self.find_many(vec![], None, None, filters)
    }

    fn count(&self, filters: Vec<Filter>) -> impl Future<Output = Result<i64, CoreError>>;
    fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> impl Future<Output = Result<Vec<T>, CoreError>>;
    fn find_by_id(&self, id: i32) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn find_by_uid(&self, uid: String) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn get_column_type_map(
        &self,
    ) -> impl Future<Output = Result<HashMap<String, ScalarValue>, CoreError>>;
}

pub trait Repository<T, C: Creatable<Entity = T>>: ViewRepository<T> {
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, CoreError>>;
    fn delete_by_ids(&self, ids: Vec<i32>) -> impl Future<Output = Result<u64, CoreError>>;
    fn delete_by_uid(&self, uid: String) -> impl Future<Output = Result<u64, CoreError>>;
    fn delete_by_uids(&self, uids: Vec<String>) -> impl Future<Output = Result<u64, CoreError>>;
    fn create(&self, entity_create: &C) -> impl Future<Output = Result<T, CoreError>>;
    fn update(&self, entity: &T) -> impl Future<Output = Result<T, CoreError>>;
    fn get_attribute_type_map(
        &self,
    ) -> impl Future<Output = Result<HashMap<String, ScalarValue>, CoreError>>;
}
