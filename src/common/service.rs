use crate::common::cache::cache_get_or_compute;
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::repository::{Creatable, Repository, ViewRepository};
use crate::common::sort::SortCriterion;
use std::collections::HashMap;
use std::future::Future;

pub type PropertyName = String;
pub type AttributeName = String;

// Common trait for business layer entities with standard fields
pub trait Entity {
    fn id(&self) -> i32;
    fn uid(&self) -> &str;
    fn version(&self) -> i32;
    fn created_at(&self) -> time::OffsetDateTime;
    fn updated_at(&self) -> time::OffsetDateTime;
}

pub const FIELD_TYPE_MAP: &str = "FIELD_TYPE_MAP";
pub const ATTRIBUTE_TYPE_MAP: &str = "ATTRIBUTE_TYPE_MAP";
pub trait ViewService {
    type Entity;
    type Repo: ViewRepository<Self::Entity> + Send + Sync;

    fn get_repository(&self) -> &Self::Repo;

    fn get_property_type_map(
        &self,
    ) -> impl Future<Output = Result<HashMap<PropertyName, ScalarValue>, CoreError>> {
        let repo_ref = self.get_repository();
        let table = repo_ref.get_table_name().to_string();
        async move {
            cache_get_or_compute(FIELD_TYPE_MAP, &table, || async move {
                repo_ref.get_column_type_map().await
            })
            .await
        }
    }

    // Default read-only operations
    fn get_all(
        &self,
        filters: Vec<Filter>,
    ) -> impl Future<Output = Result<Vec<Self::Entity>, CoreError>> {
        let repo = self.get_repository();
        async move { repo.find_all(filters).await }
    }

    fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> impl Future<Output = Result<Vec<Self::Entity>, CoreError>> {
        let repo = self.get_repository();
        async move {
            repo.find_many(sort_criteria, first_result, max_results, filters)
                .await
        }
    }

    fn count(&self, filters: Vec<Filter>) -> impl Future<Output = Result<i64, CoreError>> {
        let repo = self.get_repository();
        async move { repo.count(filters).await }
    }

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<Option<Self::Entity>, CoreError>> {
        let repo = self.get_repository();
        async move { repo.find_by_id(id).await }
    }

    fn get_by_uid(
        &self,
        uid: String,
    ) -> impl Future<Output = Result<Option<Self::Entity>, CoreError>> {
        let repo = self.get_repository();
        async move { repo.find_by_uid(uid).await }
    }
}

pub trait Service: ViewService {
    type Create: Creatable<Entity = Self::Entity>;

    fn get_attribute_type_map(
        &self,
    ) -> impl Future<Output = Result<HashMap<AttributeName, ScalarValue>, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let entity_type = self.get_repository().get_table_name().to_string();
        async move {
            cache_get_or_compute(ATTRIBUTE_TYPE_MAP, &entity_type, || {
                let repo_ref = self.get_repository();
                async move { Self::Repo::get_attribute_type_map(repo_ref).await }
            })
            .await
        }
    }

    // Default write operations
    fn create(
        &self,
        entity_create: &Self::Create,
    ) -> impl Future<Output = Result<Self::Entity, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.create(entity_create).await }
    }

    fn update(&self, entity: &Self::Entity) -> impl Future<Output = Result<Self::Entity, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.update(entity).await }
    }

    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.delete_by_id(id).await }
    }

    fn delete_by_ids(&self, ids: Vec<i32>) -> impl Future<Output = Result<u64, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.delete_by_ids(ids).await }
    }

    fn delete_by_uid(&self, uid: String) -> impl Future<Output = Result<u64, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.delete_by_uid(uid).await }
    }

    fn delete_by_uids(&self, uids: Vec<String>) -> impl Future<Output = Result<u64, CoreError>>
    where
        <Self as ViewService>::Repo: Repository<Self::Entity, Self::Create>,
    {
        let repo = self.get_repository();
        async move { repo.delete_by_uids(uids).await }
    }
}
