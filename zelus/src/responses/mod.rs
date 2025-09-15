mod file;
mod json;
mod ws;

pub use file::FileResponse;
use std::collections::HashMap;
use utoipa::openapi::{RefOr, ResponsesBuilder, Schema};
pub use ws::WebsocketResponse;

pub trait DocumentedResponse {
    fn openapi(
        responses: ResponsesBuilder,
        schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder;
}
