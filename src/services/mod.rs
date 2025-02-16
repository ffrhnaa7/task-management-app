use crate::models::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Add a task
    pub fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    // List all tasks
    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    // Get tasks (mutable)
    pub fn get_tasks(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }

    // Set tasks
    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    // Set next_id
    pub fn set_next_id(&mut self, next_id: u32) {
        self.next_id = next_id;
    }

    // Complete a task
    pub fn complete_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            true
        } else {
            false
        }
    }

    // Delete a task
    pub fn delete_task(&mut self, id: u32) -> bool {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        self.tasks.len() != initial_len
    }
}