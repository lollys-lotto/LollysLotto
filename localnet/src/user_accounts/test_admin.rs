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
    pub static ref TEST_ADMIN_KEYPAIR: Keypair = Keypair::new();
}


/// This is only the user for which we test constructing a new pool registry.
/// [TestAdmin] is the actual admin for all pre-existing accounts.
pub struct TestAdmin;
impl GeneratedAccount for TestAdmin {
    type Data = SystemAccount;

    fn address(&self) -> Pubkey {
        TEST_ADMIN_KEYPAIR.pubkey()
    }

    fn generate(&self) -> Self::Data {
        SystemAccount
    }

    fn lamports(&self) -> u64 {
        THOUSAND_SOL
    }
}

pub struct TestAdminUsdc;
impl GeneratedAccount for TestAdminUsdc {
    type Data = TokenAccount;

    fn address(&self) -> Pubkey {
        get_associated_token_address(&TestAdmin.address(), &TestUsdc.address())
    }

    fn generate(&self) -> Self::Data {
        TokenAccount::from(spl_token::state::Account {
            mint: TestUsdc.address(),
            owner: TestAdmin.address(),
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

