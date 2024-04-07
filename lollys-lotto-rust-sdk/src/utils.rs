use crate::error::{self, LollyLottoSDKError};
use anchor_client::{
    anchor_lang::AccountDeserialize,
    solana_client::{nonblocking::rpc_client::RpcClient as NonBlockingRpcClient, rpc_client},
    solana_sdk::pubkey::Pubkey,
};

pub async fn get_state<T: AccountDeserialize>(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
    type_name: &str,
) -> error::Result<T> {
    let data = client
        .get_account_data(address)
        .await
        .map_err(|_| LollyLottoSDKError::AccountNotFound(address.clone()))?;
    let state = T::try_deserialize(&mut data.as_slice()).map_err(|_| {
        LollyLottoSDKError::DeserializeFailure(address.clone(), type_name.to_string())
    })?;
    Ok(state)
}

pub fn get_state_blocking<T: AccountDeserialize>(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
    type_name: &str,
) -> error::Result<T> {
    let data = client
        .get_account_data(address)
        .map_err(|_| LollyLottoSDKError::AccountNotFound(address.clone()))?;
    let state = T::try_deserialize(&mut data.as_slice()).map_err(|_| {
        LollyLottoSDKError::DeserializeFailure(address.clone(), type_name.to_string())
    })?;
    Ok(state)
}

pub async fn get_state_non_deserializable(
    address: &Pubkey,
    client: &NonBlockingRpcClient,
) -> error::Result<Vec<u8>> {
    let data = client
        .get_account_data(address)
        .await
        .map_err(|_| LollyLottoSDKError::AccountNotFound(address.clone()))?;
    Ok(data)
}

pub fn get_state_non_deserializable_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> error::Result<Vec<u8>> {
    let data = client
        .get_account_data(address)
        .map_err(|_| LollyLottoSDKError::AccountNotFound(address.clone()))?;
    Ok(data)
}
