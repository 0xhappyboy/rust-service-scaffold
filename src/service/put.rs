use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub struct PutService;
impl Service for PutService {
    fn uri() -> &'static str {
        "/put"
    }
    fn method() -> HttpMethod {
        HttpMethod::Put
    }
    fn handle(_req: Request) -> Response {
        println!("PUT  -> Hello, world!");
        Response::ok("Hello, world!")
    }
}
