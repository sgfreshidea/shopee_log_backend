use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

pub mod controllers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let traces =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "shopee_logs_collector=trace".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(traces)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = models::blank_db();
    let api = routes::all(db);
    let routes = api.with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 1729)).await;
}
