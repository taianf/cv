use dioxus::prelude::*;

#[component]
pub fn LinkedInCard() -> Element {
    rsx! {
        div { class: "bg-gray-800 p-6 rounded-2xl shadow-xl border border-gray-700 hover:border-blue-500 transition-colors",
            h2 { class: "text-2xl font-bold text-white mb-4 flex items-center gap-2",
                i { class: "fab fa-linkedin" }
                "LinkedIn"
            }
            div { class: "space-y-4",
                p { class: "text-gray-300", "Software Engineer at Google" }
                div { class: "flex items-center gap-2 text-green-400 text-sm",
                    span { class: "relative flex h-3 w-3",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75" }
                        span { class: "relative inline-flex rounded-full h-3 w-3 bg-green-500" }
                    }
                    "Available for collaboration"
                }
                a {
                    href: "https://www.linkedin.com/in/taian-feitosa/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-block w-full text-center bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                    "Let's Connect"
                }
            }
        }
    }
}
