use exc_orderbook::combine_orderbook::Order;
use tungstenite::connect;
use url::Url;
use std::time::{Instant, Duration};

use serde::{Deserialize};

// #[derive(Debug, Deserialize)]
// pub struct OfferData {
//     #[serde(deserialize_with = "de_float_from_str")]
//     pub price: f32,
//     #[serde(deserialize_with = "de_float_from_str")]
//     pub size: f32,
// }

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

pub async fn get_binance_order_book() -> Result<(Vec<Order>, Vec<Order>), Box<dyn std::error::Error>> {
    let binance_url = format!("{}/ws/ethbtc@depth20@100ms", BINANCE_WS_API);

    let (mut socket, response) =
        connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    let start_time = Instant::now();

    let mut bids: Vec<Order> = Vec::new();
    let mut asks: Vec<Order> = Vec::new();

    loop {
        if start_time.elapsed() > Duration::from_millis(1000) {
            break;
        }
        
        let msg = match socket.read_message() {
            Ok(message) => message,
            Err(err) => {
                println!("Error: {}", err);
                return Err(Box::new(err));
            }
        };

        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            tungstenite::Message::Close(_) => {
                println!("Connection closed by server");
                return Err("Connection closed by server".into());
            },
            tungstenite::Message::Ping(_) | tungstenite::Message::Pong(_) => {
                continue;
            },
            tungstenite::Message::Binary(_) => {
                println!("Received binary data, ignoring");
                continue;
            }
        };

        let parsed: DepthStreamData = serde_json::from_str(&msg).expect("Can't parse");

        for bid in parsed.bids {
            bids.push(Order {
                exchange: "binance".to_string(),
                pair: "ethbtc".to_string(),
                price: bid.price.parse().unwrap(),
                size: bid.size.parse().unwrap(),
            });
        }

        for ask in parsed.asks {
            asks.push(Order {
                exchange: "binance".to_string(),
                pair: "ethbtc".to_string(),
                price: ask.price.parse().unwrap(),
                size: ask.size.parse().unwrap(),
            });
        }
        
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
    async fn test_get_binance_order_book() {
        match get_binance_order_book().await {
            Ok((bids, asks)) => {
                assert!(!bids.is_empty() || !asks.is_empty(), "Binance orders are empty");
                println!("Binance order book (first 5 bids and asks):");
                println!("Bids: {:?}", bids.iter().take(5).collect::<Vec<_>>());
                println!("Asks: {:?}", asks.iter().take(5).collect::<Vec<_>>());
            },
            Err(e) => {
                panic!("Failed to get Binance order book: {}", e);
            }
        }
    }
}

