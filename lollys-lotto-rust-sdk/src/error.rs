use anchor_client::solana_sdk::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum LollyLottoSDKError {
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),

    #[error("Could not deserialize {0} as type: {1}")]
    DeserializeFailure(Pubkey, String),
}

pub type Result<T> = std::result::Result<T, LollyLottoSDKError>;
