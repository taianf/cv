use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "p-8 mb-4",
            h1 { class: "text-2xl font-black tracking-tighter uppercase",
                span { class: "text-white", "Taian Feitosa" }
            }
        }
    }
}
