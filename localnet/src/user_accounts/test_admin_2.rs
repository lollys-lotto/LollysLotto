use anchor_lang::prelude::Pubkey;
use lazy_static::lazy_static;
use solana_devtools_localnet::{
    localnet_account::{SystemAccount, TokenAccount, THOUSAND_SOL},
    GeneratedAccount,
};
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::AccountState;

use crate::mints::TestUsdc;

lazy_static! {
    pub static ref TEST_ADMIN_2_KEYPAIR: Keypair = Keypair::new();
}

/// Test pool registry admin, mint authority for all mints,
/// owner of fee destination accounts, etc.
pub struct TestAdmin2;
impl GeneratedAccount for TestAdmin2 {
    type Data = SystemAccount;

    fn address(&self) -> Pubkey {
        TEST_ADMIN_2_KEYPAIR.pubkey()
    }

    fn generate(&self) -> Self::Data {
        SystemAccount
    }

    fn lamports(&self) -> u64 {
        THOUSAND_SOL
    }
}

pub struct TestAdmin2Usdc;
impl GeneratedAccount for TestAdmin2Usdc {
    type Data = TokenAccount;

    fn address(&self) -> Pubkey {
        get_associated_token_address(&TestAdmin2.address(), &TestUsdc.address())
    }

    fn generate(&self) -> Self::Data {
        TokenAccount::from(spl_token::state::Account {
            mint: TestUsdc.address(),
            owner: TestAdmin2.address(),
            amount: 100_000_000_000,
            delegate: Default::default(),
            state: AccountState::Initialized,
            is_native: Default::default(),
            delegated_amount: 0,
            close_authority: Default::default(),
        })
    }

    fn owner(&self) -> Pubkey {
        spl_token::ID
    }
}
