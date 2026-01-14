use cv::models::GitHubUser;

#[test]
fn test_github_user_deserialization() {
    let json = r#"
    {
        "login": "taianf",
        "avatar_url": "https://avatars.githubusercontent.com/u/1234567?v=4",
        "public_repos": 10,
        "followers": 20,
        "bio": "Software Engineer"
    }
    "#;

    let user: GitHubUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.login, "taianf");
    assert_eq!(user.public_repos, 10);
    assert_eq!(user.followers, 20);
    assert_eq!(user.bio, Some("Software Engineer".to_string()));
}

#[test]
fn test_github_user_serialization() {
    let user = GitHubUser {
        login: "testuser".to_string(),
        avatar_url: "https://example.com/avatar.png".to_string(),
        public_repos: 5,
        followers: 100,
        bio: None,
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("\"login\":\"testuser\""));
    assert!(json.contains("\"public_repos\":5"));
    assert!(json.contains("\"followers\":100"));
    assert!(json.contains("\"bio\":null"));
}
