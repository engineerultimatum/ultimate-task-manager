use crate::models::{SaveData, TodoNode};

/// Builder for constructing SaveData with fluent API
pub struct SaveDataBuilder {
    seed: String,
    language: String,
    todos: Vec<TodoNode>,
    next_id: usize,
    points: u32,
}

impl SaveDataBuilder {
    pub fn new(seed: String) -> Self {
        Self {
            seed,
            language: "English".to_string(),
            todos: vec![],
            next_id: 0,
            points: 0,
        }
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }

    pub fn with_initial_task(mut self, task: TodoNode) -> Self {
        self.todos.push(task);
        self
    }

    pub fn with_next_id(mut self, id: usize) -> Self {
        self.next_id = id;
        self
    }

    pub fn with_points(mut self, points: u32) -> Self {
        self.points = points;
        self
    }

    pub fn build(self) -> SaveData {
        SaveData {
            seed: self.seed,
            language: self.language,
            todos: self.todos,
            next_id: self.next_id,
            points: self.points,
        }
    }
}
