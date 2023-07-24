use tonic::transport::Channel;
use tonic::Request;

pub mod grpc {
    pub mod orderbook {
        tonic::include_proto!("orderbook");
    }
}

use crate::grpc::orderbook::order_book_client::OrderBookClient; 
use crate::grpc::orderbook::GetTopOrdersRequest;

pub async fn print_top_orders(
    mut client: OrderBookClient<Channel>,
    top: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(GetTopOrdersRequest { top });

    let response = client.get_top_orders(request).await?.into_inner();

    println!("Bids:");
    for order in &response.bids {
        println!("Order: {:?}", order);
    }

    println!("Asks:");
    for order in &response.asks {
        println!("Order: {:?}", order);
    }

    println!("Spread: {}", response.spread);

    Ok(())
}


#[tokio::main]
pub async fn main() {
    println!("Wait Client is connecting with gRPC Server...");
    println!("Fethcing Data...");

    let channel = Channel::from_static("http://localhost:50051").connect().await.unwrap();

    let client = OrderBookClient::new(channel);

    print_top_orders(client, 10).await.unwrap();
}

/* ------------
    TEST CASES    
   ------------*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_print_top_orders() {
        let channel = Channel::from_static("http://localhost:50051").connect().await.unwrap();
        let client = OrderBookClient::new(channel);

        let result = print_top_orders(client, 5).await;
        
        assert!(result.is_ok()); // The function should complete successfully
    }
}
