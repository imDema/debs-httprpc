//! Run with
//!
//! ```not_rust
//! cargo run -p example-tls-rustls
//! ```

pub mod grpc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};

use grpc::Pool;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::*;

pub mod api {
    tonic::include_proto!("challenger"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Some(addr) = std::env::args().nth(1) else {
        panic!("give me an address as first arg");
    };

    let pool = Pool::builder(grpc::GrpcPoolManager::new(&addr))
        .max_size(128)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/create", post(create_new_benchmark))
        .route("/start", post(start_benchmark))
        .route("/end", post(end_benchmark))
        .route("/next_batch", post(next_batch))
        .route("/result_q1", post(result_q1))
        .route("/result_q2", post(result_q2))
        .with_state(pool.clone());

    // run https server
    let addr: SocketAddr = "127.0.0.1:3030".parse().unwrap();
    tracing::debug!("listening on {addr}");
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_new_benchmark(
    State(pool): State<Pool>,
    Json(body): Json<BenchmarkConfiguration>,
) -> Result<Json<Benchmark>, (StatusCode, String)> {
    let r = pool
        .get()
        .await
        .unwrap()
        .create_new_benchmark(body)
        .await
        .unwrap();
    Ok(Json(r.into_inner()))
}

async fn start_benchmark(
    State(pool): State<Pool>,
    Json(body): Json<Benchmark>,
) -> Result<(), (StatusCode, String)> {
    pool.get()
        .await
        .unwrap()
        .start_benchmark(body)
        .await
        .unwrap();
    Ok(())
}

async fn end_benchmark(
    State(pool): State<Pool>,
    Json(body): Json<Benchmark>,
) -> Result<(), (StatusCode, String)> {
    pool.get().await.unwrap().end_benchmark(body).await.unwrap();
    Ok(())
}

async fn next_batch(
    State(pool): State<Pool>,
    Json(body): Json<Benchmark>,
) -> Result<Json<Batch>, (StatusCode, String)> {
    let r = pool.get().await.unwrap().next_batch(body).await.unwrap();
    Ok(Json(r.into_inner()))
}

async fn result_q1(
    State(pool): State<Pool>,
    Json(body): Json<ResultQ1>,
) -> Result<(), (StatusCode, String)> {
    pool.get().await.unwrap().result_q1(body).await.unwrap();
    Ok(())
}

async fn result_q2(
    State(pool): State<Pool>,
    Json(body): Json<ResultQ2>,
) -> Result<(), (StatusCode, String)> {
    pool.get().await.unwrap().result_q2(body).await.unwrap();
    Ok(())
}
