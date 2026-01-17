use crate::components::{AboutSection, IntroductionSection};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "w-full",
            IntroductionSection {}
            AboutSection {}
        }
    }
}
