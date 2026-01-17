use dioxus::prelude::*;

#[component]
pub fn LoginButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-xl flex items-center justify-center gap-2 transition-all shadow-lg shadow-blue-500/10",
            onclick: onclick,
            i { class: "fas fa-sign-in-alt" }
            "Login"
        }
    }
}
