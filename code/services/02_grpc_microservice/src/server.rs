// importing tonic server, request, response and status
use tonic::{transport::Server, Request, Response, Status};
// proto defines for us the service, the server, the request and the response
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentsRequest, BtcPaymentsResponse};

// include proto
pub mod payments {
    tonic::include_proto!("payments");
}

// define service
#[derive(Debug, Default)]
pub struct BitcoinService {}

// impl service with send_payment async method
#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentsRequest>,
    ) -> Result<Response<BtcPaymentsResponse>, Status> {
        println!("Got a request {:?}", request);
        let req = request.into_inner();

        // default reply, always successful
        let reply = BtcPaymentsResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into()
        };
        Ok(Response::new(reply))
    }
}

//async main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // loopback address in ipv6, equal to 127.0.0.1 in ipv4
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    // serve server with service
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
