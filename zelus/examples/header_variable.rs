use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use zelus::error::BlankError;
use zelus::{define_header_variable, service};

define_header_variable!(header_variable "This is an example header variable");

#[service]
trait ExampleService {
    #[route("/", method = GET, no_auth)]
    async fn example(
        &self,
        header_variable: TypedHeader<Authorization<Bearer>>,
    ) -> Result<(), BlankError>;
}

fn main() {}
