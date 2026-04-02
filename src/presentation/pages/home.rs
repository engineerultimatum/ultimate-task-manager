use dioxus::prelude::*;
use crate::models::Route;

#[component]
pub fn Home() -> Element {
    let language: Signal<String> = use_context();
    let mut current_route: Signal<Route> = use_context();
    
    rsx! {
        style {
            "
            body {{
                background-color: black;
                margin: 0;
            }}
            @keyframes slide {{
                from {{ transform: translateX(100vw); }}
                to   {{ transform: translateX(-100%); }}
            }}
            .sliding-text {{
                animation: slide 90s linear infinite;
                white-space: nowrap;
                display: inline-block;
            }}
            @keyframes unblur {{
                from {{ filter: blur(20px); opacity: 0; }}
                to   {{ filter: blur(0px); opacity: 1; }}
            }}
            .unblur-text {{
                animation: unblur 5s ease-out forwards;
            }}
            "
        }

        div { class: "flex flex-col h-screen",

            // ── top section (black) ──
            div { class: "flex items-center justify-center py-8 bg-black",
                h1 { class: "unblur-text text-4xl font-bold text-blue-200",
                    if language() == "English" {
                        h1 { "Username" }
                    } else {
                        h1 { "Kullanıcı adı" }
                    }
                }
            }

            //── divider line ──
            div { class: "w-full h-px bg-gray-600" }

            // ── middle section (gray) ──
            div { class: "flex flex-1 items-center justify-center gap-6 bg-red-900",

                button {
                    class: "bg-zinc-800 text-white px-8 py-4 rounded-lg text-xl hover:bg-zinc-700",
                    onclick: move |_| {
                        current_route.set(Route::Options);
                    },
                    if language() == "English" {
                        h1 { "Options" }
                    } else {
                        h1 { "Seçenekler" }
                    }
                }
                button {
                    class: "bg-blue-600 text-white px-8 py-4 rounded-lg text-xl hover:bg-blue-500",
                    onclick: move |_| {
                        current_route.set(Route::Tasks);
                    },
                    if language() == "English" {
                        h1 { "My Tasks" }
                    } else {
                        h1 { "Görevlerim" }
                    }
                }
                button {
                    class: "bg-purple-600 text-white px-8 py-4 rounded-lg text-xl hover:bg-purple-500",
                    onclick: move |_| {
                        current_route.set(Route::Calendar);
                    },
                    if language() == "English" {
                        h1 { "Calendar" }
                    } else {
                        h1 { "Takvim" }
                    }
                }
            }

            // ── sliding text (bottom) ──
            div { class: "overflow-hidden pb-8 bg-red-900",
                p { class: "sliding-text text-8xl font-bold text-blue-300",
                    "Welcome to the Ultimate Task Manager ----- Welcome to the Ultimate Task Manager ----- Welcome to the Ultimate Task Manager ----- Welcome to the Ultimate Task Manager"
                }
            }
        }
    }
}
