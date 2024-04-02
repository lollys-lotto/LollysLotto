pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{
        nonblocking::rpc_client::RpcClient as NonBlockingRpcClient,
        rpc_client,
    }
};
use lollys_lotto::state::LottoTicket;

use crate::{
    error::Result, 
    utils::{
        get_state_blocking, 
        get_state, 
    },
};


pub async fn get_lotto_ticket(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> Result<LottoTicket> {
    get_state(address, client, "LottoTicket").await
}

pub async fn get_lotto_ticket_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<LottoTicket> {
    get_state_blocking(address, client, "LottoTicket")
}