#![forbid(unsafe_code)]

// use aws_rust_testbench::foo;

use tracing::info;

use aws_rust_testbench::store::s3::get_object;

use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

use bb8::Pool;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncMysqlConnection, RunQueryDsl,
};
// use lambda_http::{http::StatusCode, run, Error};

async fn root() -> Json<Value> {
    info!("root");
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    info!("get_foo");
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    info!("get_foo");
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    // Load environment variable from .env file
    dotenvy::dotenv()?;

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // Set up the database connection
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(db_url);
    let connection = Pool::builder()
        .build(config)
        .await
        .expect("unable to establish the database connection");

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .with_state(connection);

    #[cfg(debug_assertions)]
    {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
        Ok(())
    }

    // This is an example function that leverages the Lambda Rust runtime's HTTP support
    // and the [axum](https://docs.rs/axum/latest/axum/index.html) web framework.  The
    // runtime HTTP support is backed by the [tower::Service](https://docs.rs/tower-service/0.3.2/tower_service/trait.Service.html)
    // trait.  Axum applications are also backed by the `tower::Service` trait.  That means
    // that it is fairly easy to build an Axum application and pass the resulting `Service`
    // implementation to the Lambda runtime to run as a Lambda function.  By using Axum instead
    // of a basic `tower::Service` you get web framework niceties like routing, request component
    // extraction, validation, etc.
    // AWS Rust axum example
    // https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/http-axum
    #[cfg(not(debug_assertions))]
    {
        lambda_http::run(app).await
    }
}
