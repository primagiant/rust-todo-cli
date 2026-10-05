use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "[X]" } else { "[ ]" };
        write!(f, "{} {} - {}", status, self.id, self.title)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todolist {
    pub items: Vec<Task>,
}

impl Todolist {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_task(&mut self, title: String) {
        let id = self.items.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task::new(id, title);
        self.items.push(task);
    }

    pub fn list_task(&self) {
        if self.items.is_empty() {
            println!("Belum ada tugas.");
        } else {
            print!("{}", self);
        }
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(todo) = self.items.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            Ok(())
        } else {
            Err(format!("Task dengan ID {} tidak ditemukan.", id))
        }
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.items.iter().position(|t| t.id == id) {
            self.items.remove(index);
            Ok(())
        } else {
            Err(format!("Task dengan ID {} tidak ditemukan.", id))
        }
    }
}

impl fmt::Display for Todolist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for todo in &self.items {
            writeln!(f, "{}", todo)?;
        }
        Ok(())
    }
}
