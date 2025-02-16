# Task Management App with Rust

A simple full-stack task management app with **Rust** (backend) and **React** (frontend). It allows users to create, view, update, and delete tasks.

---

## Features
- Add, view, update, and delete tasks.
- Filter tasks by status (e.g., pending, completed).
- Simple API integration for backend and frontend.

---

## Tech Stack
- **Backend**: Rust, Actix-web / Rocket, SQLite
- **Frontend**: React.js, Axios

---

## Backend Setup

1. **Install Rust**: Follow the [Rust Installation Guide](https://www.rust-lang.org/learn/get-started).
2. **Create the project**:
    ```bash
    cargo new task-manager-backend
    cd task-manager-backend
    ```
3. **Add dependencies** (Actix-web or Rocket) in `Cargo.toml`.
4. **Create API Endpoints** to manage tasks (CRUD).
5. **Run backend**:
    ```bash
    cargo run
    ```

---

## Frontend Setup

1. **Create React app**:
    ```bash
    npx create-react-app task-manager-frontend
    cd task-manager-frontend
    ```
2. **Install Axios**:
    ```bash
    npm install axios
    ```
3. **Fetch tasks** from backend and display them.
4. **Run frontend**:
    ```bash
    npm start
    ```

---

## Running the App

1. **Start backend**: `cargo run` in the `task-manager-backend` folder.
2. **Start frontend**: `npm start` in the `task-manager-frontend` folder.
3. Open the app at [http://localhost:3000](http://localhost:3000).

---

## Future Features
- Task categories
- User authentication (JWT)
- Task prioritization

---

