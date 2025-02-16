use crate::models::Task;
use std::fs;

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(tasks)?;
    fs::write("tasks.json", json)?;
    Ok(())
}

pub fn load_tasks() -> Vec<Task> {
    if let Ok(json) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&json).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}