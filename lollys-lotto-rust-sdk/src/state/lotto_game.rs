pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{nonblocking::rpc_client::RpcClient as NonBlockingRpcClient, rpc_client},
};
use lollys_lotto::state::LottoGame;

use crate::{
    error::Result,
    utils::{get_state, get_state_blocking},
};

pub async fn get_lotto_game(address: &Pubkey, client: &NonBlockingRpcClient) -> Result<LottoGame> {
    get_state(address, client, "LottoGame").await
}

pub async fn get_lotto_game_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<LottoGame> {
    get_state_blocking(address, client, "LottoGame")
}
