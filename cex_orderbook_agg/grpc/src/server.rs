use std::sync::Arc;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
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
    tonic::include_proto!("orderbook"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl OrderBook for MyServer {
    async fn get_top_orders(&self, request: Request<GetTopOrdersRequest>) -> Result<Response<GetTopOrdersResponse>, Status> {
        let top = request.into_inner().top as usize;

        let binance_order_book = Arc::new(Mutex::new((Vec::new(), Vec::new())));
        let bitstamp_order_book = Arc::new(Mutex::new((Vec::new(), Vec::new())));

        let binance_handle = {
            let binance_order_book = Arc::clone(&binance_order_book);
            tokio::spawn(async move {
                get_binance_order_book(binance_order_book).await.unwrap();
            })
        };

        let bitstamp_handle = {
            let bitstamp_order_book = Arc::clone(&bitstamp_order_book);
            tokio::spawn(async move {
                get_bitstamp_order_book(bitstamp_order_book).await.unwrap();
            })
        };

        sleep(Duration::from_secs(5)).await;

        binance_handle.abort();
        bitstamp_handle.abort();

        let binance_order_book = binance_order_book.lock().unwrap().clone();
        let bitstamp_order_book = bitstamp_order_book.lock().unwrap().clone();

        let combined_order_books = combine_order_books(vec![binance_order_book, bitstamp_order_book], "ethbtc");

        let mut combined_order_books_sorted = combined_order_books.clone();
        combined_order_books_sorted.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        let top_asks = combined_order_books_sorted.iter().take(min(top, combined_order_books_sorted.len())).map(|order| Order {
            id: format!("{}-{}", order.exchange, order.pair),
            price: order.price,
            size: order.size,
        }).collect::<Vec<Order>>();

        let mut combined_order_books_sorted_bids = combined_order_books.clone();
        combined_order_books_sorted_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

        let top_bids = combined_order_books_sorted_bids.iter().take(min(top, combined_order_books_sorted_bids.len())).map(|order| Order {
            id: format!("{}-{}", order.exchange, order.pair),
            price: order.price,
            size: order.size,
        }).collect::<Vec<Order>>();

        let spread = match (top_asks.first(), top_bids.first()) {
            (Some(best_ask), Some(best_bid)) => best_bid.price - best_ask.price,
            _ => 0.0,
        };

        let reply = GetTopOrdersResponse { asks: top_asks, bids: top_bids, spread };
        Ok(Response::new(reply))
    }
}


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
       use tokio::runtime::Runtime;
   
       #[test]
       fn test_get_top_orders() {
           let server = MyServer::default();
           let request = Request::new(GetTopOrdersRequest { top: 5 }); // Replace 5 with a desired number
   
           let mut rt = Runtime::new().unwrap();
   
           // Since `get_top_orders` is async, we need to block on it to get a result
           let result = rt.block_on(server.get_top_orders(request));
           
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
   
   
   