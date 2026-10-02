use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub struct PatchService;
impl Service for PatchService {
    fn uri() -> &'static str {
        "/patch"
    }
    fn method() -> HttpMethod {
        HttpMethod::Patch
    }
    fn handle(_req: Request) -> Response {
        println!("PATCH -> Hello, world!");
        Response::ok("Hello, world!")
    }
}
