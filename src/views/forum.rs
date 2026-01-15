use dioxus::prelude::*;

#[component]
pub fn Forum() -> Element {
    rsx! {
        div { class: "p-8",
            div { class: "flex justify-between items-center mb-8",
                h1 { class: "text-4xl font-bold text-blue-500", "Community Forum" }
                button { class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg", "New Topic" }
            }

            div { class: "space-y-4",
                for topic in ["Rust performance tips", "Designing for accessibility", "Dioxus 0.7 new features", "WebAssembly future"] {
                    div { class: "bg-gray-800 p-4 rounded-lg border border-gray-700 flex items-center justify-between hover:border-gray-500 transition-colors cursor-pointer",
                        div {
                            h3 { class: "text-lg font-semibold", "{topic}" }
                            div { class: "flex gap-4 text-sm text-gray-500",
                                span { i { class: "fas fa-user mr-1" } "Alice" }
                                span { i { class: "fas fa-clock mr-1" } "2h ago" }
                                span { i { class: "fas fa-comment mr-1" } "15 replies" }
                            }
                        }
                        div { class: "flex -space-x-2",
                            for _ in 0..3 {
                                div { class: "w-8 h-8 rounded-full border-2 border-gray-800 bg-gray-600" }
                            }
                        }
                    }
                }
            }
        }
    }
}
