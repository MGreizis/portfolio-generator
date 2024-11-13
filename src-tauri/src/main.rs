// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::Mutex;
use tera::Tera;

mod handlers;
mod models;

use models::User;

struct AppState {
    user: Mutex<Option<User>>,
    projects: Mutex<HashMap<u32, models::Project>>,
    tera: Tera,
}

/// Starts the Tauri app HTTP server.
///
/// The server listens on `127.0.0.1:8080` and responds to the following routes:
///
/// - `GET /user`: Returns the current user data.
/// - `POST /user`: Updates the current user data.
/// - `POST /projects`: Creates a new project.
/// - `GET /projects`: Returns a list of all projects.
/// - `GET /projects/{id}`: Returns the project with the given `id`.
/// - `DELETE /projects/{id}`: Deletes the project with the given `id`.
///
/// The server is shutdown when the Tauri app is closed.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/*.html").expect("Failed to initialize Tera templates");

    // Creates a single instance of the shared data to be used across all routes.
    let shared_data = web::Data::new(AppState {
        user: Mutex::new(None),
        projects: Mutex::new(HashMap::new()),
        tera,
    });

    HttpServer::new(move || {
        App::new()
            // TODO: Fix TW
            // .service(fs::Files::new("/static", "/static").show_files_listing())    WORKS
            .app_data(shared_data.clone())
            .route("/user", web::get().to(handlers::get_user))
            .route("/user", web::post().to(handlers::update_user))
            .route("/projects", web::post().to(handlers::add_project))
            .route("/projects", web::get().to(handlers::get_projects))
            .route(
                "/projects/{id}",
                web::get().to(handlers::get_single_project),
            )
            .route("/projects/{id}", web::delete().to(handlers::delete_project))
            .route("/render", web::get().to(handlers::render_portfolio))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
