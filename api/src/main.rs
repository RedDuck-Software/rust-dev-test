mod structs;

use abi::{AbiParser, Token};
use axum::{
    routing::post,
    Router,
    Json,
};
use ethers::prelude::*;
use std::sync::Arc;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

use tracing_subscriber;

use structs::{CollectErc20Request, CollectEthRequest, ApiResponse, DisperseErc20Request, DisperseEthRequest};

#[tokio::main]
async fn main() {
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
    
    let client = initialize_client(None).await;

    let contract_address = "0x760961dCCDE54efbA8a4399C7A202C96b6E8a693".parse().unwrap();
    let amounts = req.amounts.iter().map(|s| U256::from_dec_str(s).map_err(|e| e.to_string())).collect::<Result<Vec<U256>, _>>()?;
    let percents = req.percents.map_or(Ok(U256::zero()), |p| U256::from_dec_str(&p).map_err(|e| e.to_string()))?;

    let encoded_data = encode_disperse_eth_function(req.to, amounts.clone(), percents);

    let tx_hash = send_transaction(client, contract_address, encoded_data, Some(sum_amounts(&amounts))).await;

    Ok(Json(ApiResponse { tx_hashes: vec![tx_hash] }))
}

async fn disperse_handler_erc20(
    Json(req): Json<DisperseErc20Request>,
) -> Result<Json<ApiResponse>, String> {
    if req.to.len() != req.amounts.len() {
        return Err("Recipients and amounts length mismatch".to_string());
    }

    let client = initialize_client(None).await;

    let contract_address: H160 = "0x760961dCCDE54efbA8a4399C7A202C96b6E8a693".parse().unwrap();
    let amounts = req.amounts.iter().map(|s| U256::from_dec_str(s).map_err(|e| e.to_string())).collect::<Result<Vec<U256>, _>>()?;
    let percents = req.percents.map_or(Ok(U256::zero()), |p| U256::from_dec_str(&p).map_err(|e| e.to_string()))?;

    for token_address in req.tokens.iter() {
        let approve_encoded_data = encode_approve_function(contract_address, U256::max_value());
        let approve_tx_hash = send_transaction(client.clone(), token_address.parse().unwrap(), approve_encoded_data, None).await;

        println!("Approve TX Hash: {}", approve_tx_hash);
    }

    let encoded_data = encode_disperse_erc20_function(req.tokens, req.to, amounts, percents);

    let tx_hash = send_transaction(client, contract_address, encoded_data, None).await;

    Ok(Json(ApiResponse { tx_hashes: vec![tx_hash] }))
}

async fn collect_handler(
    Json(req): Json<CollectEthRequest>,
) -> Result<Json<ApiResponse>, String> {
    let contract_address = "0x760961dCCDE54efbA8a4399C7A202C96b6E8a693".parse().unwrap();
    let amount = U256::from_dec_str(&req.amount).map_err(|e| e.to_string())?;
    let percents = req.percents.map_or(Ok(U256::zero()), |p| U256::from_dec_str(&p).map_err(|e| e.to_string()))?;

    let encoded_data = encode_collect_eth_function(amount, percents);
    let mut tx_hashes = Vec::new();

    let private_keys: Vec<String> = get_private_keys();

    for private_key in private_keys.iter() {
        let client = initialize_client(Some(private_key.clone())).await;
        let tx_hash = send_transaction(client, contract_address, encoded_data.clone(), Some(amount)).await;
        tx_hashes.push(tx_hash);
    }

    Ok(Json(ApiResponse { tx_hashes }))
}

async fn collect_erc20_handler(
    Json(req): Json<CollectErc20Request>,
) -> Result<Json<ApiResponse>, String> {
    
    let contract_address: H160 = "0x760961dCCDE54efbA8a4399C7A202C96b6E8a693".parse().unwrap();
    let amount = U256::from_dec_str(&req.amount).map_err(|e| e.to_string())?;
    let percents = req.percents.map_or(Ok(U256::zero()), |p| U256::from_dec_str(&p).map_err(|e| e.to_string()))?;

    let encoded_data = encode_collect_erc20_function(req.token.clone(), amount, percents);
    let mut tx_hashes = Vec::new();

    let private_keys: Vec<String> = get_private_keys();

    for private_key in private_keys.iter() {
        let client = initialize_client(Some(private_key.clone())).await;
        
        let approve_encoded_data = encode_approve_function(contract_address, U256::max_value());
        let approve_tx_hash = send_transaction(
            client.clone(),
            req.token.parse().unwrap(),
            approve_encoded_data,
            None
        ).await;

        println!("Approve TX Hash: {}", approve_tx_hash);

        let tx_hash = send_transaction(client, contract_address, encoded_data.clone(), Some(amount)).await;
        tx_hashes.push(tx_hash);
    }

    Ok(Json(ApiResponse { tx_hashes }))
}

async fn initialize_client(private_key: Option<String>) -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let private_key = private_key.unwrap_or(env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set"));

    let provider = Provider::<Http>::try_from(rpc_url).expect("Invalid RPC URL");
    let wallet: LocalWallet = private_key.parse().expect("Invalid private key");
    Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(11155111u64)))
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

fn encode_approve_function(spender: Address, amount: U256) -> Bytes {
    let abi = AbiParser::default()
        .parse(&["function approve(address spender, uint256 value) returns (bool)"])
        .expect("Failed to parse ABI");

    abi.function("approve")
        .expect("Function not found")
        .encode_input(&[
            ethers::abi::Token::Address(spender),
            Token::Uint(amount)
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
        .from(client.address())
        .to(contract_address)
        .data(encoded_data);

    if let Some(v) = value {
        tx = tx.value(v);
    }

    let pending_tx = client.send_transaction(tx, None).await.expect("Failed to send transaction");

    let receipt = pending_tx.confirmations(1).await.expect("Failed to confirm transaction");

    format!("{:?}", receipt.unwrap().transaction_hash)
}

fn get_private_keys() -> Vec<String> {
    let private_keys = env::var("PRIVATE_KEYS_COLLECT").expect("PRIVATE_KEYS_COLLECT must be set");

    private_keys
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()

}