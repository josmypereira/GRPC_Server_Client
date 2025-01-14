use service::auth::authentication_service;
use tonic::transport::Server;
// use core::error;
use std::net::SocketAddr;

mod service;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr: SocketAddr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(authentication_service())
        .serve(addr)
        .await?;

    Ok(())
    
}
