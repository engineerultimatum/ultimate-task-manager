use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveData {
    seed: String,
    language: String,
    todos: Vec<TodoNode>,
    next_id: usize,
    points: u32,
}

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Login {},
    #[route("/home")]
    Home {},
    #[route("/home/Options")]
    Options{},
    #[route("/home/Go")]
    Go{},
    #[route("/home/Calendar")]
    Calendar{},
}
 
#[component]
fn App() -> Element {
    let seed_signal: Signal<Option<String>> = use_signal(|| None);
    let language_signal: Signal<String> = use_signal(|| "English".to_string());
    
    use_context_provider(|| language_signal);
    use_context_provider(|| seed_signal);
    
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Login() -> Element {
    let mut text = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut seed_signal: Signal<Option<String>> = use_context();
    let mut language_signal: Signal<String> = use_context();
    let nav = use_navigator(); 

    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen bg-gray-800 gap-4",

            h1 { class: "text-4xl font-bold text-blue-200", "Enter Your Seed" }

            input {
                class: "border border-gray-300 rounded px-4 py-2",
                placeholder: "Type any seed...",
                value: "{text}",
                oninput: move |e| text.set(e.value()),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        let seed = text();
                        if !seed.is_empty() {
                            let save_data = load_save(&seed);

                            match save_data {
                                Some(data) => {
                                    seed_signal.set(Some(seed.clone()));
                                    language_signal.set(data.language);
                                }
                                None => {
                                    seed_signal.set(Some(seed.clone()));
                                    let new_save = SaveData {
                                        seed: seed.clone(),
                                        language: "English".to_string(),
                                        todos: vec![
                                            TodoNode {
                                                id: 0,
                                                text: "Learn Rust".to_string(),
                                                completed: false,
                                                importance: 1,
                                                children: vec![
                                                    TodoNode {
                                                        id: 1,
                                                        text: "New Subtask".to_string(),
                                                        completed: false,
                                                        importance: 1,
                                                        children: vec![],
                                                        deadline: None,
                                                    },
                                                    TodoNode {
                                                        id: 2,
                                                        text: "New Subtask".to_string(),
                                                        completed: false,
                                                        importance: 1,
                                                        children: vec![],
                                                        deadline: None,
                                                    },
                                                ],
                                                deadline: None,
                                            },
                                        ],
                                        next_id: 3,
                                        points: 0,
                                    };
                                    save_data_to_storage(&new_save);
                                    language_signal.set("English".to_string());
                                }
                            }
                            nav.push(Route::Home {});
                        } else {
                            message.set("❌ Please enter a seed.".to_string());
                        }
                    }
                },
            }

            button {
                class: "mt-4 bg-blue-500 text-white px-6 py-2 rounded hover:bg-blue-600",
                onclick: move |_: MouseEvent| {
                    let seed = text();
                    if !seed.is_empty() {
                        let save_data = load_save(&seed);

                        match save_data {
                            Some(data) => {
                                seed_signal.set(Some(seed.clone()));
                                language_signal.set(data.language);
                            }
                            None => {
                                seed_signal.set(Some(seed.clone()));
                                let new_save = SaveData {
                                    seed: seed.clone(),
                                    language: "English".to_string(),
                                    todos: vec![
                                        TodoNode {
                                            id: 0,
                                            text: "Learn Rust".to_string(),
                                            completed: false,
                                            importance: 1,
                                            children: vec![
                                                TodoNode {
                                                    id: 1,
                                                    text: "New Subtask".to_string(),
                                                    completed: false,
                                                    importance: 1,
                                                    children: vec![],
                                                    deadline: None,
                                                },
                                                TodoNode {
                                                    id: 2,
                                                    text: "New Subtask".to_string(),
                                                    completed: false,
                                                    importance: 1,
                                                    children: vec![],
                                                    deadline: None,
                                                },
                                            ],
                                            deadline: None,
                                        },
                                    ],
                                    next_id: 3,
                                    points: 0,
                                };
                                save_data_to_storage(&new_save);
                                language_signal.set("English".to_string());
                            }
                        }
                        nav.push(Route::Home {});
                    } else {
                        message.set("❌ Please enter a seed.".to_string());
                    }
                },
                "Enter"
            }

            p { class: "text-white", "{message}" }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn load_save(seed: &str) -> Option<SaveData> {
    use web_sys::window;
    
    let storage = window()?.local_storage().ok()??;
    let key = format!("save_{}", seed);
    
    match storage.get_item(&key).ok()? {
        Some(data) => serde_json::from_str(&data).ok(),
        None => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_save(seed: &str) -> Option<SaveData> {
    use std::fs;
    
    let save_path = get_save_path(seed)?;
    
    if !save_path.exists() {
        return None;
    }
    
    match fs::read_to_string(&save_path) {
        Ok(content) => serde_json::from_str(&content).ok(),
        Err(_) => None,
    }
}

#[cfg(target_arch = "wasm32")]
fn save_data_to_storage(data: &SaveData) {
    use web_sys::window;
    
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let key = format!("save_{}", data.seed);
            if let Ok(json) = serde_json::to_string(data) {
                let _ = storage.set_item(&key, &json);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_data_to_storage(data: &SaveData) {
    use std::fs;
    
    if let Some(save_path) = get_save_path(&data.seed) {
        // Create directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Write save file
        if let Ok(json) = serde_json::to_string_pretty(data) {
            let _ = fs::write(&save_path, json);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_save_path(seed: &str) -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let save_file = format!("{}.json", seed.replace("/", "_").replace("\\", "_"));
    
    Some(config_dir.join("hot_dog").join("saves").join(save_file))
}

#[component]
fn Home() -> Element {
    let language: Signal<String> = use_context();
    let nav = use_navigator();  
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

            // ── divider line ──
            div { class: "w-full h-px bg-gray-600" }

            // ── middle section (gray) ──
            div { class: "flex flex-1 items-center justify-center gap-6 bg-red-900",

                button {
                    class: "bg-zinc-800 text-white px-8 py-4 rounded-lg text-xl hover:bg-zinc-700",
                    onclick: move |_| {
                        nav.push(Route::Options {}); // ← go to home page
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
                        nav.push(Route::Go {});
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
                        nav.push(Route::Calendar {});
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

#[component]
fn Options() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let mut language: Signal<String> = use_context();
    let nav = use_navigator();

    // Auto-save language whenever it changes
    use_effect(move || {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(mut save) = load_save(seed) {
                save.language = language.read().clone();
                save_data_to_storage(&save);
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
                        // navigate back to home
                        nav.go_back();
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

                    // toggle between two languages
                    div { class: "flex gap-2",

                        button {
                            class: if language() == "English" { "px-4 py-2 rounded bg-blue-600 text-white" } else { "px-4 py-2 rounded bg-zinc-700 text-gray-400 hover:bg-zinc-600" },
                            onclick: move |_| language.set("English".to_string()),
                            if language() == "English" {
                                h1 { "English" }
                            } else {
                                h1 { "İngilizce" }
                            }
                        }

                        button {
                            class: if language() == "Turkish" { "px-4 py-2 rounded bg-blue-600 text-white" } else { "px-4 py-2 rounded bg-zinc-700 text-gray-400 hover:bg-zinc-600" },
                            onclick: move |_| language.set("Turkish".to_string()),
                            if language() == "English" {
                                h1 { "Turkish" }
                            } else {
                                h1 { "Türkçe" }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct TodoNode {
    id: usize,
    text: String,
    completed: bool,
    importance: u32, // 1 = green, 2 = red, 3 = purple
    children: Vec<TodoNode>,
    deadline: Option<u64>, // Unix timestamp in seconds, None for regular tasks
}
#[component]
pub fn Go() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let language_signal: Signal<String> = use_context();
    let nav = use_navigator();
    
    // Load initial data from save
    let mut todos = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = load_save(seed) {
                return save.todos;
            }
        }
        vec![
            TodoNode {
                id: 0,
                text: "Learn Rust".to_string(),
                completed: false,
                importance: 1,
                children: vec![
                    TodoNode { id: 1, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![], deadline: None },
                    TodoNode { id: 2, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![], deadline: None },
                ],
                deadline: None,
            },
        ]
    });
    
    let mut next_id = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = load_save(seed) {
                return save.next_id;
            }
        }
        3
    });
    
    let points = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = load_save(seed) {
                return save.points;
            }
        }
        0u32
    });

    // Auto-save whenever todos or points change
    use_effect(move || {
        if let Some(seed) = seed_signal.read().as_ref() {
            let save = SaveData {
                seed: seed.clone(),
                language: language_signal.read().clone(),
                todos: todos.read().clone(),
                next_id: next_id.read().clone(),
                points: points.read().clone(),
            };
            save_data_to_storage(&save);
        }
    });

    rsx! {
        div { class: "container mx-auto p-8 bg-black text-white min-h-screen",
            div { class: "relative flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold", "UTiM" }
                div { class: "text-2xl font-bold text-yellow-400", "Points: {points}" }
                h2 {
                    class: "absolute right-8 top-0 text-gray-400 hover:text-gray-200 cursor-pointer",
                    onclick: move |_| {
                        nav.push(Route::Home {});
                    },
                    if language_signal() == "English" {
                        "Back"
                    } else {
                        "Geri"
                    }
                }
            }
            button {
                class: "text-lg text-green-500 mb-4 px-4 py-2 bg-green-700 rounded hover:bg-green-600",
                onclick: move |_| {
                    add_root_node(&mut todos, next_id());
                    next_id += 1;
                },
                "+ Add Root Task"
            }

            ul { class: "space-y-2",
                for node in todos.read().iter() {
                    TreeNode {
                        node: node.clone(),
                        todos,
                        next_id,
                        points,
                    }
                }
            }
        }
    }
}

#[component]
fn TreeNode(node: TodoNode, todos: Signal<Vec<TodoNode>>, mut next_id: Signal<usize>, mut points: Signal<u32>) -> Element {
    let mut expanded = use_signal(|| true);
    let mut edit_modal_open = use_signal(|| false);
    let mut completion_modal_open = use_signal(|| false);
    let mut edit_text = use_signal(|| node.text.clone());

    rsx! {
        li { class: "ml-4",
            div {
                class: "flex items-center gap-2 p-2 hover:bg-blue-900 rounded cursor-pointer",
                ondoubleclick: move |_| {
                    edit_modal_open.set(true);
                    edit_text.set(node.text.clone());
                },
                input {
                    r#type: "checkbox",
                    checked: node.completed,
                    onchange: move |_| {
                        completion_modal_open.set(true);
                    },
                }
                span {
                    class: format!(
                        "{}",
                        if node.completed {
                            "line-through text-gray-400"
                        } else {
                            match node.importance {
                                1 => "text-green-400",
                                2 => "text-red-400",
                                3 => "text-purple-400",
                                _ => "text-white",
                            }
                        },
                    ),
                    "{node.text}"
                }
                if let Some(deadline) = node.deadline {
                    div { class: "text-sm text-yellow-400 ml-2", "{format_time_remaining(deadline)}" }
                }
                if !node.children.is_empty() {
                    button {
                        class: "text-sm text-blue-500",
                        onclick: move |_| expanded.toggle(),
                        if expanded() {
                            "▼"
                        } else {
                            "▶"
                        }
                    }
                }
                button {
                    class: "text-sm text-green-500 ml-auto",
                    onclick: move |_| {
                        add_child(&mut todos, node.id, next_id());
                        next_id += 1;
                    },
                    "+"
                }
                button {
                    class: "text-sm text-red-500",
                    onclick: move |_| {
                        delete_node(&mut todos, node.id);
                    },
                    "✕"
                }
            }
            if expanded() && !node.children.is_empty() {
                ul { class: "ml-4 border-l-2 border-gray-300 pl-2",
                    for child in node.children.iter() {
                        TreeNode {
                            node: child.clone(),
                            todos,
                            next_id,
                            points,
                        }
                    }
                }
            }

            if edit_modal_open() {
                EditModal {
                    initial_text: node.text.clone(),
                    initial_importance: node.importance,
                    on_rename: move |(new_text, importance)| {
                        rename_node(&mut todos, node.id, new_text, importance);
                        edit_modal_open.set(false);
                    },
                    on_cancel: move |_| {
                        edit_modal_open.set(false);
                    },
                }
            }
            if completion_modal_open() {
                CompletionModal {
                    task_name: node.text.clone(),
                    on_completed: move |_| {
                        let importance_points = match node.importance {
                            1 => 1,
                            2 => 2,
                            3 => 3,
                            _ => 1,
                        };
                        *points.write() += importance_points;
                        delete_node(&mut todos, node.id);
                        completion_modal_open.set(false);
                    },
                    on_cancel: move |_| {
                        completion_modal_open.set(false);
                    },
                }
            }
        }
    }
}

#[component]
fn ImportanceSelector(mut importance: Signal<u32>) -> Element {
    rsx! {
        div { class: "mb-4",
            label { class: "block text-white mb-2", "Importance" }
            div { class: "flex gap-2",
                button {
                    class: if importance() == 1 { "px-4 py-2 bg-green-500 text-white rounded font-bold" } else { "px-4 py-2 bg-green-700 text-white rounded hover:bg-green-600" },
                    onclick: move |_| importance.set(1),
                    "🟢 Low"
                }
                button {
                    class: if importance() == 2 { "px-4 py-2 bg-red-500 text-white rounded font-bold" } else { "px-4 py-2 bg-red-700 text-white rounded hover:bg-red-600" },
                    onclick: move |_| importance.set(2),
                    "🔴 Medium"
                }
                button {
                    class: if importance() == 3 { "px-4 py-2 bg-purple-500 text-white rounded font-bold" } else { "px-4 py-2 bg-purple-700 text-white rounded hover:bg-purple-600" },
                    onclick: move |_| importance.set(3),
                    "🟣 High"
                }
            }
        }
    }
}

#[component]
fn EditModal(
    initial_text: String,
    initial_importance: u32,
    on_rename: EventHandler<(String, u32)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut input_value = use_signal(|| initial_text.clone());
    let importance = use_signal(|| initial_importance);

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white", "Rename Task" }
                input {
                    class: "w-full px-4 py-2 border rounded mb-4",
                    value: "{input_value}",
                    oninput: move |e| input_value.set(e.value()),
                    autofocus: true,
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            on_rename.call((input_value(), importance()));
                        }
                    },
                }
                ImportanceSelector { importance }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                        onclick: move |_| on_rename.call((input_value(), importance())),
                        "Rename"
                    }
                }
            }
        }
    }
}

