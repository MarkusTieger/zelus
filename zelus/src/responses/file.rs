use crate::SUCCESS_DESCRIPTION;
use crate::responses::DocumentedResultResponse;
use crate::types::DataStream;
use axum::response::IntoResponse;
use futures_util::TryStreamExt as _;
use http::HeaderMap;
use std::collections::HashMap;
use std::io;
use utoipa::openapi::{Content, RefOr, Response, ResponsesBuilder, Schema};

pub struct FileResponse(pub Option<HeaderMap>, pub DataStream); // TODO: Add content type as parameter, when https://github.com/rust-lang/rust/issues/95174 is stable

impl From<reqwest::Response> for FileResponse {
    fn from(value: reqwest::Response) -> Self {
        Self(
            Some(value.headers().clone()),
            DataStream::by_stream(value.bytes_stream().map_err(io::Error::other)),
        )
    }
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1.into_axum()).into_response()
    }
}

impl DocumentedResultResponse for FileResponse {
    fn openapi(
        responses: ResponsesBuilder,
        _schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder {
        responses.response(
            "200",
            Response::builder()
                .description(SUCCESS_DESCRIPTION)
                .content("application/octet-stream", Content::builder().build())
                .build(),
        )
    }
}
