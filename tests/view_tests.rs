use cv::views::{Blog, Home, Profile};
use dioxus::prelude::*;

#[test]
fn test_blog_view_creation() {
    let _view = rsx! {
        Blog {}
    };
}

#[test]
fn test_home_view_creation() {
    let _view = rsx! {
        Home {}
    };
}

#[test]
fn test_profile_view_creation() {
    // Note: Profile view requires AuthUser context, so we just test compilation
    let _view = rsx! {
        Profile {}
    };
}

#[test]
fn test_multiple_blog_instances() {
    let _view1 = rsx! { Blog {} };
    let _view2 = rsx! { Blog {} };
    // Both should be able to coexist
}

#[test]
fn test_multiple_home_instances() {
    let _view1 = rsx! { Home {} };
    let _view2 = rsx! { Home {} };
    // Both should be able to coexist
}
