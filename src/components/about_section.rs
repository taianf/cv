use crate::components::SectionHeader;
use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    rsx! {
        div { id: "about", class: "px-12 py-20",
            div { class: "max-w-4xl mx-auto",
                SectionHeader { title: "About me".to_string() }

                div { class: "prose prose-invert prose-lg text-gray-400 leading-relaxed",
                    p { class: "mb-6",
                        "I am a Fullstack Developer and AI Engineer passionate about building bridges between "
                        "users and technology. With a deep focus on Rust and modern web frameworks like Dioxus, "
                        "I strive to create performant and accessible applications."
                    }
                    p { class: "mb-6",
                        "My journey in tech is driven by curiosity and community. I believe in open source, "
                        "knowledge sharing, and the power of software to solve real-world problems. "
                        "When I'm not coding, I'm exploring new AI models or contributing to the developer ecosystem."
                    }
                    p {
                        "Feel free to reach out via my social links or join our community forum to start a conversation!"
                    }
                }
            }
        }
    }
}
