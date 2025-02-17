mod models;
mod services;
mod storage;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use services::TaskManager;
use std::sync::Mutex;
use models::Task as ModelTask; // Alias Task as ModelTask to avoid conflict
use actix_cors::Cors;
// Shared state for the TaskManager
struct AppState {
    task_manager: Mutex<TaskManager>,
}

// API: List all tasks
async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let manager = data.task_manager.lock().unwrap();
    let tasks = manager.list_tasks().clone();
    HttpResponse::Ok().json(tasks)
}

// API: Add a task
async fn add_task(
    data: web::Data<AppState>,
    task_description: web::Json<String>,
) -> impl Responder {
    let mut manager = data.task_manager.lock().unwrap();
    manager.add_task(task_description.into_inner());
    HttpResponse::Ok().body("Task added!")
}

// API: Complete a task
async fn complete_task(data: web::Data<AppState>, task_id: web::Path<u32>) -> impl Responder {
    let mut manager = data.task_manager.lock().unwrap();
    let id = task_id.into_inner();
    if manager.complete_task(id).is_some() {
        HttpResponse::Ok().body(format!("Task {} marked as completed.", id))
    } else {
        HttpResponse::NotFound().body(format!("Task with ID {} not found.", id))
    }
}

// API: Delete a task
async fn delete_task(data: web::Data<AppState>, task_id: web::Path<u32>) -> impl Responder {
    let mut manager = data.task_manager.lock().unwrap();
    let id = task_id.into_inner();
    if manager.delete_task(id).is_some() {
        HttpResponse::Ok().body(format!("Task {} deleted.", id))
    } else {
        HttpResponse::NotFound().body(format!("Task with ID {} not found.", id))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize TaskManager and load tasks
    let mut manager = TaskManager::new();
    let tasks = storage::load_tasks();
    manager.set_tasks(tasks);
    let next_id = manager.list_tasks().iter().map(|t| t.id).max().unwrap_or(0) + 1;
    manager.set_next_id(next_id);

    // Wrap TaskManager in shared state
    let app_state = web::Data::new(AppState {
        task_manager: Mutex::new(manager),
    });

    // Start the web server
    println!("Starting server at http://localhost:8081/tasks");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::permissive())
            .route("/tasks", web::get().to(list_tasks))
            .route("/tasks", web::post().to(add_task))
            .route("/tasks/{id}/complete", web::put().to(complete_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}