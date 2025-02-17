# Task Manager

A simple Task Manager web application built with **Vanilla JavaScript** for the frontend and **Actix-web** for the backend. This app allows users to manage their tasks by adding, completing, and deleting them through a RESTful API.

## Features

- **Add a Task**: Submit a new task via a text input field.
- **Complete a Task**: Mark tasks as completed.
- **Delete a Task**: Remove tasks from the list.
- **Dark Mode**: Toggle between light and dark modes.

## Frontend (Vanilla JavaScript)

The frontend is a simple web application using HTML, CSS, and JavaScript:
- **Task Input**: Users can add tasks through an input field and a button.
- **Task List**: Displays tasks with options to mark as complete or delete.
- **Dark Mode Toggle**: Click to switch between dark and light themes.

**Key Elements**:
- Tasks are fetched from the backend API using `fetch()`.
- Tasks are displayed dynamically with event listeners to mark them complete or delete them.
- Dark Mode toggles by changing CSS classes.

## Backend (Actix-web)

The backend is built using **Rust's Actix-web** framework, offering a simple REST API for managing tasks.

### API Endpoints:
- **GET /tasks**: List all tasks.
- **POST /tasks**: Add a new task.
- **PUT /tasks/{id}/complete**: Mark a task as completed.
- **DELETE /tasks/{id}**: Delete a task.

### Core Logic:
- **TaskManager**: Manages task operations like adding, completing, and deleting tasks.
- **AppState**: Stores the `TaskManager` instance shared across API endpoints using `Mutex` for thread safety.
- **CORS**: Cross-Origin Resource Sharing (CORS) is enabled to allow requests from different origins.

The backend uses an in-memory `TaskManager` to store tasks, with the option to extend it for persistent storage.

## Running the Application

### Frontend:
1. Open `index.html` in your browser to interact with the frontend.

### Backend:
1. Run the backend with `cargo run` to start the server at `http://localhost:8081/tasks`.
2. Use the frontend to interact with the backend via the API.

## Future Plans

- **Upgrade Frontend Framework**: Improve the frontend by upgrading from Vanilla JavaScript to a more robust framework like **React** or **Vue.js** to enhance scalability and maintainability.
- **Recommendation System**: Implement a recommendation system using **Python** to help users decide how to arrange their tasks based on their **MBTI** personality type. This system will suggest optimal task prioritization strategies tailored to individual preferences.
- **User Authentication**: Add user authentication (login/logout) functionality to allow users to save their tasks for long-term use and maintain personalized task management.

## Conclusion

This application provides a basic task management system that can be extended further by adding features like authentication, task due dates, and persistent storage. You can toggle between dark and light modes, add new tasks, mark them as complete, and delete them using the backend API.
