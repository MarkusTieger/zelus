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

pub trait DocumentedResultResponse {
    fn openapi(
        responses: ResponsesBuilder,
        schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder;
}

impl<T: DocumentedResultResponse, E: DocumentedResponse + 'static> DocumentedResponse
    for Result<T, E>
{
    fn openapi(
        responses: ResponsesBuilder,
        schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder {
        E::openapi(T::openapi(responses, schemas), schemas)
    }
}
