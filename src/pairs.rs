use client::*;
use errors::*;
use serde_json::from_str;

pub static BTCUSD : &'static str = "BTCUSD";
pub static LTCUSD : &'static str = "LTCUSD";
pub static LTCBTC : &'static str = "LTCBTC";
pub static ETHUSD : &'static str = "ETHUSD";
pub static ETHBTC : &'static str = "ETHBTC";
pub static ETCUSD : &'static str = "ETCUSD";
pub static ETCBTC : &'static str = "ETCBTC";
pub static BFXUSD : &'static str = "BFXUSD";
pub static BFXBTC : &'static str = "BFXBTC";
pub static ZECUSD : &'static str = "ZECUSD";
pub static ZECBTC : &'static str = "ZECBTC";
pub static XMRUSD : &'static str = "XMRUSD";   
pub static XMRBTC : &'static str = "XMRBTC";
pub static RRTUSD : &'static str = "RRTUSD";
pub static RRTBTC : &'static str = "RRTBTC"; 
pub static XRPUSD : &'static str = "XRPUSD";
pub static XRPBTC : &'static str = "XRPBTC"; 
pub static EOSETH : &'static str = "EOSETH";
pub static EOSUSD : &'static str = "EOSUSD";
pub static EOSBTC : &'static str = "EOSBTC";
pub static IOTUSD : &'static str = "IOTUSD";
pub static IOTBTC : &'static str = "IOTBTC";
pub static IOTETH : &'static str = "IOTETH";
pub static IOTEUR : &'static str = "IOTEUR";
pub static BCCBTC : &'static str = "BCCBTC";
pub static BCUBTC : &'static str = "BCUBTC";
pub static BCCUSD : &'static str = "BCCUSD";
pub static BCUUSD : &'static str = "BCUUSD";
pub static GNTETH : &'static str = "GNTETH";
pub static GNTUSD : &'static str = "GNTUSD";
pub static GNTBTC : &'static str = "GNTBTC";
pub static SANETH : &'static str = "SANETH";
pub static SANUSD : &'static str = "SANUSD";
pub static SANBTC : &'static str = "SANBTC";
pub static AVTETH : &'static str = "AVTETH";
pub static AVTUSD : &'static str = "AVTUSD";
pub static AVTBTC : &'static str = "AVTBTC";
pub static QASHETH : &'static str = "QASHETH";
pub static QASHUSD : &'static str = "QASHUSD";
pub static QASHBTC : &'static str = "QASHBTC";

#[derive(Clone)]
pub struct Pairs {
    client: Client,
}

impl Pairs {
    pub fn new() -> Self {
        Pairs {
            client: Client::new(None, None),
        }
    }

    pub fn all_pairs(&self) -> Result<Vec<String>>
    {
        let ignore_list = ["USD".to_string()];

        let endpoint: String = String::from("conf/pub:list:pair:exchange");
        let data = self.client.get(endpoint, String::new())?;

        let result: Vec<Vec<String>> = from_str(data.as_str())?;
        let all_pairs = &result[0];
        let mut filtered_pairs : Vec<String> = Vec::new();
        for pair in all_pairs {
            for ignored in &ignore_list {
                if ! &pair.contains(ignored) {
                    filtered_pairs.push(pair.to_string());
                }
            }
        }
        Ok(filtered_pairs)
    }
}
