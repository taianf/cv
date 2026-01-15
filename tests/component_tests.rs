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

// Enhanced edge case tests

#[test]
fn test_blog_post_card_with_empty_strings() {
    let _card = rsx! {
        BlogPostCard {
            title: String::new(),
            description: String::new(),
            index: 0
        }
    };
}

#[test]
fn test_blog_post_card_with_very_long_strings() {
    let long_title = "a".repeat(1000);
    let long_description = "b".repeat(2000);

    let _card = rsx! {
        BlogPostCard {
            title: long_title,
            description: long_description,
            index: 999999
        }
    };
}

#[test]
fn test_blog_post_card_with_boundary_indices() {
    let boundary_indices = vec![i32::MIN, -1, 0, 1, i32::MAX];

    for index in boundary_indices {
        let _card = rsx! {
            BlogPostCard {
                title: "Boundary Test".to_string(),
                description: "Testing boundary values".to_string(),
                index: index
            }
        };
    }
}

#[test]
fn test_blog_post_card_with_unicode_content() {
    let unicode_title = "测试标题 🚀 Título".to_string();
    let unicode_description = "测试描述 📝 Descripción".to_string();

    let _card = rsx! {
        BlogPostCard {
            title: unicode_title,
            description: unicode_description,
            index: 42
        }
    };
}

#[test]
fn test_blog_post_card_with_whitespace_content() {
    let whitespace_cases = vec![
        " ",        // Single space
        "   ",      // Multiple spaces
        "\t",       // Tab
        "\n",       // Newline
        "\r",       // Carriage return
        " \t\n\r ", // Mixed whitespace
    ];

    for whitespace in whitespace_cases {
        let _card = rsx! {
            BlogPostCard {
                title: whitespace.to_string(),
                description: whitespace.to_string(),
                index: 1
            }
        };
    }
}

#[test]
fn test_section_card_with_empty_props() {
    let _card = rsx! {
        SectionCard {
            title: String::new(),
            icon: String::new(),
            div { "Empty props test" }
        }
    };
}

#[test]
fn test_section_card_with_long_props() {
    let long_title = "a".repeat(500);
    let long_icon = "fa-".to_string() + &"b".repeat(100);

    let _card = rsx! {
        SectionCard {
            title: long_title,
            icon: long_icon,
            div { "Long props test" }
        }
    };
}

#[test]
fn test_section_card_with_special_icon_names() {
    let special_icons = vec![
        "fa-test",
        "fas fa-test",
        "fab fa-test",
        "far fa-test",
        "fal fa-test",
        "fa-test-with-dashes",
        "fa_test_with_underscores",
        "fa123test456",
        "fa-test@#$%",
    ];

    for icon in special_icons {
        let _card = rsx! {
            SectionCard {
                title: "Special Icon Test".to_string(),
                icon: icon.to_string(),
                div { "Testing special icon: {icon}" }
            }
        };
    }
}

#[test]
fn test_section_card_with_nested_children() {
    let _card = rsx! {
        SectionCard {
            title: "Nested Children".to_string(),
            icon: "fa-nested".to_string(),
            div {
                h3 { "Nested heading" }
                p { "Nested paragraph" }
                ul {
                    li { "Item 1" }
                    li { "Item 2" }
                    li { "Item 3" }
                }
                div {
                    span { "Nested span" }
                    a { href: "#", "Nested link" }
                }
            }
        }
    };
}

#[test]
fn test_social_link_with_empty_props() {
    let _link = rsx! {
        SocialLink {
            href: String::new(),
            icon: String::new(),
            label: String::new()
        }
    };
}

#[test]
fn test_social_link_with_invalid_urls() {
    let invalid_urls = vec![
        "not-a-url",
        "http://",
        "https://",
        "ftp://example.com",
        "javascript:alert('xss')",
        "data:text/html,<script>alert('xss')</script>",
        "//example.com",
        "example.com", // Missing protocol
    ];

    for url in invalid_urls {
        let _link = rsx! {
            SocialLink {
                href: url.to_string(),
                icon: "fa-test".to_string(),
                label: "Invalid URL Test".to_string()
            }
        };
    }
}

