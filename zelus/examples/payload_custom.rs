#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use zelus::error::BlankError;
use zelus::{service, utoipa};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
struct PayloadStruct {
    field1: u32,
    field2: String,
}

#[service]
trait ExampleService {
    #[route("/", method = POST, no_auth)]
    #[example("payload.json")] // This is optional. It loads the request example
    async fn example(&self, payload: PayloadStruct) -> Result<(), BlankError>; // When there is only one payload argument, it is used as base
}

fn main() {}
