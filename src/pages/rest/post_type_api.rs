use crate::business::post_type_service::PostType;
use crate::common::error::CoreError;
use crate::common::service::Service;
use crate::define_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;

define_to_with_common_fields_fe!(PostType {
    pub code: String,
    pub name: String,
});
impl From<PostTypeTO> for PostType {
    fn from(to: PostTypeTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            code: to.code,
            name: to.name,
        }
    }
}
impl From<PostType> for PostTypeTO {
    fn from(entity: PostType) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            code: entity.code,
            name: entity.name,
        }
    }
}
#[server(name=CreatePostType, prefix="/load", endpoint="/post_types/create")]
pub async fn create_post_type(code: String, name: String) -> Result<PostTypeTO, ServerFnError> {
    use crate::business::post_type_service::PostTypeCreate;
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let role: Option<String> = match session.get("role") {
        Ok(v) => v,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.missing_session").to_json(),
            ))
        }
    };
    match role.as_deref() {
        Some("ADMIN") => {}
        _ => {
            return Err(ServerFnError::ServerError(
                CoreError::forbidden("error.forbidden").to_json(),
            ))
        }
    }

    let state: actix_web::web::Data<AppState> = extract().await?;
    let create = PostTypeCreate { code, name };
    state
        .post_type_service
        .create(&create)
        .await
        .map(PostTypeTO::from)
        .map_err(|e| e.to_json())
        .map_err(ServerFnError::ServerError)
}
