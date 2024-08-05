mod structs;

use abi::{AbiParser, Token};
use axum::{
    routing::post,
    Router,
    Json,
};
use ethers::prelude::*;
use std::{sync::Arc};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

use tracing_subscriber;

use structs::{CollectErc20Request, CollectEthRequest, ApiResponse, DisperseErc20Request, DisperseEthRequest};


#[tokio::main]
async fn main() {
    println!("HERE");
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/disperse-eth", post(disperse_handler_eth))
        .route("/disperse-erc20", post(disperse_handler_erc20))
        .route("/collect-eth", post(collect_handler))
        .route("/collect-erc20", post(collect_erc20_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn disperse_handler_eth(
    Json(req): Json<DisperseEthRequest>,
) -> Result<Json<ApiResponse>, String> {
    if req.to.len() != req.amounts.len() {
        return Err("Recipients and amounts length mismatch".to_string());
    }

    let client = initialize_client().await;

    let contract_address = "0xYourContractAddressHere".parse().unwrap();
    let percents = req.percents.unwrap_or(U256::zero());

    let encoded_data = encode_disperse_eth_function(req.to, req.amounts.clone(), percents);

    let tx_hash = send_transaction(client, contract_address, encoded_data, Some(sum_amounts(&req.amounts))).await;

    Ok(Json(ApiResponse { tx_hash }))
}

async fn disperse_handler_erc20(
    Json(req): Json<DisperseErc20Request>,
) -> Result<Json<ApiResponse>, String> {
    if req.to.len() != req.amounts.len() {
        return Err("Recipients and amounts length mismatch".to_string());
    }

    let client = initialize_client().await;

    let contract_address = "0xYourContractAddressHere".parse().unwrap();
    let percents = req.percents.unwrap_or(U256::zero());

    let encoded_data = encode_disperse_erc20_function(req.tokens, req.to, req.amounts, percents);

    let tx_hash = send_transaction(client, contract_address, encoded_data, None).await;

    Ok(Json(ApiResponse { tx_hash }))
}

async fn collect_handler(
    Json(req): Json<CollectEthRequest>,
) -> Result<Json<ApiResponse>, String> {
    let client = initialize_client().await;

    let contract_address = "0xYourContractAddressHere".parse().unwrap();
    let percents = req.percents.unwrap_or(U256::zero());

    let encoded_data = encode_collect_eth_function(req.amount, percents);
    let tx_hash = send_transaction(client, contract_address, encoded_data, Some(req.amount)).await;

    Ok(Json(ApiResponse { tx_hash }))
}

async fn collect_erc20_handler(
    Json(req): Json<CollectErc20Request>,
) -> Result<Json<ApiResponse>, String> {
    let client = initialize_client().await;

    let contract_address = "0xYourContractAddressHere".parse().unwrap();
    let percents = req.percents.unwrap_or(U256::zero());

    let encoded_data = encode_collect_erc20_function(req.token, req.amount, percents);
    let tx_hash = send_transaction(client, contract_address, encoded_data, None).await;

    Ok(Json(ApiResponse { tx_hash }))
}

async fn initialize_client() -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let provider = Provider::<Http>::try_from(rpc_url).expect("Invalid RPC URL");
    let wallet: LocalWallet = private_key.parse().expect("Invalid private key");
    Arc::new(SignerMiddleware::new(provider, wallet))
}

fn encode_disperse_eth_function(to: Vec<String>, amounts: Vec<U256>, percents: U256) -> Bytes {
    let abi = AbiParser::default()
        .parse(&["function disperseETH(address[] calldata to, uint256[] calldata amounts, uint256 percents)"])
        .expect("Failed to parse ABI");

    abi.function("disperseETH")
        .expect("Function not found")
        .encode_input(&[
            Token::Array(
                to.into_iter()
                    .map(|addr| Address::from_str(&addr).map(Token::Address))
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            ),
            Token::Array(amounts.into_iter().map(Token::Uint).collect()),
            Token::Uint(percents)
        ])
        .expect("Failed to encode input")
        .into()
}

fn encode_disperse_erc20_function(tokens: Vec<String>, to: Vec<String>, amounts: Vec<U256>, percents: U256) -> Bytes {
    let abi = AbiParser::default()
        .parse(&["function disperseERC20(address[] calldata tokens, address[] calldata to, uint256[] calldata amounts, uint256 percents)"])
        .expect("Failed to parse ABI");

    abi.function("disperseERC20")
        .expect("Function not found")
        .encode_input(&[
            Token::Array(
                tokens.into_iter()
                    .map(|addr| Address::from_str(&addr).map(Token::Address))
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            ),
            Token::Array(
                to.into_iter()
                    .map(|addr| Address::from_str(&addr).map(Token::Address))
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            ),
            Token::Array(amounts.into_iter().map(Token::Uint).collect()),
            Token::Uint(percents)
        ])
        .expect("Failed to encode input")
        .into()
}

fn encode_collect_eth_function(amount: U256, percents: U256) -> Bytes {
    let abi = AbiParser::default()
        .parse(&["collectETH(uint256 amount,uint256 percents)"])
        .expect("Failed to parse ABI");

    abi.function("collectETH")
        .expect("Function not found")
        .encode_input(&[
            Token::Uint(amount),
            Token::Uint(percents)
        ])
        .expect("Failed to encode input")
        .into()
}

fn encode_collect_erc20_function(token: String, amount: U256, percents: U256) -> Bytes {
    let abi = AbiParser::default()
        .parse(&["collectETH(address token, uint256 amount, uint256 percents)"])
        .expect("Failed to parse ABI");

    abi.function("collectETH")
        .expect("Function not found")
        .encode_input(&[
            Address::from_str(&token).map(Token::Address).unwrap(),
            Token::Uint(amount),
            Token::Uint(percents)
        ])
        .expect("Failed to encode input")
        .into()
}

fn sum_amounts(amounts: &Vec<U256>) -> U256 {
    let mut total = U256::zero();

    for amount in amounts.iter() {
        total = total.checked_add(*amount).expect("Overflow occurred");
    }

    total
}

async fn send_transaction(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    encoded_data: Bytes,
    value: Option<U256>
) -> String {
    let mut tx = TransactionRequest::new()
        .to(contract_address)
        .data(encoded_data);

    if let Some(v) = value {
        tx = tx.value(v);
    }

    let pending_tx = client.send_transaction(tx, None).await.expect("Failed to send transaction");

    let receipt = pending_tx.confirmations(5).await.expect("Failed to confirm transaction");

    format!("{:?}", receipt.unwrap().transaction_hash)
}