use serde::{Deserialize, Serialize};

/// Core domain model for a todo item in a tree structure
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TodoNode {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub importance: u32, // 1 = green, 2 = red, 3 = purple
    pub children: Vec<TodoNode>,
    pub deadline: Option<u64>, // Unix timestamp in seconds, None for regular tasks
}

/// Persisted user data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub seed: String,
    pub language: String,
    pub todos: Vec<TodoNode>,
    pub next_id: usize,
    pub points: u32,
}

/// UI-specific route enum (simple, non-Routable)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Login,
    Home,
    Options,
    Tasks,
    Calendar,
}
