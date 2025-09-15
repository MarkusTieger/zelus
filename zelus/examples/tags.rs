use zelus::error::BlankError;
use zelus::service;

#[service(tag = "example")] // With tags you can make some kind of "categories" in the openapi documentation
trait ExampleService {
    #[route("/", method = GET, no_auth)]
    async fn example(&self) -> Result<(), BlankError>;
}

fn main() {}
