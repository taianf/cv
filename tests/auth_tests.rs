use cv::models::AuthUser;
use serde_json;

#[test]
fn test_auth_user_serialization() {
    let user = AuthUser {
        email: "test@example.com".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    assert_eq!(json, "{\"email\":\"test@example.com\"}");
}

#[test]
fn test_auth_user_deserialization() {
    let json = "{\"email\":\"test@example.com\"}";
    let user: AuthUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.email, "test@example.com");
}
