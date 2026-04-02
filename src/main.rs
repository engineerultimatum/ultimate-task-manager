mod models;
mod business_logic;
mod data;
mod presentation;
mod constants;

use dioxus::prelude::*;
use models::Route;
use presentation::{Login, Home, Options, Go, Calendar};
use constants::TAILWIND_CSS;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let seed_signal: Signal<Option<String>> = use_signal(|| None);
    let language_signal: Signal<String> = use_signal(|| "English".to_string());
    let mut current_route = use_signal(|| Route::Login);
    
    use_context_provider(|| language_signal);
    use_context_provider(|| seed_signal);
    use_context_provider(|| current_route);
    
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        match current_route() {
            Route::Login => rsx! {
                Login {}
            },
            Route::Home => rsx! {
                Home {}
            },
            Route::Options => rsx! {
                Options {}
            },
            Route::Tasks => rsx! {
                Go {}
            },
            Route::Calendar => rsx! {
                Calendar {}
            },
        }
    }
}
