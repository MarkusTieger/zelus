use crate::responses::DocumentedResultResponse;
use axum::response::IntoResponse;
use http::header::LOCATION;
use http::{HeaderValue, StatusCode};
use std::collections::HashMap;
use utoipa::openapi::{RefOr, Response, ResponsesBuilder, Schema};

#[must_use]
pub struct Redirect<const CODE: u16>(axum::response::Redirect);

trait RedirectStatusCode {
    const STATUS: StatusCode;

    fn new_internal(uri: &str) -> axum::response::Redirect
    where
        Self: Sized;
}
impl RedirectStatusCode for Redirect<303> {
    const STATUS: StatusCode = StatusCode::SEE_OTHER;

    fn new_internal(uri: &str) -> axum::response::Redirect
    where
        Self: Sized,
    {
        axum::response::Redirect::to(uri)
    }
}
impl RedirectStatusCode for Redirect<307> {
    const STATUS: StatusCode = StatusCode::TEMPORARY_REDIRECT;

    fn new_internal(uri: &str) -> axum::response::Redirect
    where
        Self: Sized,
    {
        axum::response::Redirect::temporary(uri)
    }
}
impl RedirectStatusCode for Redirect<308> {
    const STATUS: StatusCode = StatusCode::PERMANENT_REDIRECT;

    fn new_internal(uri: &str) -> axum::response::Redirect
    where
        Self: Sized,
    {
        axum::response::Redirect::permanent(uri)
    }
}

impl<const CODE: u16> DocumentedResultResponse for Redirect<CODE>
where
    Self: RedirectStatusCode,
{
    fn openapi(
        responses: ResponsesBuilder,
        _schemas: &mut HashMap<String, RefOr<Schema>>,
    ) -> ResponsesBuilder {
        responses.response(
            CODE.to_string(),
            Response::builder()
                .description(
                    Self::STATUS
                        .canonical_reason()
                        .unwrap_or_else(|| Self::STATUS.as_str()),
                )
                .build(),
        )
    }
}

impl<const CODE: u16> Redirect<CODE>
where
    Self: RedirectStatusCode,
{
    pub fn new(uri: &str) -> Self {
        Self(Self::new_internal(uri))
    }

    pub fn into_uri(self) -> HeaderValue {
        self.0
            .into_response()
            .headers()
            .get(LOCATION)
            .cloned()
            .expect("axum::response::Redirect should always have a location header")
    }
}

impl<const CODE: u16> IntoResponse for Redirect<CODE>
where
    Self: RedirectStatusCode,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
