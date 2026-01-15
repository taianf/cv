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

#[server]
pub async fn get_google_auth_url() -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use oauth2::basic::BasicClient;
        use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope};

        let client_id = std::env::var("GOOGLE_CLIENT_ID").ok();
        let app_env = std::env::var("APP_ENV").unwrap_or_default().to_lowercase();

        // Mock and Dev strategy
        let is_placeholder = client_id
            .as_deref()
            .map(|id| id.contains("your_"))
            .unwrap_or(true);
        let is_dev = app_env == "development" || app_env == "dev";

        if is_placeholder || is_dev {
            return Ok("/auth/callback?code=mock_code_for_dev".to_string());
        }

        let client_id = client_id.unwrap();
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .unwrap_or_else(|_| "YOUR_CLIENT_SECRET".to_string());
        let redirect_url = std::env::var("GOOGLE_REDIRECT_URL")
            .unwrap_or_else(|_| "http://localhost:8080/auth/callback".to_string());

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            None,
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        let (auth_url, _csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .url();

        Ok(auth_url.to_string())
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server]
pub async fn exchange_code_for_user(code: String) -> Result<AuthUser, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use oauth2::basic::BasicClient;
        use oauth2::{
            AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse,
            TokenUrl,
        };

        // Dev/Mock strategy
        let app_env = std::env::var("APP_ENV").unwrap_or_default().to_lowercase();
        let is_dev = app_env == "development" || app_env == "dev";

        if code == "mock_code_for_dev" || is_dev {
            return Ok(AuthUser {
                email: "dev.user@example.com".to_string(),
            });
        }

        let client_id =
            std::env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "YOUR_CLIENT_ID".to_string());
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .unwrap_or_else(|_| "YOUR_CLIENT_SECRET".to_string());
        let redirect_url = std::env::var("GOOGLE_REDIRECT_URL")
            .unwrap_or_else(|_| "http://localhost:8080/auth/callback".to_string());

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| ServerFnError::new(format!("Token exchange failed: {}", e)))?;

        let access_token = token_result.access_token().secret();

        // Fetch user info from Google
        let client = reqwest::Client::new();
        let user_info: serde_json::Value = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to fetch user info: {}", e)))?
            .json()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to parse user info: {}", e)))?;

        let email = user_info["email"]
            .as_str()
            .ok_or_else(|| ServerFnError::new("Email not found in user info"))?
            .to_string();

        Ok(AuthUser { email })
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
