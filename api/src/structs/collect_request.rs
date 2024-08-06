
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CollectEthRequest {
    pub amount: String,
    pub percents: Option<String>
}

#[derive(Deserialize)]
pub struct CollectErc20Request {
    pub token: String,
    pub amount: String,
    pub percents: Option<String>
}