use crate::mints::usdc::TestUsdc;
use anchor_lang::prelude::Pubkey;
use lazy_static::lazy_static;
use solana_devtools_localnet::{
    localnet_account::{token::TokenAccount, SystemAccount, THOUSAND_SOL},
    GeneratedAccount,
};
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::AccountState;

lazy_static! {
    pub static ref NOT_ADMIN_KEYPAIR: Keypair = Keypair::new();
}

pub struct NotAdmin;
impl GeneratedAccount for NotAdmin {
    type Data = SystemAccount;

    fn address(&self) -> Pubkey {
        NOT_ADMIN_KEYPAIR.pubkey()
    }

    fn generate(&self) -> Self::Data {
        SystemAccount
    }

    fn lamports(&self) -> u64 {
        THOUSAND_SOL
    }
}

pub struct NotAdminUsdc;
impl GeneratedAccount for NotAdminUsdc {
    type Data = TokenAccount;

    fn address(&self) -> Pubkey {
        get_associated_token_address(&NotAdmin.address(), &TestUsdc.address())
    }

    fn generate(&self) -> Self::Data {
        TokenAccount::from(spl_token::state::Account {
            mint: TestUsdc.address(),
            owner: NotAdmin.address(),
            amount: 1_000_000_000,
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
