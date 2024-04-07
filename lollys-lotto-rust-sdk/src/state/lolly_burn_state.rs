pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{nonblocking::rpc_client::RpcClient as NonBlockingRpcClient, rpc_client},
};
use lollys_lotto::state::LollyBurnState;

use crate::{
    error::Result,
    utils::{get_state, get_state_blocking},
};

pub async fn get_lolly_burn_state(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> Result<LollyBurnState> {
    get_state(address, client, "LollyBurnState").await
}

pub async fn get_lolly_burn_state_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<LollyBurnState> {
    get_state_blocking(address, client, "LollyBurnState")
}
