// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{web, App, HttpServer};
mod handlers;

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
    HttpServer::new(|| {
        App::new()
            .route("/user", web::get().to(handlers::get_user))
            .route("/user", web::post().to(handlers::update_user))
            .route("/projects", web::post().to(handlers::add_project))
            .route("/projects", web::get().to(handlers::get_projects))
            .route(
                "/projects/{id}",
                web::get().to(handlers::get_single_project),
            )
            .route("/projects/{id}", web::delete().to(handlers::delete_project))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
