pub use anchor_client::{
    anchor_lang::solana_program::pubkey::Pubkey,
    solana_client::{
        nonblocking::rpc_client::RpcClient as NonBlockingRpcClient,
        rpc_client,
    }
};
use lollys_lotto::state::EventEmitter;

use crate::{
    error::Result, 
    utils::{
        get_state_blocking, 
        get_state, 
    },
};


pub async fn get_event_emitter(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> Result<EventEmitter> {
    get_state(address, client, "EventEmitter").await
}

pub async fn get_event_emitter_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<EventEmitter> {
    get_state_blocking(address, client, "EventEmitter")
}