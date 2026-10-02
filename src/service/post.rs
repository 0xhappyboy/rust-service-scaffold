use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub struct PostService;
impl Service for PostService {
    fn uri() -> &'static str {
        "/post"
    }
    fn method() -> HttpMethod {
        HttpMethod::Post
    }
    fn handle(_req: Request) -> Response {
        println!("POST -> Hello, world!");
        Response::ok("Hello, world!")
    }
}
