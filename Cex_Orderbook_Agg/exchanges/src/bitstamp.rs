use exc_orderbook::combine_orderbook::Order;
use tungstenite::{connect, Message};
use url::Url;
use serde_json::{json, Value};

use std::thread::sleep;
use std::time::Duration;

static BITSTAMP_WS_API: &str = "wss://ws.bitstamp.net";

pub async fn get_bitstamp_order_book() -> Result<(Vec<Order>, Vec<Order>), Box<dyn std::error::Error>> {
    let url = Url::parse(BITSTAMP_WS_API)?;

    let (mut socket, _response) = connect(url)?;

    let subscribe_msg = json!({
        "event": "bts:subscribe",
        "data": {
            "channel": "order_book_ethbtc"
        }
    });

    socket.write_message(Message::Text(subscribe_msg.to_string()))?;

    let start_time = std::time::Instant::now();

    let mut bids: Vec<Order> = Vec::new();
    let mut asks: Vec<Order> = Vec::new();

    loop {
        if start_time.elapsed() > Duration::from_millis(1000) {
            break;
        }

        let msg = socket.read_message()?;

        match msg {
            Message::Text(text) => {
                if let Ok(data) = serde_json::from_str::<Value>(&text) {
                    if data["event"].as_str() == Some("data") {
                        let bids_data = data["data"]["bids"].as_array().unwrap();
                        let asks_data = data["data"]["asks"].as_array().unwrap();

                        for bid in bids_data.iter().take(20) {
                            let bid = bid.as_array().unwrap();
                            let price = bid[0].as_str().unwrap().parse::<f64>()?;
                            let size = bid[1].as_str().unwrap().parse::<f64>()?;
                            bids.push(Order { exchange: "Bitstamp".into(),pair: "ethbtc".into(), price, size });
                        }

                        for ask in asks_data.iter().take(20) {
                            let ask = ask.as_array().unwrap();
                            let price = ask[0].as_str().unwrap().parse::<f64>()?;
                            let size = ask[1].as_str().unwrap().parse::<f64>()?;
                            asks.push(Order { exchange: "Bitstamp".into(),pair: "ethbtc".into(), price, size });
                        }
                    }
                }
            }
            _ => (),
        }

        sleep(Duration::from_millis(100));
    }

    Ok((bids, asks))
}

/* ------------
    TEST CASES    
   ------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bitstamp_order_book() {
        match get_bitstamp_order_book().await {
            Ok((bids, asks)) => {
                assert!(bids.len() > 0, "Bitstamp bids is empty");
                assert!(asks.len() > 0, "Bitstamp asks is empty");
                println!("Bitstamp order book (first 5 bids and asks):");
                println!("Bids: {:?}", bids.iter().take(5).collect::<Vec<_>>());
                println!("Asks: {:?}", asks.iter().take(5).collect::<Vec<_>>());
            },
            Err(e) => {
                panic!("Failed to get Bitstamp order book: {}", e);
            }
        }
    }
}
