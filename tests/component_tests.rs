use cv::components::{BlogPostCard, SectionCard, SocialLink};
use dioxus::prelude::*;

#[test]
fn test_blog_post_card_creation() {
    let title = "Test Post".to_string();
    let description = "Test Description".to_string();
    let index = 1;

    // Test that component can be created without panicking
    let _card = rsx! {
        BlogPostCard {
            title: title.clone(),
            description: description.clone(),
            index: index
        }
    };
}

#[test]
fn test_section_card_creation() {
    let title = "Test Section".to_string();
    let icon = "fa-test".to_string();

    let _card = rsx! {
        SectionCard {
            title: title.clone(),
            icon: icon.clone(),
            div { "Test content" }
        }
    };
}

#[test]
fn test_social_link_creation() {
    let href = "https://example.com".to_string();
    let icon = "fa-test".to_string();
    let label = "Test Link".to_string();

    let _link = rsx! {
        SocialLink {
            href: href.clone(),
            icon: icon.clone(),
            label: label.clone()
        }
    };
}

#[test]
fn test_blog_post_card_with_special_characters() {
    let title = "Test & <Special> \"Characters\"".to_string();
    let description = "Description with 'quotes' and symbols!".to_string();

    let _card = rsx! {
        BlogPostCard {
            title: title,
            description: description,
            index: 1
        }
    };
}

#[test]
fn test_social_link_with_various_icons() {
    let icons = vec!["fa-github", "fa-linkedin", "fa-twitter", "fa-facebook"];

    for icon in icons {
        let _link = rsx! {
            SocialLink {
                href: "https://example.com".to_string(),
                icon: icon.to_string(),
                label: "Test".to_string()
            }
        };
    }
}
