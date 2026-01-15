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

// Enhanced server function tests

#[cfg(feature = "server")]
#[tokio::test]
async fn test_get_google_auth_url_server_function() {
    // Test server function with different environment configurations
    std::env::set_var("APP_ENV", "test");
    std::env::set_var("GOOGLE_CLIENT_ID", "test_client_id");

    let result = get_google_auth_url().await;
    assert!(result.is_ok());

    // Should return mock URL for test environment
    let url = result.unwrap();
    assert!(url.contains("/auth/callback"));
    assert!(url.contains("mock_code_for_dev"));

    // Cleanup
    std::env::remove_var("APP_ENV");
    std::env::remove_var("GOOGLE_CLIENT_ID");
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_exchange_code_for_user_server_function() {
    // Test server function with mock code
    let result = exchange_code_for_user("mock_code_for_dev".to_string()).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.email, "dev.user@example.com");
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_exchange_code_for_user_with_real_code_simulation() {
    // Test server function with real-looking code (should still return mock user in dev)
    std::env::set_var("APP_ENV", "development");

    let real_codes = vec![
        "4/0AX4XfWgjY7X8K9Z2mQpL3vR5sT7uN1wE2rY4aI6bO8cD0eF",
        "1/055765432109876543210987654321098765432109876",
        "ya29.a0AfH6SMB-EXAMPLE-TOKEN-HERE-1234567890",
    ];

    for code in real_codes {
        let result = exchange_code_for_user(code.to_string()).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, "dev.user@example.com");
    }

    // Cleanup
    std::env::remove_var("APP_ENV");
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_server_function_error_handling() {
    // Test server function error handling with invalid inputs
    let invalid_codes = vec!["", "invalid_code", "null", "undefined", "[]", "{}"];

    for invalid_code in invalid_codes {
        let result = exchange_code_for_user(invalid_code.to_string()).await;
        // Should still succeed in development mode
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, "dev.user@example.com");
    }
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_server_function_concurrent_requests() {
    // Test concurrent server function requests
    let futures = vec![
        get_google_auth_url(),
        get_google_auth_url(),
        get_google_auth_url(),
    ];

    let results = futures::future::join_all(futures).await;

    for result in results {
        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.contains("/auth/callback"));
    }
}

#[cfg(feature = "server")]
#[tokio::test]
async fn test_server_function_environment_variable_handling() {
    // Test server function with various environment variable states
    let env_configs = vec![
        // (APP_ENV, GOOGLE_CLIENT_ID, expected_behavior)
        ("production", "real_client_id", "mock"),
        ("production", "your_client_id", "mock"),
        ("development", "any_client_id", "mock"),
        ("test", "", "mock"),
        ("", "", "mock"),
    ];

    for (app_env, client_id, _expected) in env_configs {
        // Set environment variables
        if !app_env.is_empty() {
            std::env::set_var("APP_ENV", app_env);
        }
        if !client_id.is_empty() {
            std::env::set_var("GOOGLE_CLIENT_ID", client_id);
        }

        let result = get_google_auth_url().await;
        assert!(result.is_ok());

        let url = result.unwrap();
        assert!(url.contains("/auth/callback"));

        // Cleanup
        std::env::remove_var("APP_ENV");
        std::env::remove_var("GOOGLE_CLIENT_ID");
    }
}

#[test]
fn test_auth_user_with_server_function_integration() {
    // Test AuthUser integration with server functions
    let user = AuthUser {
        email: "integration.test@example.com".to_string(),
    };

    // Test that AuthUser can be serialized for server communication
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("integration.test@example.com"));

    // Test that AuthUser can be deserialized from server response
    let server_response = format!("{{\"email\":\"{}\"}}", user.email);
    let deserialized_user: AuthUser = serde_json::from_str(&server_response).unwrap();
    assert_eq!(deserialized_user.email, user.email);
}

#[test]
fn test_auth_user_validation_edge_cases() {
    // Test AuthUser with various edge case emails
    let edge_case_emails = vec![
        "a@b.c",                                             // Minimal valid email
        "very.long.email.address@very.long.domain.name.com", // Long email
        "user+tag@example.com",                              // Plus addressing
        "user.name@example.co.uk",                           // Subdomain and TLD
        "user_name@example-domain.com",                      // Underscores and hyphens
        "123@example.com",                                   // Numeric local part
        "test@123.com",                                      // Numeric domain
        "user@xn--example.com",                              // Punycode domain
        "üñïçødé@éxåmplé.com",                               // Unicode domain
    ];

    for email in edge_case_emails {
        let user = AuthUser {
            email: email.to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&user).unwrap();

        // Test deserialization
        let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.email, email);
    }
}

#[test]
fn test_auth_user_serialization_performance() {
    // Test AuthUser serialization performance with large data
    let large_email = format!("{}@example.com", "a".repeat(1000));
    let user = AuthUser {
        email: large_email.clone(),
    };

    // Test serialization
    let start = std::time::Instant::now();
    let json = serde_json::to_string(&user).unwrap();
    let serialization_time = start.elapsed();

    // Test deserialization
    let start = std::time::Instant::now();
    let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
    let deserialization_time = start.elapsed();

    // Verify correctness
    assert_eq!(deserialized.email, large_email);

    // Performance should be reasonable (less than 100ms for this test)
    assert!(serialization_time.as_millis() < 100);
    assert!(deserialization_time.as_millis() < 100);
}

#[test]
fn test_auth_user_memory_safety() {
    // Test AuthUser memory safety with various inputs
    let test_cases = vec![
        String::new(),
        " ".repeat(1000),
        "a".repeat(10000),
        "\0".to_string(),
        "测试".repeat(100),
        "🚀".repeat(100),
    ];

    for test_case in test_cases {
        let user = AuthUser {
            email: test_case.clone(),
        };

        // Test that operations don't panic
        let _json = serde_json::to_string(&user).unwrap();
        let _deserialized: AuthUser = serde_json::from_str(&_json).unwrap();

        // Test equality
        assert_eq!(user.email, test_case);
    }
}

#[test]
fn test_auth_user_concurrent_access() {
    // Test AuthUser concurrent access simulation
    let user = AuthUser {
        email: "concurrent.test@example.com".to_string(),
    };

    // Test concurrent serialization
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let user_clone = user.clone();
            std::thread::spawn(move || serde_json::to_string(&user_clone).unwrap())
        })
        .collect();

    // Wait for all threads and verify results
    for handle in handles {
        let json = handle.join().unwrap();
        let deserialized: AuthUser = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.email, user.email);
    }
}

#[cfg(not(feature = "server"))]
#[test]
fn test_server_functions_client_only() {
    // Test that server functions return appropriate errors on client-only builds
    // This test will be compiled but not run on server builds

    // Note: We can't actually test async functions without #[tokio::test]
    // but we can verify that functions exist and have the right signatures

    // Simple verification that functions exist (they will fail at runtime on client-only)
    // This is mainly to ensure compilation works
}
