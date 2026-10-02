use crate::{
    service::Service,
    types::{HttpMethod, Request, Response},
};
pub type HandlerFn = fn(Request) -> Response;
#[derive(Clone, Copy)]
pub struct ServiceEntry {
    pub method: HttpMethod,
    pub uri: &'static str,
    pub handler: HandlerFn,
}
#[derive(Default)]
pub struct ServiceRegistry {
    entries: Vec<ServiceEntry>,
}
impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register<S: Service>(&mut self) -> &mut Self {
        self.entries.push(ServiceEntry {
            method: S::method(),
            uri: S::uri(),
            handler: S::handle,
        });
        self
    }
    pub fn entries(&self) -> &[ServiceEntry] {
        &self.entries
    }
}
/// Declare the full service list in ONE place.
///
/// Usage:
/// ```ignore
/// services![GetService, PostService, PutService];
/// ```
#[macro_export]
macro_rules! services {
    ($($svc:path),* $(,)?) => {
        /// The static list of all services to expose.
        pub fn all_services() -> $crate::registry::ServiceRegistry {
            let mut reg = $crate::registry::ServiceRegistry::new();
            $( reg.register::<$svc>(); )*
            reg
        }
    };
}
