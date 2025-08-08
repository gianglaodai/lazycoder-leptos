use crate::business::error::CoreError;
use crate::business::filter::Filter;
use std::future::Future;
use crate::business::sort::SortCriterion;

pub trait Repository<T, C> {
    fn find_all(&self, filters: Vec<Filter>) -> impl Future<Output = Result<Vec<T>, CoreError>> {
        self.find_many(vec![], None, None, filters)
    }

    fn count(
        &self,
        filters: Vec<Filter>,
    ) -> impl Future<Output = Result<i64, CoreError>>;
    fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> impl Future<Output = Result<Vec<T>, CoreError>>;
    fn find_by_id(&self, id: i32) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn find_by_uid(&self, uid: String) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, CoreError>>;
    fn delete_by_uid(&self, uid: String) -> impl Future<Output = Result<u64, CoreError>>;
    fn create(&self, entity_create: &C) -> impl Future<Output = Result<T, CoreError>>;
    fn update(&self, entity: &T) -> impl Future<Output = Result<T, CoreError>>;
}
