use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        Task {
            name: name,
            description: description,
            done: false,
        }
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }

    pub fn format(&self) -> String {
        // Determine whether the done field will be occupied by a checkmark or a space.
        let done = if self.done { "✓" } else { " " };

        format!("[{}] {}: {}", done, self.name, self.description)
    }
}
