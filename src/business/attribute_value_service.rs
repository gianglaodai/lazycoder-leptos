use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: attribute_values
define_struct_with_common_fields!(AttributeValue {
    req {
        pub attribute_id: i32,
        pub entity_id: i32,
        pub entity_type: String,
    }
    opt {
        pub int_value: Option<i32>,
        pub double_value: Option<f64>,
        pub string_value: Option<String>,
        pub boolean_value: Option<bool>,
        pub date_value: Option<time::Date>,
        pub datetime_value: Option<time::OffsetDateTime>,
        pub time_value: Option<time::Time>,
    }
});

// View entity: attribute_values with joined attribute metadata
define_readonly_struct_with_common_fields!(AttributeValueInfo {
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
    pub attribute_id: i32,
    pub attribute_name: String,
    pub attribute_entity_type: String,
    pub attribute_data_type: String,
    pub entity_id: i32,
    pub entity_type: String,
});

// Repositories
pub trait AttributeValueRepository:
    Repository<AttributeValue, AttributeValueCreate> + Send + Sync
{
}
pub trait AttributeValueInfoRepository: ViewRepository<AttributeValueInfo> + Send + Sync {}

// Service for table (CRUD)
#[derive(Clone)]
pub struct AttributeValueService<R: AttributeValueRepository> {
    repository: Arc<R>,
}

impl<R: AttributeValueRepository> AttributeValueService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: AttributeValueRepository> ViewService for AttributeValueService<R> {
    type Entity = AttributeValue;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: AttributeValueRepository> Service for AttributeValueService<R> {
    type Create = AttributeValueCreate;
}

// Service for view (read-only)
#[derive(Clone)]
pub struct AttributeValueInfoService<R: AttributeValueInfoRepository> {
    repository: Arc<R>,
}

impl<R: AttributeValueInfoRepository> AttributeValueInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: AttributeValueInfoRepository> ViewService for AttributeValueInfoService<R> {
    type Entity = AttributeValueInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
