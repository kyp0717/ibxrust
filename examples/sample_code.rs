use ibapi::contracts::Contract;

#[derive(Debug, Clone)]
pub enum Stage {
    Connect,
    Open,
    Hold,
    Close,
    Disconnect,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub symbol: String,
    pub contract_id: i64,
    pub position: i32,
    pub entry_price: f64,
    pub current_price: f64,
    pub contract: Option<Contract>,
    pub stage: Stage,
}

impl Trade {
    pub fn new(symbol: String) -> Self {
        Trade {
            symbol,
            contract_id: 0,
            position: 0,
            entry_price: 0.0,
            current_price: 0.0,
            contract: None,
            stage: Stage::Connect,
        }
    }

    pub fn create_contract(&mut self) {
        let contract = Contract::stock(&self.symbol);
        self.contract = Some(contract);
    }

    pub fn calculate_pnl(&self) -> f64 {
        if self.position == 0 {
            return 0.0;
        }
        (self.current_price - self.entry_price) * self.position as f64
    }

    pub fn update_price(&mut self, price: f64) {
        self.current_price = price;
    }

    pub fn open_position(&mut self, shares: i32, price: f64) {
        self.position = shares;
        self.entry_price = price;
        self.stage = Stage::Hold;
    }

    pub fn close_position(&mut self) -> f64 {
        let pnl = self.calculate_pnl();
        self.position = 0;
        self.entry_price = 0.0;
        self.stage = Stage::Close;
        pnl
    }
}
