pub mod services {
    tonic::include_proto!("services"); // The string specified here must match the proto package name
}

use services::{payment_service_client::PaymentServiceClient, PaymentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(PaymentRequest {
        user_id: "user123".to_string(), 
        amount: 100.0,
    });

    let response = client.process_payment(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}