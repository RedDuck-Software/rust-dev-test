# How to deploy DisperseCollect smart contract:

1. Select rpc url of the chain to deploy
2. Get private key of deployer address
3. Get etherscan api key to verify contract
4. If forge is not installed, install it by this guide: https://book.getfoundry.sh/getting-started/installation
5. To deploy contract: run "forge create --rpc-url <"your https rpc url"> --private-key <"your private key"> --etherscan-api-key <"your etherscan api key"> --verify src/DisperseCollect.sol:DisperseCollect" in ./disperse-collect-contract folder
6. To verify contract: run "forge verify-contract <"contract address"> ./src/DisperseCollect.sol:DisperseCollect --chain-id <"chain id"> --etherscan-api-key <"your etherscan api key">" in ./disperse-collect-contract folder

Now your contract is deployed and verified!

Current last version of the deployed contract on Sepolia Eth: https://sepolia.etherscan.io/address/0x760961dCCDE54efbA8a4399C7A202C96b6E8a693

# How to run Rust API

1. cd api
2. Create .env file by .env.example
3. Put all values there:
   3.1 RPC_URL: Rpc url of the Sepolia Eth
   3.2 PRIVATE_KEY: Private key of the address to disperse funds
   3.3 PRIVATE_KEYS_COLLECT: Private keys of the addresses to collect funds from, example: PRIVATE_KEYS_COLLECT="key1,key2,key3"
4. Run "cargo build"
5. Run "cargo run --package api --bin api" to start an API

Now your api is started on localhost:3000.

Main endpoints:

1. POST /disperse-eth. This endpoint is used to disperse ETH from PRIVATE_KEY in .env to passed addresses, see DisperseEthRequest
2. POST /disperse-erc20. This endpoint is used to disperse ERC20 tokens from PRIVATE_KEY in .env to passed addresses, see DisperseErc20Request
3. POST /collect-eth. This endpoint is used to collect ETH from PRIVATE_KEYS_COLLECT in .env to collect address of the contract, see CollectEthRequest
4. POST /collect-erc20. This endpoint is used to collect ERC20 tokens from PRIVATE_KEYS_COLLECT in .env to collect address of the contract, see CollectErc20Request

Also, all endpoints have "percents" as an optional parameter. This parameter determines whether we want to send all "amount" or only "percents" of this amount. Max value on contract is 10000 (100%). For example, if we want to send 22.5% of the amount, we pass "2250" as "percents".