#[component]
fn CompletionModal(
    
    task_name: String,
    on_completed: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white", "Complete Task?" }
                p { class: "text-white mb-6", "Mark '{task_name}' as completed?" }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                        onclick: move |_| on_completed.call(()),
                        "Completed"
                    }
                }
            }
        }
    }
}

#[component]
fn DayModal(
    day: i32,
    month: u32,
    year: u32,
    todos: Signal<Vec<TodoNode>>,
    mut next_id: Signal<usize>,
    seed_signal: Signal<Option<String>>,
    language_signal: Signal<String>,
    on_create: EventHandler<(String, u32)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut task_name = use_signal(|| String::new());
    let mut importance = use_signal(|| 1);

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white",
                    "Add Task for {day} {format_month(month as i32)}"
                }
                input {
                    class: "w-full px-4 py-2 border rounded mb-4 bg-gray-800 text-white border-gray-600",
                    placeholder: "Task name...",
                    value: "{task_name}",
                    oninput: move |e| task_name.set(e.value()),
                    autofocus: true,
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !task_name().is_empty() {
                            on_create.call((task_name(), importance()));
                            task_name.set(String::new());
                        }
                    },
                }
                ImportanceSelector { importance }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| {
                            on_cancel.call(());
                            task_name.set(String::new());
                        },
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                        disabled: task_name().is_empty(),
                        onclick: move |_| {
                            if !task_name().is_empty() {
                                on_create.call((task_name(), importance()));
                                task_name.set(String::new());
                            }
                        },
                        "Create Task"
                    }
                }
            }
        }
    }
}

