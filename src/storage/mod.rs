use crate::models::Task;
use std::fs;
use serde_json;

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(tasks)?;
    let dir = std::path::Path::new("tasks.json").parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }

    if let Err(e) = fs::write("tasks.json", json) {
        eprintln!("Failed to save tasks to file: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn load_tasks() -> Vec<Task> {
    if let Ok(json) = fs::read_to_string("tasks.json") {
        match serde_json::from_str(&json) {
            Ok(tasks) => tasks,
            Err(_) => {
                eprintln!("Failed to parse tasks from JSON. Returning empty tasks list.");
                Vec::new()
            }
        }
    } else {
        Vec::new()  // Return empty list if file can't be read
    }
}
