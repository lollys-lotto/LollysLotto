pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{nonblocking::rpc_client::RpcClient as NonBlockingRpcClient, rpc_client},
};
use lollys_lotto::state::UserMetadata;

use crate::{
    error::Result,
    utils::{get_state, get_state_blocking},
};

pub async fn get_user_metadata(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> Result<UserMetadata> {
    get_state(address, client, "UserMetadata").await
}

pub async fn get_user_metadata_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<UserMetadata> {
    get_state_blocking(address, client, "UserMetadata")
}
