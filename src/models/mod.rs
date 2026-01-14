use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitHubUser {
    pub login: String,
    pub avatar_url: String,
    pub public_repos: u32,
    pub followers: u32,
    pub bio: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthUser {
    pub email: String,
}

#[server]
pub async fn get_github_user(username: String) -> Result<GitHubUser, ServerFnError> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://api.github.com/users/{}", username))
        .header("User-Agent", "dioxus-cv-app")
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to send request: {}", e)))?;

    let user = res
        .json::<GitHubUser>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse JSON: {}", e)))?;
    Ok(user)
}

#[cfg(feature = "server")]
static LOGGED_IN_USER: std::sync::Mutex<Option<AuthUser>> = std::sync::Mutex::new(None);

#[server]
pub async fn get_current_user() -> Result<Option<AuthUser>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(LOGGED_IN_USER.lock().unwrap().clone())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(None)
    }
}

#[server]
pub async fn login_mock() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        *LOGGED_IN_USER.lock().unwrap() = Some(AuthUser {
            email: "taianmeca@gmail.com".to_string(),
        });
        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        *LOGGED_IN_USER.lock().unwrap() = None;
        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}
