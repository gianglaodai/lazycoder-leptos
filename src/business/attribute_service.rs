use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(Attribute {
    req {
        pub name: String,
        pub entity_type: String,
        pub data_type: String,
    }
    opt {}
});
define_readonly_struct_with_common_fields!(AttributeInfo {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

pub trait AttributeRepository: ViewRepository<Attribute> + Send + Sync {}
pub trait AttributeInfoRepository: ViewRepository<AttributeInfo> + Send + Sync {}

#[derive(Clone)]
pub struct AttributeService<R: AttributeRepository> {
    repository: Arc<R>,
}

impl<R: AttributeRepository> AttributeService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: AttributeRepository> ViewService for AttributeService<R> {
    type Entity = Attribute;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

#[derive(Clone)]
pub struct AttributeInfoService<R: AttributeInfoRepository> {
    repository: Arc<R>,
}

impl<R: AttributeInfoRepository> AttributeInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: AttributeInfoRepository> ViewService for AttributeInfoService<R> {
    type Entity = AttributeInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
