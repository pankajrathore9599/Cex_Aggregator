use exc_orderbook::combine_orderbook::Order;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt; // for .next()
use url::Url;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::{time::sleep, sync::futures}; // updated to async sleep
use std::time::Duration;

static BITSTAMP_WS_API: &str = "wss://ws.bitstamp.net";

pub async fn get_bitstamp_order_book(order_book: Arc<Mutex<(Vec<Order>, Vec<Order>)>>) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(BITSTAMP_WS_API)?;

    let (ws_stream, _response) = connect_async(url).await?;
    println!("Connected to bitstamp stream.");

    let (mut socket, mut receiver) = ws_stream.split();

    let subscribe_msg = json!({
        "event": "bts:subscribe",
        "data": {
            "channel": "order_book_ethbtc"
        }
    });

    futures_util::SinkExt::send(&mut socket, Message::Text(subscribe_msg.to_string())).await?;

    let mut message_count = 0; // Add this line

    loop {
        match receiver.next().await {
            Some(Ok(Message::Text(text))) => {
                if let Ok(data) = serde_json::from_str::<Value>(&text) {
                    if data["event"].as_str() == Some("data") {
                        let mut bids: Vec<Order> = Vec::new();
                        let mut asks: Vec<Order> = Vec::new();
                        let bids_data = data["data"]["bids"].as_array().unwrap();
                        let asks_data = data["data"]["asks"].as_array().unwrap();

                        for bid in bids_data.iter().take(20) {
                            let bid = bid.as_array().unwrap();
                            let price = bid[0].as_str().unwrap().parse::<f64>()?;
                            let size = bid[1].as_str().unwrap().parse::<f64>()?;
                            bids.push(Order { exchange: "Bitstamp".into(), pair: "ethbtc".into(), price, size });
                        }

                        for ask in asks_data.iter().take(20) {
                            let ask = ask.as_array().unwrap();
                            let price = ask[0].as_str().unwrap().parse::<f64>()?;
                            let size = ask[1].as_str().unwrap().parse::<f64>()?;
                            asks.push(Order { exchange: "Bitstamp".into(), pair: "ethbtc".into(), price, size });
                        }

                        let mut shared_order_book = order_book.lock().unwrap();
                        *shared_order_book = (bids, asks);

                        message_count += 1; // Update the counter after processing the message

                        if message_count >= 10 {
                            break; // This will break the loop after processing 10 messages
                        }
                    }
                }
            },
            _ => (),
        }
        sleep(Duration::from_millis(200)).await;
    }

    Ok(())
}




/* ------------
    TEST CASES    
   ------------*/   

   pub async fn simulate_get_bitstamp_order_book(
    order_book: Arc<tokio::sync::Mutex<(Vec<Order>, Vec<Order>)>>, 
    tx: Arc<tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<()>>>>
) {
    let mut message_count = 0; 

    loop {
        let mut bids: Vec<Order> = Vec::new();
        let mut asks: Vec<Order> = Vec::new();

        for _ in 0..20 {
            let price = rand::random::<f64>();
            let size = rand::random::<f64>();
            bids.push(Order { exchange: "Bitstamp".into(), pair: "ethbtc".into(), price, size });
        }

        for _ in 0..20 {
            let price = rand::random::<f64>();
            let size = rand::random::<f64>();
            asks.push(Order { exchange: "Bitstamp".into(), pair: "ethbtc".into(), price, size });
        }

        let mut shared_order_book = order_book.lock().await;
        *shared_order_book = (bids, asks);

        message_count += 1; 

        if message_count >= 10 {
            let mut tx_lock = tx.lock().await;
            if let Some(sender) = tx_lock.take() {
                let _ = sender.send(()); 
            }
            break; 
        }

        tokio::time::sleep(Duration::from_millis(400)).await;
    }
}


   
   #[tokio::test]
   async fn test_simulate_get_bitstamp_order_book() {
       let order_book = Arc::new(tokio::sync::Mutex::new((Vec::new(), Vec::new())));
       let order_book_clone = Arc::clone(&order_book);
   
       let (tx, rx) = tokio::sync::oneshot::channel();
       let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));
   
       let handle = tokio::spawn(simulate_get_bitstamp_order_book(order_book_clone, Arc::clone(&tx)));
   
       // Wait for the simulate function to generate some data.
       let _ = rx.await;
   
       let order_book_data = order_book.lock().await;
       let (bids, asks) = (&order_book_data.0.clone(), &order_book_data.1.clone());
   
       assert!(!bids.is_empty(), "Simulated Bitstamp bids is empty");
       assert!(!asks.is_empty(), "Simulated Bitstamp asks is empty");
       println!("Simulated Bitstamp order book (first 5 bids and asks):");
       println!("Bids: {:?}", bids.iter().take(5).collect::<Vec<_>>());
       println!("Asks: {:?}", asks.iter().take(5).collect::<Vec<_>>());
   }
   

