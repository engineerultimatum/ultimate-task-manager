use dioxus::prelude::*;
use crate::data::SaveManager;
use crate::models::Route;
use crate::business_logic::{
    get_days_in_month, get_first_day_of_week, get_current_month,
    format_month, create_deadline_timestamp, generate_calendar_days,
    TodoNodeFactory,
};
use crate::presentation::components::{CalendarDay, DayModal};

#[component]
pub fn Calendar() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let language_signal: Signal<String> = use_context();
    let mut current_route: Signal<Route> = use_context();
    
    let mut current_month = use_signal(get_current_month);
    
    let mut calendar_modal_open = use_signal(|| false);
    let mut selected_day = use_signal(|| 0);
    let mut todos = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = SaveManager::load_save(seed) {
                return save.todos;
            }
        }
        vec![]
    });
    
    let mut next_id = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = SaveManager::load_save(seed) {
                return save.next_id;
            }
        }
        0
    });
    
    let (year, month) = current_month();
    let days_in_month = get_days_in_month(month, year);
    let first_day_of_week = get_first_day_of_week(month, year);
    
    rsx! {
        div { class: "flex flex-col h-screen bg-black text-white",
            div { class: "flex items-center justify-between py-8 px-8 bg-black",
                h1 { class: "text-3xl font-bold text-blue-200", "Calendar" }
                h2 {
                    class: "text-gray-400 hover:text-gray-200 cursor-pointer",
                    onclick: move |_| {
                        current_route.set(Route::Home);
                    },
                    if language_signal() == "English" {
                        "Back"
                    } else {
                        "Geri"
                    }
                }
            }

            div { class: "flex-1 flex flex-col items-center justify-center px-8",
                div { class: "w-full max-w-4xl",
                    // Month/Year header with navigation
                    div { class: "flex items-center justify-between mb-8",
                        button {
                            class: "text-3xl text-blue-400 hover:text-blue-300",
                            onclick: move |_| {
                                let (y, m) = current_month();
                                if m == 0 {
                                    current_month.set((y - 1, 11));
                                } else {
                                    current_month.set((y, m - 1));
                                }
                            },
                            "◀"
                        }
                        div { class: "text-2xl font-bold text-center flex-1",
                            "{format_month(month)} {year}"
                        }
                        button {
                            class: "text-3xl text-blue-400 hover:text-blue-300",
                            onclick: move |_| {
                                let (y, m) = current_month();
                                if m == 11 {
                                    current_month.set((y + 1, 0));
                                } else {
                                    current_month.set((y, m + 1));
                                }
                            },
                            "▶"
                        }
                    }

                    // Calendar grid
                    div { class: "bg-gray-900 p-6 rounded-lg",
                        // Day labels
                        div { class: "grid grid-cols-7 gap-2 mb-4",
                            for day_label in &["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
                                div { class: "text-center font-bold text-gray-400 text-sm h-10 flex items-center justify-center",
                                    "{day_label}"
                                }
                            }
                        }

                        // Calendar days (6x7 grid)
                        div { class: "grid grid-cols-7 gap-2",
                            for day_info in generate_calendar_days(month, year, first_day_of_week, days_in_month) {
                                CalendarDay {
                                    day: day_info.0,
                                    is_current_month: day_info.1,
                                    on_double_click: move |_| {
                                        if day_info.1 {
                                            selected_day.set(day_info.0);
                                            calendar_modal_open.set(true);
                                        }
                                    },
                                }
                            }
                        }
                    }

                    if calendar_modal_open() {
                        DayModal {
                            day: selected_day(),
                            month: month as u32,
                            year: year as u32,
                            todos,
                            next_id,
                            seed_signal,
                            language_signal,
                            on_create: move |(task_text, _importance): (String, u32)| {
                                let deadline_timestamp = create_deadline_timestamp(year, month, selected_day());
                                todos
                                    .with_mut(|t| {
                                        t.push(
                                            TodoNodeFactory::create_deadline(
                                                next_id(),
                                                task_text.clone(),
                                                deadline_timestamp,
                                            ),
                                        );
                                    });
                                next_id += 1;
                                if let Some(seed) = seed_signal.read().as_ref() {
                                    let save = crate::models::SaveData {
                                        seed: seed.clone(),
                                        language: language_signal.read().clone(),
                                        todos: todos.read().clone(),
                                        next_id: next_id.read().clone(),
                                        points: 0,
                                    };
                                    SaveManager::save_data(&save);
                                }
                                calendar_modal_open.set(false);
                            },
                            on_cancel: move |_| {
                                calendar_modal_open.set(false);
                            },
                        }
                    }
                }
            }
        }
    }
}
