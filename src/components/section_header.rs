use dioxus::prelude::*;

#[component]
pub fn SectionHeader(title: String) -> Element {
    rsx! {
        h2 { class: "text-3xl font-bold mb-12 flex items-center gap-4",
            span { class: "w-12 h-1 bg-blue-500 rounded-full" }
            "{title}"
        }
    }
}
