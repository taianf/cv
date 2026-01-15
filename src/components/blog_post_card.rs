use dioxus::prelude::*;

#[component]
pub fn BlogPostCard(title: String, description: String, index: i32) -> Element {
    rsx! {
        div {
            class: "bg-gray-800 p-6 rounded-xl border border-gray-700 hover:border-blue-500 transition-all cursor-pointer",
            h2 { class: "text-2xl font-semibold mb-2", "{title}" }
            p { class: "text-gray-400 mb-4", "{description}" }
            div { class: "flex items-center gap-2 text-blue-500 font-medium",
                "Read more"
                i { class: "fas fa-arrow-right text-xs" }
            }
        }
    }
}