fn rename_node(todos: &mut Signal<Vec<TodoNode>>, id: usize, new_text: String, importance: u32) {
    todos.with_mut(|t| {
        rename_node_in_tree(t, id, &new_text, importance);
    });
}

fn rename_node_in_tree(nodes: &mut [TodoNode], id: usize, new_text: &str, importance: u32) {
    for node in nodes {
        if node.id == id {
            node.text = new_text.to_string();
            node.importance = importance;
            return;
        }
        rename_node_in_tree(&mut node.children, id, new_text, importance);
    }
}

fn add_child(todos: &mut Signal<Vec<TodoNode>>, parent_id: usize, new_id: usize) {
    todos.with_mut(|t| {
        add_child_to_tree(t, parent_id, new_id);
    });
}

fn add_child_to_tree(nodes: &mut [TodoNode], parent_id: usize, new_id: usize) {
    for node in nodes {
        if node.id == parent_id {
            node.children.push(TodoNode {
                id: new_id,
                text: "New subtask".to_string(),
                completed: false,
                importance: 1,
                children: vec![],
                deadline: None,
            });
            return;
        }
        add_child_to_tree(&mut node.children, parent_id, new_id);
    }
}

fn delete_node(todos: &mut Signal<Vec<TodoNode>>, id: usize) {
    todos.with_mut(|t| {
        delete_node_from_tree(t, id);
    });
}

