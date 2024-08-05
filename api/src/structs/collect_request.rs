use serde::Deserialize;
use ethers::types::U256;

#[derive(Deserialize)]
pub struct CollectEthRequest {
    pub amount: U256,
    pub percents: Option<U256>
}

#[derive(Deserialize)]
pub struct CollectErc20Request {
    pub token: String,
    pub amount: U256,
    pub percents: Option<U256>
}