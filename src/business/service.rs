use crate::business::error::CoreError;
use crate::business::filter::ScalarValue;
use crate::business::repository::{Creatable, Repository, ViewRepository};
use std::collections::HashMap;
use std::future::Future;

pub type PropertyName = String;
pub type AttributeName = String;

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
            use crate::business::cache::{cache_get_or_compute, FIELD_TYPE_MAP};
            cache_get_or_compute(FIELD_TYPE_MAP, &table, || async move {
                repo_ref.get_column_type_map().await
            })
            .await
        }
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
            use crate::business::cache::{cache_get_or_compute, ATTRIBUTE_TYPE_MAP};
            cache_get_or_compute(ATTRIBUTE_TYPE_MAP, &entity_type, || {
                let repo_ref = self.get_repository();
                async move {
                    <Self::Repo as Repository<Self::Entity, Self::Create>>::get_attribute_type_map(
                        repo_ref,
                    )
                    .await
                }
            })
            .await
        }
    }
}
