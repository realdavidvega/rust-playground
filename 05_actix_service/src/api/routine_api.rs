use crate::{models::routine_model::Routine, repository::mongo_repo::MongoRepo};
use actix_web::{
    get,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/routines")]
pub async fn create_routine(db: Data<MongoRepo>, new_routine: Json<Routine>) -> HttpResponse {
    let routine = new_routine.into_inner();
    let id = db.create_routine(routine).await;
    match id {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/routines/{id}")]
pub async fn get_routine(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_routine(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}