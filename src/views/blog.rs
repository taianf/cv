use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div { class: "p-8",
            h1 { class: "text-4xl font-bold mb-8 text-blue-500", "Blog" }
            div { class: "grid gap-6",
                for i in 1..4 {
                    div { class: "bg-gray-800 p-6 rounded-xl border border-gray-700 hover:border-blue-500 transition-all cursor-pointer",
                        h2 { class: "text-2xl font-semibold mb-2", "Latest Insight #{i}" }
                        p { class: "text-gray-400 mb-4", "Exploring the depths of technology and creativity in this latest post." }
                        div { class: "flex items-center gap-2 text-blue-500 font-medium",
                            "Read more"
                            i { class: "fas fa-arrow-right text-xs" }
                        }
                    }
                }
            }
        }
    }
}
