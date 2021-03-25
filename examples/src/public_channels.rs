extern crate bitfinex;

use bitfinex::{ errors::*, events::*, websockets::* };
use bitfinex::{ pairs::*, precision::* };

use bitfinex::api::*;
use bitfinex::currency::*;

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info(info) = event {
            println!("Platform status: {:?}, Version {}", info.platform, info.version);
        }
    }

    fn on_auth(&mut self, _event: NotificationEvent) {}

    fn on_subscribed(&mut self, event: NotificationEvent) {
        if let NotificationEvent::TradingSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        } else if let NotificationEvent::CandlesSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        } else if let NotificationEvent::RawBookSubscribed(msg) = event {
            println!("Subscribed: {:?}", msg);
        }        
    }

    fn on_data_event(&mut self, event: DataEvent) {
        if let DataEvent::TickerTradingEvent(channel, trading) = event {
            println!("Ticker Trading ({})- Bid {:?}, Ask: {}", channel, trading.bid, trading.ask);
        } else if let DataEvent::RawBookEvent(channel, raw_book) = event {
            println!("Raw book ({}) - Price {:?}, Amount: {}", channel, raw_book.price, raw_book.amount);
        } else if let DataEvent::BookTradingSnapshotEvent(channel, book) = event {
            println!("BookTradingSnapshotEvent ({}) - Size {:?}", channel, book.len());
        } else if let DataEvent::BookTradingUpdateEvent(channel, pair) = event {
            println!("BookTradingUpdateEvent ({}) - Price {:?}, Amount: {}", channel, pair.price, pair.amount);
        }
        // ... Add for all events you have subscribed (Trades, Books, ...)
    }

    fn on_error(&mut self, message: Error) {
        println!("{:?}", message);
    }    
}

fn main() {

    let api = Bitfinex::new(None, None);
    let trading_pairs = api.pairs.all_pairs();
    match trading_pairs {
        Ok(answer) => println!("pairs: {:?}", answer.len()),
        Err(e) => println!("Error: {}", e),
    }

    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    // TICKER
    //web_socket.subscribe_ticker(ALL, EventType::Trading);

    // TRADES
    //web_socket.subscribe_trades(ALL, EventType::Trading);

    // BOOKS
    //web_socket.subscribe_books(BTCUSD, EventType::Trading, P0, "F0", 1);
    //web_socket.subscribe_books(ETHBTC, EventType::Trading, P0, "F0", 1);

    //
    // // RAW BOOKS
    //web_socket.subscribe_raw_books(BTCUSD, EventType::Trading);
    //
    // // CANDLES
    // web_socket.subscribe_candles(BTCUSD, "1m");

    web_socket.event_loop().unwrap(); // check error
}
