use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::{Read, Write},
};

use crate::task::Task;

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        // Open tasks.json and read it into a string.
        let file = File::open("rust_list.json");

        // Check if the file exists
        if file.is_err() {
            // If it doesn't, return an empty TaskList.
            return TaskList { tasks: Vec::new() };
        }

        // If it does, read it into a string
        let mut json = String::new();
        file.unwrap().read_to_string(&mut json).unwrap();

        // Deserialize the string into a TaskList
        let tasks: TaskList = serde_json::from_str(&json).unwrap();
        return tasks;
    }

    pub fn add(&mut self, name: String, description: String) {
        // Create a new task.
        let task = Task::new(name, description);

        // Add it to the list.
        self.tasks.push(task);
    }

    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn toggle(&mut self, index: usize) {
        self.tasks[index].toggle();
    }

    pub fn format(&self) -> Vec<String> {
        let mut formatted_tasks = Vec::new();

        for task in self.tasks.iter() {
            formatted_tasks.push(task.format());
        }

        return formatted_tasks;
    }

    pub fn save(&self) {
        let mut file = File::create("rust_list.json").unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}
