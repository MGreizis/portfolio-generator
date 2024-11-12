use crate::models::{Project, User};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
// use std::collections::HashMap;
// use std::sync::Mutex;

/// Returns the current user data as JSON if it exists, or a 404 response if not.
pub async fn get_user(data: web::Data<super::AppState>) -> impl Responder {
    let user = data.user.lock().unwrap();
    match &*user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

/// Updates the current user data to the provided `User` JSON object.
/// Returns a 200 response with the text "User data updated" if successful, or a 500 response if not.
pub async fn update_user(
    data: web::Data<super::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let mut user_data = data.user.lock().unwrap();
    *user_data = Some(user.into_inner());
    HttpResponse::Ok().body("User data updated")
}

/// Adds a new project to the list of all projects, keyed by the project's ID, and returns a 200 response with a message indicating the ID of the new project.
pub async fn add_project(data: web::Data<AppState>, project: web::Json<Project>) -> impl Responder {
    let mut projects = data.projects.lock().unwrap();
    let project_id = project.id;
    projects.insert(project_id, project.into_inner());
    HttpResponse::Ok()
        .status(actix_web::http::StatusCode::OK)
        .body(format!("Project with id {} created", project_id))
}

/// Returns all projects as a JSON object, where each project is keyed by its ID.
/// Returns a 200 response if successful, or a 500 response if not.
pub async fn get_projects(data: web::Data<AppState>) -> impl Responder {
    let projects = data.projects.lock().unwrap(); // Lock the projects hashmap
    let project_list: Vec<_> = projects.values().cloned().collect();
    HttpResponse::Ok().json(project_list) // Return all projects as JSON
}

/// Returns a single project with the given ID as a JSON object if it exists, or a 404 response if not.
pub async fn get_single_project(
    data: web::Data<super::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let projects = data.projects.lock().unwrap();
    let project_id = path.into_inner();

    if let Some(project) = projects.get(&project_id) {
        HttpResponse::Ok().json(project) // Return the project if found
    } else {
        HttpResponse::NotFound().body("Project not found") // Return 404 if not found
    }
}

/// Deletes a project with the given ID from the list of all projects.
///
/// If the project with the specified ID exists, it is removed from the list,
/// and a 200 response with a message indicating the ID of the deleted project is returned.
/// If the project does not exist, a 404 response with a "Project not found" message is returned.
pub async fn delete_project(
    data: web::Data<super::AppState>,
    path: web::Path<(u32,)>,
) -> impl Responder {
    let id = path.into_inner().0;
    let mut projects = data.projects.lock().unwrap();
    if projects.remove(&id).is_some() {
        HttpResponse::Ok().body(format!("Project with id {} deleted", id))
    } else {
        HttpResponse::NotFound().body("Project not found")
    }
}
