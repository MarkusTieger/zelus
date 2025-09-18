use crate::responses::DocumentedResultResponse;
use axum::response::IntoResponse;
use std::collections::HashMap;
use utoipa::openapi::{RefOr, ResponsesBuilder, Schema};

pub struct WebsocketResponse(pub axum::response::Response);

impl IntoResponse for WebsocketResponse {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

impl DocumentedResultResponse for WebsocketResponse {
    fn openapi(
        responses: ResponsesBuilder,
        schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder {
        <()>::openapi(responses, schemas)
    }
}
