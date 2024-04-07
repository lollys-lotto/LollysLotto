pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{nonblocking::rpc_client::RpcClient as NonBlockingRpcClient, rpc_client},
};
use lollys_lotto::state::LollysLotto;

use crate::{
    error::Result,
    utils::{get_state, get_state_blocking},
};

pub async fn get_lolly_lotto(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> Result<LollysLotto> {
    get_state(address, client, "LollysLotto").await
}

pub async fn get_lolly_lotto_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<LollysLotto> {
    get_state_blocking(address, client, "LollysLotto")
}
