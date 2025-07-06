use crate::business::error::CoreError;
use std::future::Future;
use std::str::FromStr;
use uuid::Uuid;

pub struct SortCriterion {
    pub field: String,
    pub ascending: bool,
}

impl FromStr for SortCriterion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Ok(SortCriterion {
                field: "".to_string(),
                ascending: true,
            })
        }
        let (ascending, field) = match s.chars().next().unwrap() {
            '+' => (true, &s[1..].trim()),
            '-' => (false, &s[1..].trim()),
            _ => (true, &s.trim()),
        };
        Ok(SortCriterion { field: field.to_string(), ascending })
    }
}
pub trait Repository<T> {
    fn find_all(&self) -> impl Future<Output=Result<Vec<T>, CoreError>>;
    fn find_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>) -> impl Future<Output = Result<Vec<T>, CoreError>>;
    fn find_by_id(&self, id: i32) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn find_by_uid(&self, uid: Uuid) -> impl Future<Output = Result<Option<T>, CoreError>>;
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, CoreError>>;
    fn delete_by_uid(&self, uid: Uuid) -> impl Future<Output = Result<u64, CoreError>>;
    fn create(&self, entity: &T) -> impl Future<Output = Result<T, CoreError>>;
    fn update(&self, entity: &T) -> impl Future<Output = Result<T, CoreError>>;
}
