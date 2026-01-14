use dioxus::prelude::*;

#[component]
pub fn AuthCallback() -> Element {
    let mut auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();
    let navigate = use_navigator();

    use_effect(move || {
        spawn(async move {
            #[cfg(feature = "web")]
            {
                if let Some(window) = web_sys::window() {
                    let url = window.location().href().unwrap_or_default();
                    if let Ok(url_obj) = web_sys::Url::new(&url) {
                        let params = url_obj.search_params();
                        if let Some(code) = params.get("code") {
                            if let Ok(user) = crate::models::exchange_code_for_user(code).await {
                                auth_user.set(Some(user.clone()));
                                if let Ok(Some(storage)) = window.local_storage() {
                                    let _ = storage.set_item("auth_email", &user.email);
                                }
                                navigate.push(crate::Route::Profile {});
                                return;
                            }
                        }
                    }
                }
            }
            navigate.push(crate::Route::Home {});
        });
    });

    rsx! {
        div { class: "flex items-center justify-center min-h-screen bg-gray-900",
            div { class: "text-center",
                div { class: "animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-blue-500 mx-auto mb-4" }
                p { class: "text-xl text-gray-300", "Authenticating with Google..." }
            }
        }
    }
}
