// proto defines for us the client and the request
use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentsRequest;

// include proto
pub mod payments {
    tonic::include_proto!("payments");
}

// async main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // loopback address in ipv6, equal to 127.0.0.1 in ipv4
    let addr = "http://[::1]:50051";

    // connect to addr
    let mut client = BitcoinClient::connect(addr).await?;

    let req = tonic::Request::new(
        BtcPaymentsRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            amount: 22
        }
    );

    let res = client.send_payment(req).await?;
    println!("RESPONSE={:?}", res);
    Ok(())
}

