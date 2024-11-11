use actix_web::{web, HttpResponse, Responder};

pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Fetching user data")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Updating user data")
}

pub async fn add_project() -> impl Responder {
    HttpResponse::Ok().body("Adding project")
}

pub async fn get_projects() -> impl Responder {
    HttpResponse::Ok().body("Fetching projects")
}

pub async fn get_single_project() -> impl Responder {
    HttpResponse::Ok().body("Fetching single project")
}

pub async fn delete_project(path: web::Path<(u32,)>) -> impl Responder {
    let id = path.into_inner().0;
    HttpResponse::Ok().body(format!("Deleting project with id: {}", id))
}
