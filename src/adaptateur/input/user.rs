use actix_web::{web, HttpResponse, Responder};
use crate::domain::models::user::{CreateUser, Users};
use crate::ports::input::user::UserRepository;
use uuid::Uuid;


pub async fn get_user(
    path: web::Path<Uuid>,
    repo: web::Data<dyn UserRepository>,
) -> impl Responder {
    let id = path.into_inner();
    match repo.find_by_id(id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn create_user(
    repo: web::Data<dyn UserRepository>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    match repo.create(user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_user(
    repo: web::Data<dyn UserRepository>,
    user: web::Json<Users>,
) -> impl Responder {
    match repo.update(user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_user(
    path: web::Path<Uuid>,
    repo: web::Data<dyn UserRepository>,
) -> impl Responder {
    let id = path.into_inner();
    match repo.delete(id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{id}", web::get().to(get_user))
            .route("", web::post().to(create_user))
            .route("", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}