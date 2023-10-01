use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use starknet::core::types::{BlockId, FieldElement, FunctionCall, BroadcastedTransaction};
use url::Url;

// Async wrappers for all the RPC calls. Needed because I didn't manage to make async benchmarking work in criterion
#[tokio::main]
pub async fn block_number(rpc_client: &JsonRpcClient<HttpTransport>) {
    rpc_client.block_number().await;
}
#[tokio::main]
pub async fn get_block_with_tx_hashes(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId) {
    rpc_client.get_block_with_tx_hashes(block_id).await;
}

#[tokio::main]
pub async fn get_block_with_txs(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId) {
    rpc_client.get_block_with_txs(block_id).await;
}


#[tokio::main]
pub async fn get_state_update(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId) {
    rpc_client.get_state_update(block_id).await;
}

#[tokio::main]
pub async fn get_storage_at(rpc_client: &JsonRpcClient<HttpTransport>, contract_address: FieldElement, key: FieldElement, block_id: BlockId) {
    rpc_client.get_storage_at(contract_address, key, block_id).await;
}

#[tokio::main]
pub async fn get_transaction_by_hash(rpc_client: &JsonRpcClient<HttpTransport>, hash: FieldElement) {
    rpc_client.get_transaction_by_hash(hash).await;
}

#[tokio::main]
pub async fn get_transaction_by_block_id_and_index(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId, index: u64) {
    rpc_client.get_transaction_by_block_id_and_index(block_id, index).await;
}

#[tokio::main]
pub async fn get_transaction_receipt(rpc_client: &JsonRpcClient<HttpTransport>, hash: FieldElement) {
    rpc_client.get_transaction_receipt(hash).await;
}

#[tokio::main]
pub async fn get_class(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId, hash: FieldElement) {
    rpc_client.get_class(block_id, hash).await;
}

#[tokio::main]
pub async fn get_class_hash_at(rpc_client: &JsonRpcClient<HttpTransport>, block_id: BlockId, contract_address: FieldElement) {
    rpc_client.get_class_hash_at(block_id, contract_address).await;
}

#[tokio::main]
pub async fn call(rpc_client: &JsonRpcClient<HttpTransport>, request: FunctionCall, block_id: BlockId) {
    rpc_client.call(request, block_id).await;
}

// Need moree imports for estimate fee.
// #[tokio::main]
// pub async fn estimate_fee(rpc_client: &JsonRpcClient<HttpTransport>, request: BroadcastedTransaction, block_id: BlockId) {
//     rpc_client.estimate_fee(request, block_id).await;
// }


// Get a client from a url address 
pub fn provider(address: &str) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(address)
            .unwrap(),
    ))
}

