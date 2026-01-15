use cv::models::AuthUser;

#[test]
fn test_auth_user_creation() {
    let user = AuthUser {
        email: "test@example.com".to_string(),
    };
    assert_eq!(user.email, "test@example.com");
}

#[test]
fn test_auth_user_clone() {
    let user1 = AuthUser {
        email: "test@example.com".to_string(),
    };
    let user2 = user1.clone();
    assert_eq!(user1.email, user2.email);
}

#[test]
fn test_auth_user_equality() {
    let user1 = AuthUser {
        email: "test@example.com".to_string(),
    };
    let user2 = AuthUser {
        email: "test@example.com".to_string(),
    };
    assert_eq!(user1, user2);
}

#[test]
fn test_auth_user_inequality() {
    let user1 = AuthUser {
        email: "test1@example.com".to_string(),
    };
    let user2 = AuthUser {
        email: "test2@example.com".to_string(),
    };
    assert_ne!(user1, user2);
}

#[test]
fn test_auth_user_with_various_email_formats() {
    let valid_emails = vec![
        "user@example.com",
        "user.name@example.com",
        "user+tag@example.co.uk",
        "user_name@sub.example.com",
    ];

    for email in valid_emails {
        let user = AuthUser {
            email: email.to_string(),
        };
        assert_eq!(user.email, email);
    }
}

#[test]
fn test_auth_user_debug_format() {
    let user = AuthUser {
        email: "test@example.com".to_string(),
    };
    let debug_str = format!("{:?}", user);
    assert!(debug_str.contains("test@example.com"));
}

#[test]
fn test_auth_user_empty_email() {
    let user = AuthUser {
        email: String::new(),
    };
    assert_eq!(user.email, "");
}

#[test]
fn test_auth_user_long_email() {
    let long_email = format!("{}@example.com", "a".repeat(100));
    let user = AuthUser {
        email: long_email.clone(),
    };
    assert_eq!(user.email, long_email);
}
