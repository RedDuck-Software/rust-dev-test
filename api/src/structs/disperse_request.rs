use serde::Deserialize;

#[derive(Deserialize)]
pub struct DisperseEthRequest {
    pub to: Vec<String>,
    pub amounts: Vec<String>,
    pub percents: Option<String>
}

#[derive(Deserialize)]
pub struct DisperseErc20Request {
    pub tokens: Vec<String>,
    pub to: Vec<String>,
    pub amounts: Vec<String>,
    pub percents: Option<String>
}