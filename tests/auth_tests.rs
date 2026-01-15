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

#[test]
fn test_auth_user_round_trip_serialization() {
    let original = AuthUser {
        email: "roundtrip@example.com".to_string(),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_auth_user_serialization_with_special_chars() {
    let user = AuthUser {
        email: "user+tag@example.com".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
    assert_eq!(user.email, deserialized.email);
}

#[test]
fn test_auth_user_pretty_serialization() {
    let user = AuthUser {
        email: "pretty@example.com".to_string(),
    };
    let json = serde_json::to_string_pretty(&user).unwrap();
    assert!(json.contains("pretty@example.com"));
    assert!(json.contains("email"));
}

#[test]
fn test_auth_user_deserialization_with_extra_fields() {
    // JSON with extra fields should still deserialize
    let json = r#"{"email":"test@example.com","extra_field":"ignored"}"#;
    let user: AuthUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.email, "test@example.com");
}

#[test]
fn test_auth_user_serialization_empty_email() {
    let user = AuthUser {
        email: String::new(),
    };
    let json = serde_json::to_string(&user).unwrap();
    assert_eq!(json, "{\"email\":\"\"}");
}

#[test]
fn test_auth_user_deserialization_unicode() {
    let user = AuthUser {
        email: "user@例え.jp".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
    assert_eq!(user.email, deserialized.email);
}
