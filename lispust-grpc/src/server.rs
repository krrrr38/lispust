extern crate lispust_core;

use tonic::{transport::Server, Request, Response, Status};

use lispust::lispust_server::{Lispust, LispustServer};
use lispust::{RunRequest, RunResponse};

pub mod lispust {
    tonic::include_proto!("lispust");
}

#[derive(Default)]
pub struct LispustService {}

#[tonic::async_trait]
impl Lispust for LispustService {
    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let message = match lispust_core::run(&request.into_inner().message) {
            Ok(ret) => ret,
            Err(e) => e.to_string(),
        };
        let reply = lispust::RunResponse { message };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let svc = LispustService::default();

    println!("LispustServer listening on {}", addr);

    Server::builder()
        .add_service(LispustServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
