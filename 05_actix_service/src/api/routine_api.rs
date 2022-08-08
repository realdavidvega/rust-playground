use crate::{models::routine_model::Routine, repository::mongo_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/routine")]
pub async fn create_routine(db: Data<MongoRepo>, new_routine: Json<Routine>) -> HttpResponse {
    let routine = new_routine.into_inner();
    let id = db.create_routine(routine).await;
    match id {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}