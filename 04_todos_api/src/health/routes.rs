use dropshot::{ApiDescription, HttpError, HttpResponseOk, RequestContext, endpoint};
use std::sync::Arc;

#[endpoint {
    method = GET,
    path = "/health",
}]
async fn health(_ctx: Arc<RequestContext<()>>) -> Result<HttpResponseOk<String>, HttpError> {
    Ok(HttpResponseOk(String::from("Service up and running.")))
}

pub fn register(api: &mut ApiDescription<()>) {
    api.register(health).unwrap();
}
