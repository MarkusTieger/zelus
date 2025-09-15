use zelus::error::BlankError;
use zelus::{define_query_variable, service};

define_query_variable!(query_variable "This is an example query variable");

#[service]
trait ExampleService {
    #[route("/", method = GET, query = [ query_variable ], no_auth)]
    async fn example(&self, query_variable: u32) -> Result<(), BlankError>;
}

fn main() {}
