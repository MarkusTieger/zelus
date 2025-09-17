mod file;
mod json;
mod redirect;
mod ws;

pub use file::FileResponse;
pub use redirect::Redirect;
use std::collections::HashMap;
use utoipa::openapi::{RefOr, ResponsesBuilder, Schema};
pub use ws::WebsocketResponse;

pub trait DocumentedResponse {
    fn openapi(
        responses: ResponsesBuilder,
        schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder;
}