fn delete_node_from_tree(nodes: &mut Vec<TodoNode>, id: usize) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        delete_node_from_tree(&mut node.children, id);
    }
}

#[component]
fn CalendarDay(day: i32, is_current_month: bool, on_double_click: EventHandler<()>) -> Element {
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

#[component]
fn Calendar() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let language_signal: Signal<String> = use_context();
    let nav = use_navigator();
    
    let mut current_month = use_signal(|| {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days_since_epoch = now / 86400;
        
        // Calculate year accounting for leap years
        let mut year = 1970i32;
        let mut remaining_days = days_since_epoch as i32;
        
        loop {
            let days_in_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 366 } else { 365 };
            if remaining_days < days_in_year {
                break;
            }
            remaining_days -= days_in_year;
            year += 1;
        }
        
        // Calculate month from remaining days
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let month_days = if is_leap {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        
        let mut month = 0;
        for (i, &days) in month_days.iter().enumerate() {
            if remaining_days < days {
                month = i as i32;
                break;
            }
            remaining_days -= days;
        }
        
        (year, month)
    });
    
    let mut calendar_modal_open = use_signal(|| false);
    let mut selected_day = use_signal(|| 0);
    let mut todos = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = load_save(seed) {
                return save.todos;
            }
        }
        vec![]
    });
    
    let mut next_id = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = load_save(seed) {
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
                        let _ = nav.push(Route::Home {});
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
                            on_create: move |(task_text, importance): (String, u32)| {
                                let deadline_timestamp = create_deadline_timestamp(year, month, selected_day());
                                todos
                                    .with_mut(|t| {
                                        t.push(TodoNode {
                                            id: next_id(),
                                            text: task_text.clone(),
                                            completed: false,
                                            importance,
                                            children: vec![],
                                            deadline: Some(deadline_timestamp),
                                        });
                                    });
                                next_id += 1;
                                if let Some(seed) = seed_signal.read().as_ref() {
                                    let save = SaveData {
                                        seed: seed.clone(),
                                        language: language_signal.read().clone(),
                                        todos: todos.read().clone(),
                                        next_id: next_id.read().clone(),
                                        points: 0,
                                    };
                                    save_data_to_storage(&save);
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

// Helper function to get days in month
fn get_days_in_month(month: i32, year: i32) -> i32 {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        1 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        3 | 5 | 8 | 10 => 30,
        _ => 31,
    }
}

// Helper function to get first day of week (0 = Sunday)
fn get_first_day_of_week(month: i32, year: i32) -> i32 {
    // Zeller's congruence algorithm
    let m = if month < 2 { month + 13 } else { month + 1 };
    let y = if month < 2 { year - 1 } else { year };
    let q = 1;
    
    let k = y % 100;
    let j = y / 100;
    
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    let day = (h + 5) % 7;
    
    if day < 0 { day + 7 } else { day }
}

fn format_month(month: i32) -> &'static str {
    match month {
        0 => "January",
        1 => "February",
        2 => "March",
        3 => "April",
        4 => "May",
        5 => "June",
        6 => "July",
        7 => "August",
        8 => "September",
        9 => "October",
        10 => "November",
        11 => "December",
        _ => "Unknown",
    }
}

fn create_deadline_timestamp(year: i32, month: i32, day: i32) -> u64 {
    // Use a safe calculation to avoid overflow
    // Calculate total days from 1970 to the target date
    let mut total_days: i64 = 0;
    
    // Add days for each year from 1970 to target year
    for y in 1970..year {
        if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            total_days += 366;
        } else {
            total_days += 365;
        }
    }
    
    // Add days for each month in the target year
    for m in 0..month {
        total_days += get_days_in_month(m, year) as i64;
    }
    
    // Add the day of month
    total_days += day as i64;
    
    // Convert to seconds, set time to end of day (23:59:59)
    let timestamp = (total_days * 86400i64 - 1) as u64; // -1 for 23:59:59
    timestamp
}

