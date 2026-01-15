use cv::Route;

#[test]
fn test_route_home_creation() {
    let route = Route::Home {};
    assert_eq!(format!("{:?}", route), "Home");
}

#[test]
fn test_route_blog_creation() {
    let route = Route::Blog {};
    assert_eq!(format!("{:?}", route), "Blog");
}

#[test]
fn test_route_profile_creation() {
    let route = Route::Profile {};
    assert_eq!(format!("{:?}", route), "Profile");
}

#[test]
fn test_route_auth_callback_creation() {
    let route = Route::AuthCallback {};
    assert_eq!(format!("{:?}", route), "AuthCallback");
}

#[test]
fn test_route_equality() {
    let route1 = Route::Home {};
    let route2 = Route::Home {};
    assert_eq!(route1, route2);
}

#[test]
fn test_route_inequality() {
    let route1 = Route::Home {};
    let route2 = Route::Blog {};
    assert_ne!(route1, route2);
}

#[test]
fn test_route_clone() {
    let route1 = Route::Home {};
    let route2 = route1.clone();
    assert_eq!(route1, route2);
}

#[test]
fn test_all_routes_are_unique() {
    let routes = vec![
        Route::Home {},
        Route::Blog {},
        Route::Profile {},
        Route::AuthCallback {},
    ];

    // Verify each route is different from the others
    for (i, route1) in routes.iter().enumerate() {
        for (j, route2) in routes.iter().enumerate() {
            if i == j {
                assert_eq!(route1, route2);
            } else {
                assert_ne!(route1, route2);
            }
        }
    }
}
