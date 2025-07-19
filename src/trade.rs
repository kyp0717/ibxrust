use ibapi::contracts;
use ibapi::contracts::Contract;


enum Stage {
    Connect,
    Open,
    Hold,
    Close,
    Disconnect,
}


#[derive(Debug, Clone)]
struct Trade {
    symbol: String,
    contract_id: String,
    position: i32,
    contract: contracts::Contract,
}

impl Trade {
    fn new() -> Self {
        todo!()
    }

    fn get_contrac

    fn make_contract(&self) -> Self {
        let contract = Contract {
            contract_id: self.asparse().unwrap_or_default(),
            symbol: self.asset.symbol.clone(),
            sec_type: "STK".to_string(),
            exchange: "SMART".to_string(),
            currency: "USD".to_string(),
            ..Default::default()
        };
        
        Trade {
            asset: self.asset.clone(),
            position: self.position,
            contract,
        }
    }
}
