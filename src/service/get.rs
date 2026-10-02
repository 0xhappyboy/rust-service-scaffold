use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub struct GetService;
impl Service for GetService {
    fn uri() -> &'static str {
        "/get"
    }
    fn method() -> HttpMethod {
        HttpMethod::Get
    }
    fn handle(_req: Request) -> Response {
        println!("GET  -> Hello, world!");
        Response::ok("Hello, world!")
    }
}
