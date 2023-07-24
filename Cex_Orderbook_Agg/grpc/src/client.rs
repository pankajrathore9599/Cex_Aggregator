use tonic::transport::Channel;
use tonic::Request;

pub mod grpc {
    pub mod orderbook {
        tonic::include_proto!("orderbook");
    }
}

use crate::grpc::orderbook::order_book_client::OrderBookClient; // Corrected import
use crate::grpc::orderbook::GetTopOrdersRequest; // Added import

use tokio::runtime::Runtime;

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


pub fn main() {
    println!("Wait Client is connecting with gRPC Server...");
    println!("Fethcing Data...");
    let rt = Runtime::new().unwrap();
    let channel = rt.block_on(Channel::from_static("http://localhost:50051").connect()).unwrap();

    let client = OrderBookClient::new(channel);

    rt.block_on(print_top_orders(client, 10)).unwrap();
}

/* ------------
    TEST CASES    
   ------------*/

   #[cfg(test)]
   mod tests {
       use super::*;
       use futures::executor::block_on;
       
       #[test]
       fn test_print_top_orders() {
           let rt = Runtime::new().unwrap();
           let channel = rt.block_on(Channel::from_static("http://localhost:50051").connect()).unwrap();
           let client = OrderBookClient::new(channel);
   
           // Since `print_top_orders` is async, we need to block on it to get a result
           let result = block_on(print_top_orders(client, 5));
           
           assert!(result.is_ok()); // The function should complete successfully
       }
   }
   
