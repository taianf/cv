use dioxus::prelude::*;

#[component]
pub fn SocialLinks() -> Element {
    rsx! {
        section { class: "mt-12 text-center",
            h3 { class: "text-xl font-semibold text-gray-300 mb-6", "Other Platforms" }
            div { class: "flex justify-center gap-6",
                SocialLink { href: "#", icon: "fab fa-twitter", label: "Twitter" }
                SocialLink { href: "#", icon: "fas fa-globe", label: "Portfolio" }
                SocialLink { href: "#", icon: "fas fa-file-pdf", label: "Resume" }
            }
        }
    }
}

#[component]
fn SocialLink(href: String, icon: String, label: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "text-gray-400 hover:text-white transition-colors flex items-center gap-2",
            i { class: "{icon}" }
            "{label}"
        }
    }
}
