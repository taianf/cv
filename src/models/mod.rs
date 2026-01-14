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
