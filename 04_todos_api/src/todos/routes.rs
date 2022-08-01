use crate::todos::{Todo, TodoRequest};
use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk,
    HttpResponseUpdatedNoContent, Path, RequestContext, TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
struct PathParams {
    id: Uuid,
}

#[endpoint {
    method = GET,
    path = "/todos",
}]
async fn index(_ctx: Arc<RequestContext<()>>) -> Result<HttpResponseOk<Vec<Todo>>, HttpError> {
    let todos = Todo::index()?;
    Ok(HttpResponseOk(todos))
}

#[endpoint {
    method = POST,
    path = "/todos",
}]
async fn create(
    _ctx: Arc<RequestContext<()>>,
    body: TypedBody<TodoRequest>,
) -> Result<dropshot::HttpResponseCreated<Uuid>, HttpError> {
    let body = body.into_inner();
    validate_request(&body)?;

    let todo = Todo::create(body);
    match todo {
        Ok(v) => Ok(HttpResponseCreated(v)),
        Err(_) => Err(HttpError::for_bad_request(
            None,
            String::from("Unknown error")
        ))
    }
}

#[endpoint {
    method = PUT,
    path = "/todos/{id}",
}]
async fn update(
    _ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>,
    body: TypedBody<TodoRequest>,
) -> Result<dropshot::HttpResponseUpdatedNoContent, HttpError> {
    let body = body.into_inner();
    validate_request(&body)?;

    match Todo::update(path.into_inner().id, body) {
        Ok(_) => Ok(HttpResponseUpdatedNoContent()),
        Err(_) => Err(HttpError::for_bad_request(
            None,
            String::from("Unknown error")
        )),
    }
}

#[endpoint {
    method = DELETE,
    path = "/todos/{id}",
}]
async fn delete(
    _ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>,
) -> Result<dropshot::HttpResponseDeleted, HttpError> {
    let res = Todo::delete(path.into_inner().id);

    match res {
        Ok(0) => Err(HttpError::for_not_found(None, String::from("Not Found"))),
        Ok(1) => Ok(HttpResponseDeleted()),
        Ok(_) => Err(HttpError::for_not_found(None, String::from("Not Found"))),
        Err(_) => Err(HttpError::for_bad_request(None, String::from("Bad Request"))),
    }
}

pub fn register(api: &mut ApiDescription<()>) {
    api.register(index).unwrap();
    api.register(create).unwrap();
    api.register(update).unwrap();
    api.register(delete).unwrap();
}

fn validate_request(br: &TodoRequest) -> Result<(), HttpError> {
    if br.title.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, String::from("Empty Title")));
    }
    Ok(())
}
