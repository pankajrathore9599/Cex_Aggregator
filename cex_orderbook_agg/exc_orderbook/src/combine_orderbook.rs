use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Order {
    pub exchange: String,
    pub pair: String,
    pub price: f64,
    pub size: f64,
}

pub fn combine_order_books<T>(order_books: T, pair: &str) -> Vec<Order>
where
    T: IntoIterator<Item = (Vec<Order>, Vec<Order>)>,
{
    let mut combined_order_book: Vec<Order> = Vec::new();

    for (bids, asks) in order_books {
        for mut order in bids {
            order.exchange = order.exchange.clone();
            order.pair = pair.to_string();
            combined_order_book.push(order);
        }
        for mut order in asks {
            order.exchange = order.exchange.clone();
            order.pair = pair.to_string();
            combined_order_book.push(order);
        }
    }

    combined_order_book
}

/* ------------
    TEST CASES    
   ------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_order_books() {
        // Mock data
        let binance_bids = vec![
            Order {
                exchange: "Binance".to_string(),
                pair: "".to_string(),
                price: 0.06339,
                size: 74.5795,
            },
            Order {
                exchange: "Binance".to_string(),
                pair: "".to_string(),
                price: 0.0634,
                size: 0.0708,
            },
            // Add more mock orders as needed
        ];

        let bitstamp_asks = vec![
            Order {
                exchange: "Bitstamp".to_string(),
                pair: "".to_string(),
                price: 0.06325351,
                size: 0.17432178,
            },
            Order {
                exchange: "Bitstamp".to_string(),
                pair: "".to_string(),
                price: 0.0632535,
                size: 0.7,
            },
            // Add more mock orders as needed
        ];

        let order_books = vec![(binance_bids, bitstamp_asks)];

        let combined_order_book = combine_order_books(order_books, "ethbtc");

        // Assertions
        assert_eq!(combined_order_book.len(), 4);
        assert_eq!(combined_order_book[0].exchange, "Binance");
        assert_eq!(combined_order_book[0].pair, "ethbtc");
        assert_eq!(combined_order_book[0].price, 0.06339);
        assert_eq!(combined_order_book[0].size, 74.5795);

        assert_eq!(combined_order_book[1].exchange, "Binance");
        assert_eq!(combined_order_book[1].pair, "ethbtc");
        assert_eq!(combined_order_book[1].price, 0.0634);
        assert_eq!(combined_order_book[1].size, 0.0708);

        assert_eq!(combined_order_book[2].exchange, "Bitstamp");
        assert_eq!(combined_order_book[2].pair, "ethbtc");
        assert_eq!(combined_order_book[2].price, 0.06325351);
        assert_eq!(combined_order_book[2].size, 0.17432178);

        assert_eq!(combined_order_book[3].exchange, "Bitstamp");
        assert_eq!(combined_order_book[3].pair, "ethbtc");
        assert_eq!(combined_order_book[3].price, 0.0632535);
        assert_eq!(combined_order_book[3].size, 0.7);
    }
}
