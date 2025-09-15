use tokio::net::TcpListener;
use zelus::error::BlankError;
use zelus::{axum, framework_router, service};

#[derive(Clone)]
struct Base;

#[service]
trait ExampleService {
    #[route("/", method = GET, no_auth)]
    async fn example(&self) -> Result<(), BlankError>;
}

#[zelus::async_trait]
impl ExampleService for Base {
    async fn example(&self) -> Result<(), BlankError> {
        // implementation here
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let base = Base;
    let (router,) = framework_router!(Base base (default,) { ExampleService });
    axum::serve(
        TcpListener::bind("[::]:3000")
            .await
            .expect("Unable to bind"),
        router
            .into_openapi()
            .split_for_parts()
            .0
            .into_make_service(),
    )
    .await
    .expect("Unable to serve http server");
}
