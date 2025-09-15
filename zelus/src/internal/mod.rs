mod error;
mod header;
mod json;
mod serializer;
mod special;

pub use error::{error_by_response, error_schema};
pub use header::{header_insert, header_name};
pub use json::FrameworkJsonResponse;
pub use serializer::StringSerializer;
pub use special::AxumSpecialWrapper;

#[must_use]
pub fn from_raw<T: From<reqwest::Response>>(response: reqwest::Response) -> T {
    T::from(response)
}
