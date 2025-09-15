use zelus::error::BlankError;
use zelus::{define_path_variable, service};

define_path_variable!(path_variable "This is an example path variable");

#[service]
trait ExampleService {
    #[route("/{path_variable}", method = GET, no_auth)]
    async fn example(&self, path_variable: u32) -> Result<(), BlankError>;
}

fn main() {}
