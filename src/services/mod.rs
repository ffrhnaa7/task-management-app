// Instead of directly using `Task`, use `ModelTask` now
// Import Task with an alias to avoid conflict
use crate::models::Task as ModelTask;

pub struct TaskManager {
    tasks: Vec<ModelTask>, // Update type of `tasks` field
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new(), next_id: 1 }
    }

    pub fn list_tasks(&self) -> Vec<ModelTask> {
        self.tasks.clone()
    }

    pub fn add_task(&mut self, description: String) {
        let task = ModelTask {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    pub fn complete_task(&mut self, id: u32) -> Option<ModelTask> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task.clone()) // Return the completed task
        } else {
            None
        }
    }

    pub fn delete_task(&mut self, id: u32) -> Option<ModelTask> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(pos)) // Return the removed task
        } else {
            None
        }
    }

    pub fn set_tasks(&mut self, tasks: Vec<ModelTask>) {
        self.tasks = tasks;
    }

    pub fn set_next_id(&mut self, next_id: u32) {
        self.next_id = next_id;
    }
}
