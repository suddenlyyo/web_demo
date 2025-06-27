use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
};
use crate::{model::{CreateUserDto, UpdateUserDto}, service::UserService};

#[get("/users")]
pub async fn get_all_users(service: Data<UserService>) -> impl Responder {
    let users = service.get_all_users();
    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
pub async fn get_user(
    service: Data<UserService>,
    path: Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    match service.get_user_by_id(id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[post("/users")]
pub async fn create_user(
    service: Data<UserService>,
    dto: Json<CreateUserDto>,
) -> impl Responder {
    let user = service.create_user(dto.into_inner());
    HttpResponse::Created().json(user)
}

#[put("/users/{id}")]
pub async fn update_user(
    service: Data<UserService>,
    path: Path<u32>,
    dto: Json<UpdateUserDto>,
) -> impl Responder {
    let id = path.into_inner();
    match service.update_user(id, dto.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    service: Data<UserService>,
    path: Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    if service.delete_user(id) {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}