use crate::business::user_service::User;
use crate::define_to_with_common_fields_fe;
use leptos::prelude::*;
use leptos::prelude::ServerFnError;
use crate::business::error::CoreError;

define_to_with_common_fields_fe!(User {
    pub username: String,
    pub email: String,
    pub role: UserRole,
});

impl From<User> for UserTO {
    fn from(to: User) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            username: to.username,
            email: to.email,
            role: UserRole::from(to.role),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UserRole {
    USER,
    ADMIN,
}

impl From<crate::business::user_service::UserRole> for UserRole {
    fn from(value: crate::business::user_service::UserRole) -> Self {
        match value {
            crate::business::user_service::UserRole::ADMIN => UserRole::ADMIN,
            crate::business::user_service::UserRole::USER => UserRole::USER,
        }
    }
}

#[server(name=Register, prefix = "/load", endpoint = "/auth/register")]
pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .auth_service
        .register(username, email, password)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=Login, prefix = "/load", endpoint = "/auth/login")]
pub async fn login(
    username_or_email: String,
    password: String,
    remember: bool,
) -> Result<UserTO, ServerFnError> {
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    // Use AuthService to authenticate
    let state: actix_web::web::Data<AppState> = extract().await?;
    let user = match state
        .auth_service
        .login(username_or_email, password)
        .await
    {
        Ok(u) => UserTO::from(u),
        Err(e) => return Err(ServerFnError::ServerError(e.to_json())),
    };

    let role = user.role;
    // Set session info
    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    // Store full user in session so client can avoid extra me() calls
    if let Err(_) = session.insert("user", &user) {
        return Err(ServerFnError::ServerError(CoreError::unauthorized("error.cant_store_session").to_json()));
    }
    // Keep role for backward compatibility
    if let Err(_) = session.insert(
            "role",
            match role {
                UserRole::ADMIN => "ADMIN",
                UserRole::USER => "USER",
            },
        ) {
        return Err(ServerFnError::ServerError(CoreError::unauthorized("error.cant_store_session").to_json()));
    }
    session.insert("remember", remember).ok();
    Ok(user)
}

#[server(name=Me, prefix = "/load", endpoint = "/auth/me")]
pub async fn me() -> Result<UserRole, ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let role: Option<String> = match session.get("role") {
        Ok(v) => v,
        Err(_) => return Err(ServerFnError::ServerError(CoreError::unauthorized("error.missing_session").to_json())),
    };
    match role.as_deref() {
        Some("ADMIN") => Ok(UserRole::ADMIN),
        Some("USER") => Ok(UserRole::USER),
        _ => Err(ServerFnError::ServerError(CoreError::unauthorized("error.unauthorized").to_json())),
    }
}

#[server(name=CurrentUser, prefix = "/load", endpoint = "/auth/current-user")]
pub async fn current_user() -> Result<Option<UserTO>, ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let user: Option<UserTO> = match session.get("user") {
        Ok(v) => v,
        Err(_) => return Err(ServerFnError::ServerError(CoreError::unauthorized("error.missing_session").to_json())),
    };
    Ok(user)
}

#[server(name=Logout, prefix = "/load", endpoint = "/auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    session.purge();
    Ok(())
}
