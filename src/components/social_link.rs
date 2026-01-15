use dioxus::prelude::*;

#[component]
pub fn SocialLink(href: String, icon: String, label: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
            i { class: "fab {icon} text-xl" }
            "{label}"
        }
    }
}
