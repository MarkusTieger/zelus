#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use zelus::error::BlankError;
use zelus::{service, utoipa};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
struct ExampleResult {
    test: u32,
}

#[service]
trait ExampleService {
    #[route("/", method = GET, no_auth)]
    async fn example(&self) -> Result<ExampleResult, BlankError>;
}

fn main() {}