#[test]
fn test_social_link_with_very_long_props() {
    let long_href = "https://example.com/".to_string() + &"path/".repeat(100);
    let long_icon = "fa-".to_string() + &"test-".repeat(50);
    let long_label = "a".repeat(200);

    let _link = rsx! {
        SocialLink {
            href: long_href,
            icon: long_icon,
            label: long_label
        }
    };
}

#[test]
fn test_social_link_with_unicode_content() {
    let unicode_cases = vec![
        ("https://测试.com", "fa-测试", "测试链接"),
        ("https://tést.com", "fa-tést", "Tést Link"),
        ("https://🚀.com", "fa-🚀", "🚀 Link"),
    ];

    for (href, icon, label) in unicode_cases {
        let _link = rsx! {
            SocialLink {
                href: href.to_string(),
                icon: icon.to_string(),
                label: label.to_string()
            }
        };
    }
}

#[test]
fn test_social_link_with_various_protocols() {
    let protocol_cases = vec![
        ("https://secure.example.com", "fa-https", "Secure Link"),
        ("http://insecure.example.com", "fa-http", "Insecure Link"),
        ("mailto:test@example.com", "fa-envelope", "Email Link"),
        ("tel:+1234567890", "fa-phone", "Phone Link"),
        ("ftp://files.example.com", "fa-ftp", "FTP Link"),
    ];

    for (href, icon, label) in protocol_cases {
        let _link = rsx! {
            SocialLink {
                href: href.to_string(),
                icon: icon.to_string(),
                label: label.to_string()
            }
        };
    }
}

#[test]
fn test_component_composition() {
    // Test that components can be composed together
    let _composed = rsx! {
        div {
            SectionCard {
                title: "Component Composition".to_string(),
                icon: "fa-compose".to_string(),
                div {
                    BlogPostCard {
                        title: "Nested Blog Card".to_string(),
                        description: "This is nested inside a section card".to_string(),
                        index: 1
                    }
                    SocialLink {
                        href: "https://example.com".to_string(),
                        icon: "fa-nested".to_string(),
                        label: "Nested Social Link".to_string()
                    }
                }
            }
        }
    };
}

#[test]
fn test_multiple_component_instances() {
    // Test creating multiple instances of each component
    let _multiple = rsx! {
        div {
            // Multiple BlogPostCard instances
            for i in 0..5 {
                BlogPostCard {
                    title: format!("Post {}", i),
                    description: format!("Description for post {}", i),
                    index: i
                }
            }

            // Multiple SectionCard instances
            for i in 0..3 {
                SectionCard {
                    title: format!("Section {}", i),
                    icon: format!("fa-section-{}", i),
                    div { "Content for section {i}" }
                }
            }

            // Multiple SocialLink instances
            for i in 0..4 {
                SocialLink {
                    href: format!("https://example{}.com", i),
                    icon: format!("fa-link-{}", i),
                    label: format!("Link {}", i)
                }
            }
        }
    };
}

#[test]
fn test_component_with_control_characters() {
    let control_chars = vec![
        "\0",       // Null
        "\x01",     // Start of heading
        "\x1F",     // Unit separator
        "\x7F",     // Delete
        "\u{200B}", // Zero-width space
        "\u{2060}", // Word joiner
    ];

    for control_char in control_chars {
        let _components = rsx! {
            div {
                BlogPostCard {
                    title: format!("Title{}", control_char),
                    description: format!("Description{}", control_char),
                    index: 1
                }
                SectionCard {
                    title: format!("Section{}", control_char),
                    icon: format!("fa{}", control_char),
                    div { "Content{control_char}" }
                }
                SocialLink {
                    href: format!("https://example{}.com", control_char),
                    icon: format!("fa{}", control_char),
                    label: format!("Link{}", control_char)
                }
            }
        };
    }
}

#[test]
fn test_component_performance_with_large_content() {
    // Test that components handle large content efficiently
    let large_content = "x".repeat(10000);

    let _large_components = rsx! {
        div {
            BlogPostCard {
                title: large_content.clone(),
                description: large_content.clone(),
                index: 1
            }
            SectionCard {
                title: large_content.clone(),
                icon: large_content.clone(),
                div { "{large_content}" }
            }
            SocialLink {
                href: format!("https://{}.com", large_content),
                icon: large_content.clone(),
                label: large_content
            }
        }
    };
}
