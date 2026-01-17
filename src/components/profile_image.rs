use dioxus::prelude::*;

#[component]
pub fn ProfileImage(src: String, alt: String) -> Element {
    rsx! {
        div { class: "relative group",
            div { class: "absolute -inset-4 bg-blue-500/10 rounded-[40px] border border-blue-500/20 transform rotate-3 transition-transform group-hover:rotate-0" }
            div { class: "relative aspect-square rounded-[32px] overflow-hidden bg-blue-600 shadow-2xl transition-transform group-hover:scale-[1.02] duration-500",
                img {
                    src: "{src}",
                    class: "w-full h-full object-cover",
                    alt: "{alt}",
                }
            }
        }
    }
}
