use crate::models::{TodoNode, TaskType};
use std::time::{SystemTime, UNIX_EPOCH};

/// Factory for creating TodoNode instances with consistent defaults
pub struct TodoNodeFactory;

impl TodoNodeFactory {
    /// Create a regular task
    pub fn create_regular(id: usize, text: String) -> TodoNode {
        TodoNode {
            id,
            text,
            completed: false,
            importance: 1,
            children: vec![],
            deadline: None,
            task_type: TaskType::Regular,
            last_completed_date: None,
        }
    }

    /// Create a habit task (resets daily)
    pub fn create_habit(id: usize, text: String, importance: u32) -> TodoNode {
        TodoNode {
            id,
            text,
            completed: false,
            importance,
            children: vec![],
            deadline: None,
            task_type: TaskType::Habit,
            last_completed_date: None,
        }
    }

    /// Create a deadline task (high importance)
    pub fn create_deadline(id: usize, text: String, deadline: u64) -> TodoNode {
        TodoNode {
            id,
            text,
            completed: false,
            importance: 3, // Auto high importance for deadlines
            children: vec![],
            deadline: Some(deadline),
            task_type: TaskType::Regular,
            last_completed_date: None,
        }
    }

    /// Create a subtask
    pub fn create_subtask(id: usize, text: String, importance: u32) -> TodoNode {
        TodoNode {
            id,
            text,
            completed: false,
            importance,
            children: vec![],
            deadline: None,
            task_type: TaskType::Regular,
            last_completed_date: None,
        }
    }
}

/// Check if a habit needs to be reset (new day)
pub fn should_reset_habit(last_completed: Option<u64>) -> bool {
    match last_completed {
        None => false, // Never completed, no reset needed
        Some(timestamp) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Different day if more than 86400 seconds (1 day) have passed
            (now - timestamp) > 86400
        }
    }
}

/// Get today's date as Unix timestamp (start of day)
pub fn get_today_timestamp() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Get start of today (00:00:00)
    (now / 86400) * 86400
}
