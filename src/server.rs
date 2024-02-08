//! Dummy server
use rand::distributions::Standard;
use rand::prelude::*;

use tonic::{transport::Server, Response};

mod api;
use api::*;

use api::challenger_server::{Challenger, ChallengerServer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Default)]
pub struct DumbServer {}

#[tonic::async_trait]
impl Challenger for DumbServer {
    /// Create a new Benchmark based on the configuration
    async fn create_new_benchmark(
        &self,
        _request: tonic::Request<api::BenchmarkConfiguration>,
    ) -> Result<tonic::Response<api::Benchmark>, tonic::Status> {
        Ok(Response::new(api::Benchmark { id: 0xdeadbeef }))
    }
    /// This marks the starting point of the throughput measurements
    async fn start_benchmark(
        &self,
        _request: tonic::Request<api::Benchmark>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(Response::new(()))
    }
    /// get the next Batch
    async fn next_batch(
        &self,
        _request: tonic::Request<api::Benchmark>,
    ) -> Result<tonic::Response<api::Batch>, tonic::Status> {
        Ok(Response::new(Batch {
            seq_id: 2,
            last: false,
            day_end: false,
            vault_ids: vec![1001, 1003],
            cluster_ids: vec![5, 6],
            states: vec![DriveState {
                date: Some(Timestamp { seconds: 12345345, nanos: 5467 }),
                serial_number: "1234567890".into(),
                model: "MODEL12345".into(),
                failure: false,
                vault_id: 1003,
                readings: SmallRng::from_rng(thread_rng()).unwrap().sample_iter(Standard).take(35).collect(),
            }],
        }))
    }
    /// post the result
    async fn result_q1(
        &self,
        _request: tonic::Request<api::ResultQ1>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(Response::new(()))
    }
    async fn result_q2(
        &self,
        _request: tonic::Request<api::ResultQ2>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(Response::new(()))
    }
    /// This marks the end of the throughput measurements
    async fn end_benchmark(
        &self,
        _request: tonic::Request<api::Benchmark>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "h2=info,debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = "127.0.0.1:5052".parse()?;
    let dumb = DumbServer::default();

    tracing::info!("starting server, binding to {addr}");

    Server::builder()
        .add_service(ChallengerServer::new(dumb))
        .serve(addr)
        .await?;

    Ok(())
}
