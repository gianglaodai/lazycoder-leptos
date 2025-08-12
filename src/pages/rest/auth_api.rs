use leptos::prelude::*;

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

#[server(prefix = "/api", endpoint = "auth/login")]
pub async fn auth_login(
    username_or_email: String,
    password: String,
    remember: bool,
) -> Result<UserRole, ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos::prelude::expect_context;

    // Mock authentication: admin/admin => ADMIN; any non-empty username with password "password" => USER
    let role = if (username_or_email.eq_ignore_ascii_case("admin")
        || username_or_email.eq_ignore_ascii_case("admin@example.com"))
        && password == "admin"
    {
        Some(UserRole::ADMIN)
    } else if !username_or_email.is_empty() && password == "password" {
        Some(UserRole::USER)
    } else {
        None
    };

    if let Some(role) = role {
        let req = expect_context::<actix_web::HttpRequest>();
        let session = req.get_session();
        session
            .insert(
                "role",
                match role {
                    UserRole::ADMIN => "ADMIN",
                    UserRole::USER => "USER",
                },
            )
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        session.insert("remember", remember).ok();
        Ok(role)
    } else {
        Err(ServerFnError::new("Invalid credentials"))
    }
}

#[server(prefix = "/api", endpoint = "auth/me")]
pub async fn auth_me() -> Result<UserRole, ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos::prelude::expect_context;

    let req = expect_context::<actix_web::HttpRequest>();
    let session = req.get_session();
    let role: Option<String> = session
        .get("role")
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    match role.as_deref() {
        Some("ADMIN") => Ok(UserRole::ADMIN),
        Some("USER") => Ok(UserRole::USER),
        _ => Err(ServerFnError::new("Unauthenticated")),
    }
}

#[server(prefix = "/api", endpoint = "auth/logout")]
pub async fn auth_logout() -> Result<(), ServerFnError> {
    use actix_session::SessionExt as _;
    use leptos::prelude::expect_context;

    let req = expect_context::<actix_web::HttpRequest>();
    let session = req.get_session();
    session.purge();
    Ok(())
}
