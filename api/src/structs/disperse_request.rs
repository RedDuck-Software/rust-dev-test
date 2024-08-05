use serde::Deserialize;
use ethers::types::U256;

#[derive(Deserialize)]
pub struct DisperseEthRequest {
    pub to: Vec<String>,
    pub amounts: Vec<U256>,
    pub percents: Option<U256>
}

#[derive(Deserialize)]
pub struct DisperseErc20Request {
    pub tokens: Vec<String>,
    pub to: Vec<String>,
    pub amounts: Vec<U256>,
    pub percents: Option<U256>
}