use exc_orderbook::combine_orderbook::Order;
use url::Url;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::StreamExt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub bids: Vec<BidOrAsk>,
    pub asks: Vec<BidOrAsk>,
}

#[derive(Debug, Deserialize)]
pub struct BidOrAsk {
    pub price: String,
    pub size: String,
}

#[derive(Debug, Deserialize)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

// Added `pair` argument
pub async fn get_binance_order_book(order_book: Arc<Mutex<(Vec<Order>, Vec<Order>)>>, pair: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binance_url = format!("{}/ws/{}@depth20@100ms", BINANCE_WS_API, pair);

    let (mut socket, _) = connect_async(Url::parse(&binance_url).unwrap()).await.expect("Can't connect");

    println!("Connected to {} binance stream.", pair);

    loop {
        let msg = socket.next().await;
        match msg {
            Some(Ok(Message::Text(text))) => {
                let parsed: DepthStreamData = serde_json::from_str(&text).expect("Can't parse");

                let mut bids: Vec<Order> = Vec::new();
                let mut asks: Vec<Order> = Vec::new();

                for bid in parsed.bids {
                    bids.push(Order {
                        exchange: "binance".to_string(),
                        pair: pair.to_string(),
                        price: bid.price.parse().unwrap(),
                        size: bid.size.parse().unwrap(),
                    });
                }

                for ask in parsed.asks {
                    asks.push(Order {
                        exchange: "binance".to_string(),
                        pair: pair.to_string(),
                        price: ask.price.parse().unwrap(),
                        size: ask.size.parse().unwrap(),
                    });
                }

                let mut shared_order_book = order_book.lock().unwrap();
                *shared_order_book = (bids, asks);
            }
            Some(Err(e)) => {
                println!("Error in WebSocket communication: {:?}", e);
                break;
            }
            None => break,
            _ => continue,
        };
    }

    Ok(())
}

/* ------------
    TEST CASES    
   ------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_get_binance_order_book() {
        let order_book = Arc::new(Mutex::new((Vec::new(), Vec::new())));
        let order_book_clone = Arc::clone(&order_book);  // clone the Arc

        let handle = tokio::spawn(async move {
            get_binance_order_book(order_book_clone, "ethbtc").await.unwrap();
        });

        // Wait for some seconds to collect some data.
        sleep(Duration::from_secs(2)).await;

        // Cancel the get_binance_order_book task.
        handle.abort();

        let (bids, asks) = order_book.lock().unwrap().clone();

        assert!(!bids.is_empty() || !asks.is_empty(), "Binance orders are empty");
        println!("{} order book (first 5 bids and asks):", "ethbtc");
        println!("Bids: {:?}", bids.iter().take(5).collect::<Vec<_>>());
        println!("Asks: {:?}", asks.iter().take(5).collect::<Vec<_>>());
    }
}
