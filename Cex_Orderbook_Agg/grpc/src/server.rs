use tonic::{transport::Server, Request, Response, Status};
use crate::orderbook::{
    order_book_server::{OrderBook, OrderBookServer},
    GetTopOrdersRequest, GetTopOrdersResponse, Order,
};
use exchanges::binance::get_binance_order_book;
use exchanges::bitstamp::get_bitstamp_order_book;
use exc_orderbook::combine_orderbook::combine_order_books;
use std::cmp::min;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl OrderBook for MyServer {
    async fn get_top_orders(&self, request: Request<GetTopOrdersRequest>) -> Result<Response<GetTopOrdersResponse>, Status> {
        let top = request.into_inner().top as usize;

        // Fetch the order books from Binance and Bitstamp
        let binance_order_book = get_binance_order_book().await.expect("Failed to get Binance order book");
        let bitstamp_order_book = get_bitstamp_order_book().await.expect("Failed to get Bitstamp order book");

        // Combine the order books
        let mut combined_order_books = combine_order_books(vec![binance_order_book, bitstamp_order_book], "ethbtc");
        combined_order_books.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        let asks = combined_order_books.iter().take(min(top, combined_order_books.len())).map(|order| Order {
            id: format!("{}-{}", order.exchange, order.pair), // Form an ID based on exchange and pair
            price: order.price,
            size: order.size,
        }).collect::<Vec<Order>>();
        
        combined_order_books.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        let bids = combined_order_books.iter().take(min(top, combined_order_books.len())).map(|order| Order {
            id: format!("{}-{}", order.exchange, order.pair), // Form an ID based on exchange and pair
            price: order.price,
            size: order.size,
        }).collect::<Vec<Order>>();

        let spread = if let (Some(top_ask), Some(top_bid)) = (asks.first(), bids.first()) {
            top_bid.price - top_ask.price // Calculation of the spread
        } else {
            0.0
        };

        let reply = GetTopOrdersResponse { asks, bids, spread };
        Ok(Response::new(reply))
    }
}

// Your imports and struct definitions...

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    println!("Server is running on -> {:?}", addr);

    let order_book_server = OrderBookServer::new(MyServer::default());

    Server::builder()
        .add_service(order_book_server)
        .serve(addr)
        .await?;

    Ok(())
}

/* ------------
    TEST CASES    
   ------------*/

#[cfg(test)]
   mod tests {
       use super::*;
       use futures::executor::block_on;
       
       #[test]
       fn test_get_top_orders() {
           let server = MyServer::default();
           let request = Request::new(GetTopOrdersRequest { top: 5 }); // Replace 5 with a desired number
   
           // Since `get_top_orders` is async, we need to block on it to get a result
           let result = block_on(server.get_top_orders(request));
           
           match result {
               Ok(response) => {
                   let orders_response = response.into_inner();
                   
                   // Here you can write your assertions, for example:
                   assert!(orders_response.asks.len() <= 5);
                   assert!(orders_response.bids.len() <= 5);
               },
               Err(status) => {
                   // This should not happen in a test case
                   panic!("Received an error: {}", status);
               },
           }
       }
   }
   