use exc_orderbook::combine_orderbook::combine_order_books;
use exchanges::binance::get_binance_order_book;
use exchanges::bitstamp::get_bitstamp_order_book;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let binance_order_book = Arc::new(Mutex::new((Vec::new(), Vec::new())));
    let bitstamp_order_book = Arc::new(Mutex::new((Vec::new(), Vec::new())));

    // Clone before moving into async blocks
    let binance_order_book_clone = Arc::clone(&binance_order_book);
    let bitstamp_order_book_clone = Arc::clone(&bitstamp_order_book);

    // Spawn tasks
    let binance_handle = tokio::spawn(async move {
        get_binance_order_book(binance_order_book_clone).await.unwrap();
    });

    let bitstamp_handle = tokio::spawn(async move {
        get_bitstamp_order_book(bitstamp_order_book_clone).await.unwrap();
    });

    // Give it some time to collect data.
    sleep(Duration::from_secs(3)).await;

    // Cancel the tasks.
    binance_handle.abort();
    bitstamp_handle.abort();

    // Extract the fetched order books
    let binance_order_book = binance_order_book.lock().unwrap().clone();
    let bitstamp_order_book = bitstamp_order_book.lock().unwrap().clone();

    // Combine the order books
    let combined_order_books = combine_order_books(vec![binance_order_book, bitstamp_order_book], "ethbtc");

    println!("Combined order book:");
    for order in &combined_order_books {
        println!("{:?}", order);
    }

    // Sort the combined order book in ascending order by price.
    let mut combined_order_books_sorted = combined_order_books.clone();
    combined_order_books_sorted.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    // The top ten asks are the first ten orders after sorting in ascending order.
    let top_ten_asks = combined_order_books_sorted.iter().take(10).collect::<Vec<_>>();
    println!("Top 10 asks:");
    for ask in &top_ten_asks {
        println!("{:?}", ask);
    }

    // Create another sorted vector for bids
    let mut combined_order_books_sorted_bids = combined_order_books.clone();
    combined_order_books_sorted_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

    // The top ten bids are the first ten orders after sorting in descending order.
    let top_ten_bids = combined_order_books_sorted_bids.iter().take(10).collect::<Vec<_>>();
    println!("Top 10 bids:");
    for bid in &top_ten_bids {
        println!("{:?}", bid);
    }

    // The spread is the difference between the best ask price and the best bid price.
    let spread = match (top_ten_asks.first(), top_ten_bids.first()) {
        (Some(best_ask), Some(best_bid)) => best_ask.price - best_bid.price,
        _ => panic!("Unable to calculate spread"),
    };
    println!("Spread: {}", spread);
}
