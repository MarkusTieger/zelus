use zelus::error::BlankError;
use zelus::service;

#[service]
trait ExampleService {
    #[route("/", method = POST, no_auth)]
    #[example("payload.json")] // This is optional. It loads the request example
    async fn example(&self, field1: u32, field2: String) -> Result<(), BlankError>;
}

fn main() {}