fn format_time_remaining(deadline: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if now >= deadline {
        "EXPIRED".to_string()
    } else {
        let remaining = deadline - now;
        let days = remaining / 86400;
        let hours = (remaining % 86400) / 3600;
        let minutes = (remaining % 3600) / 60;
        
        if days > 0 {
            format!("{}d {}h left", days, hours)
        } else if hours > 0 {
            format!("{}h {}m left", hours, minutes)
        } else {
            format!("{}m left", minutes)
        }
    }
}

fn add_root_node(todos: &mut Signal<Vec<TodoNode>>, new_id: usize) {
    todos.with_mut(|t| {
        t.push(TodoNode {
            id: new_id,
            text: "New task".to_string(),
            completed: false,
            importance: 1,
            children: vec![],
            deadline: None,
        });
    });
}

// Generate calendar days for a 6x7 grid
// Returns a vector of (day, is_current_month) tuples
fn generate_calendar_days(month: i32, year: i32, first_day_of_week: i32, days_in_month: i32) -> Vec<(i32, bool)> {
    let mut days = vec![];
    
    // Get previous month info
    let (prev_month, prev_year) = if month == 0 {
        (11, year - 1)
    } else {
        (month - 1, year)
    };
    let prev_days_in_month = get_days_in_month(prev_month, prev_year);
    
    // Add days from previous month
    let start_day = prev_days_in_month - first_day_of_week + 1;
    for day in start_day..=prev_days_in_month {
        days.push((day, false));
    }
    
    // Add days of current month
    for day in 1..=days_in_month {
        days.push((day, true));
    }
    
    // Add days from next month to fill the grid (6x7 = 42 days)
    let remaining = 42 - days.len() as i32;
    for day in 1..=remaining {
        days.push((day, false));
    }
    
    days
}
