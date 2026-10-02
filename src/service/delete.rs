use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub struct DeleteService;
impl Service for DeleteService {
    fn uri() -> &'static str {
        "/delete"
    }
    fn method() -> HttpMethod {
        HttpMethod::Delete
    }
    fn handle(_req: Request) -> Response {
        println!("DELETE -> Hello, world!");
        Response::ok("Hello, world!")
    }
}
