use crate::components::{ProfileImage, SocialLink};
use dioxus::prelude::*;

#[component]
pub fn IntroductionSection() -> Element {
    rsx! {
        div { class: "flex items-center min-h-[90vh] px-12 py-20",
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-16 items-center max-w-6xl mx-auto",
                div {
                    p { class: "text-blue-500 font-bold tracking-widest uppercase mb-4 text-sm",
                        "Fullstack Developer & AI Engineer"
                    }
                    h1 { class: "text-7xl font-black mb-8 leading-tight",
                        span { class: "block", "Taian" }
                        span { class: "block text-white/90", "Feitosa" }
                    }
                    p { class: "text-xl text-gray-400 mb-12 max-w-lg leading-relaxed",
                        "Building bridges between logic and creativity through code, "
                        "community, and innovative AI solutions."
                    }

                    div { class: "flex gap-8 items-center",
                        SocialLink {
                            href: "https://github.com/taianf".to_string(),
                            icon: "fa-github".to_string(),
                            label: "Github".to_string()
                        }
                        SocialLink {
                            href: "https://www.linkedin.com/in/taian-feitosa/".to_string(),
                            icon: "fa-linkedin".to_string(),
                            label: "LinkedIn".to_string()
                        }
                    }
                }

                ProfileImage {
                    src: "https://media.licdn.com/dms/image/v2/D4D03AQFvFMptVJnakQ/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1673901553883?e=1770249600&v=beta&t=KYANNIIT41GRnj54wCutvAhjrRTF3JXdKGNU1OpymSc".to_string(),
                    alt: "Taian Feitosa".to_string()
                }
            }
        }
    }
}
