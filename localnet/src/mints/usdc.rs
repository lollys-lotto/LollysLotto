use crate::TestUser;
use lollys_lotto_rust_sdk::constants::USDC_DEVNET_MINT;
use solana_devtools_localnet::{localnet_account::Mint, GeneratedAccount};
use solana_sdk::pubkey::Pubkey;

pub struct TestUsdc;

impl GeneratedAccount for TestUsdc {
    type Data = Mint;

    fn address(&self) -> Pubkey {
        USDC_DEVNET_MINT
    }

    fn generate(&self) -> Self::Data {
        Mint::new(Some(TestUser.address()), 0, 6)
    }

    fn owner(&self) -> Pubkey {
        spl_token::ID
    }
}
