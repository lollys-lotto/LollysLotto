use crate::TestAdmin;
use lollys_lotto_rust_sdk::constants::LOLLY_MINT;
use solana_devtools_localnet::{localnet_account::Mint, GeneratedAccount};
use solana_sdk::pubkey::Pubkey;

pub struct TestLolly;

impl GeneratedAccount for TestLolly {
    type Data = Mint;

    fn address(&self) -> Pubkey {
        LOLLY_MINT
    }

    fn generate(&self) -> Self::Data {
        Mint::new(Some(TestAdmin.address()), 100_000_000_000, 6)
    }

    fn owner(&self) -> Pubkey {
        spl_token::ID
    }

}
