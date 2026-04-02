use dioxus::prelude::*;

#[component]
pub fn CalendarDay(day: i32, is_current_month: bool, on_double_click: EventHandler<()>) -> Element {
    let day_class = if is_current_month {
        "h-24 bg-gray-800 rounded cursor-pointer hover:bg-gray-700 flex items-center justify-center text-lg font-bold"
    } else {
        "h-24 bg-gray-800 rounded cursor-pointer hover:bg-gray-700 flex items-center justify-center text-lg font-bold text-gray-500 opacity-40"
    };
    
    rsx! {
        div {
            class: day_class,
            ondoubleclick: move |_| on_double_click.call(()),
            "{day}"
        }
    }
}
