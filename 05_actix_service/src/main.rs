use actix_web::{App, HttpServer, web::Data};
use api::routine_api::{create_routine, get_routine};
use repository::mongo_repo::MongoRepo;

mod api; 
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_routine)
            .service(get_routine)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}