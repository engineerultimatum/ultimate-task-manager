use dioxus::prelude::*;
use crate::data::SaveManager;
use crate::models::Route;
use crate::presentation::components::LanguageToggle;

#[component]
pub fn Options() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let language: Signal<String> = use_context();
    let mut current_route: Signal<Route> = use_context();

    // Auto-save language whenever it changes
    use_effect(move || {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(mut save) = SaveManager::load_save(seed) {
                save.language = language.read().clone();
                SaveManager::save_data(&save);
            }
        }
    });

    rsx! {
        style {
            "
            body {{
                background-color: black;
                margin: 0;
            }}
            @keyframes unblur {{
                from {{ filter: blur(20px); opacity: 0; }}
                to   {{ filter: blur(0px); opacity: 1; }}
            }}
            .unblur-text {{
                animation: unblur 3s ease-out forwards;
            }}
            "
        }

        div { class: "flex flex-col h-screen bg-black text-white",

            // ── top bar ──
            div { class: "flex items-center justify-center py-8 bg-black",
                h1 { class: "unblur-text text-4xl font-bold text-blue-200",
                    if language() == "English" {
                        h1 { "Options" }
                    } else {
                        h1 { "Seçenekler" }
                    }
                }

                h2 {
                    class: "absolute right-8 top-8 text-gray-400 hover:text-gray-200 cursor-pointer",
                    onclick: move |_| {
                        current_route.set(Route::Home);
                    },
                    if language() == "English" {
                        h1 { "Back" }
                    } else {
                        h1 { "Geri" }
                    }
                }
            }
            // ── content ──
            div { class: "flex flex-col px-8 py-6 gap-6",

                // language row
                div { class: "flex items-center justify-between bg-zinc-900 px-6 py-4 rounded-lg",

                    p { class: "text-lg text-gray-300",
                        if language() == "English" {
                            h1 { "Language" }
                        } else {
                            h1 { "Dil" }
                        }
                    }

                    LanguageToggle { language }
                }
            }
        }
    }
}
