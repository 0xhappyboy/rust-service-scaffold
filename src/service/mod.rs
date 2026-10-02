mod delete;
mod get;
mod patch;
mod post;
mod put;
use crate::types::{HttpMethod, Request, Response};
pub use delete::DeleteService;
pub use get::GetService;
pub use patch::PatchService;
pub use post::PostService;
pub use put::PutService;
/// registry
crate::services![
    GetService,
    PostService,
    PutService,
    DeleteService,
    PatchService,
];
/// Every service (bound to one URI) must implement this trait.
pub trait Service: Send + Sync + 'static {
    /// The URI path this service exposes, e.g. "/users"
    fn uri() -> &'static str;
    /// The HTTP method this service handles
    fn method() -> HttpMethod;
    /// Handle a request and produce a response.
    fn handle(req: Request) -> Response;
}
