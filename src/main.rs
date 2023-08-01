#![forbid(unsafe_code)]

// use std::path::PathBuf;

use aws_rust_testbench::{actions, auth, dummy};

use axum::{
    extract::{MatchedPath, OriginalUri, Path},
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
// use axum_server::tls_rustls::RustlsConfig;

use bb8::Pool;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncMysqlConnection, RunQueryDsl,
};
// use lambda_http::{http::StatusCode, run, Error};

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    #[cfg(debug_assertions)]
    {
        // Load environment variable from .env file
        dotenvy::dotenv()?;
    }

    std::env::set_var("DATABASE_URL", "");
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .json()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
                path.path().to_owned()
            } else {
                request.uri().path().to_owned()
            };

            tracing::info_span!(
                "http_request",
                method = ?request.method(),
                path,
                some_other_field = tracing::field::Empty,
            )
        })
        .on_request(|_request: &Request<_>, _span: &tracing::Span| {
            tracing::info!(message = "begin request!")
        });

    // Set up the database connection
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(db_url);
    let connection = Pool::builder()
        .build(config)
        .await
        .expect("unable to establish the database connection");

    // Define all routes
    let dummy_routes = Router::new()
        .route("/", get(dummy::root))
        .route("/foo", get(dummy::get_foo).post(dummy::post_foo))
        .route("/foo/:name", post(dummy::post_foo_name))
        .with_state(connection);

    let auth_routes = Router::new()
        .route("/login", get(auth::login))
        .route("/signup", get(auth::signup));

    let actions_routes = Router::new().route("/", get(actions::do_something));

    // Nest routes
    let api_routes = Router::new()
        .nest("/dummy", dummy_routes)
        .nest("/auth", auth_routes)
        .nest("/actions", actions_routes);

    let app = Router::new().nest("/api/v1", api_routes).layer(trace_layer);

    // configure certificate and private key used by https
    // let config = RustlsConfig::from_pem_file(
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("dummy_certs")
    //         .join("cert.pem"),
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("dummy_certs")
    //         .join("key.pem"),
    // )
    // .await
    // .unwrap();

    #[cfg(debug_assertions)]
    {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        axum::Server::bind(&addr) // Without https
            // axum_server::bind_rustls(addr, config) // With https
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
