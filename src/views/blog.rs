use crate::components::BlogPostCard;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div { class: "p-8",
            h1 { class: "text-4xl font-bold mb-8 text-blue-500", "Blog" }
            div { class: "grid gap-6",
                for i in 1..4 {
                    BlogPostCard {
                        title: format!("Latest Insight #{i}"),
                        description: "Exploring the depths of technology and creativity in this latest post.".to_string(),
                        index: i
                    }
                }
            }
        }
    }
}
