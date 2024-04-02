use lazy_static::lazy_static;
use lollys_lotto::{state::{LollyBurnState, LollysLotto, LottoGame, LottoTicket, UserMetadata}, Pubkey};
use solana_devtools_localnet::{
    GeneratedAccount, 
    LocalnetConfiguration, 
    TransactionSimulator,
    localnet_account::TokenAccount,
};
use std::{ops::Deref, sync::Mutex};

use lolly_lotto_localnet::{
    mints::TestUsdc, 
    primary_localnet, 
    state::{
        TestEventEmitter, 
        // TestUserMetadata
    }, 
    traits::HasMockRuntime, 
    user_accounts::{TestAdmin, TestUser}, 
    TestUserUsdc,
};


pub const DEFAULT_MAX_SLOT_PRICE_STALENESS: u8 = 11;

lazy_static! {
    pub static ref PRIMARY_LOCALNET_CONFIG: Mutex<LocalnetConfiguration> =
        Mutex::new(test_config());
}

/// Add programs in the test file where the path is determined.
fn test_config() -> LocalnetConfiguration {
    primary_localnet()
        .unwrap()
        .program_binary_file(lollys_lotto::ID, "../target/deploy/lollys_lotto.so")
        .unwrap()
}


/// All the addresses needed to run tests,
/// and some getters for blockchain state.
///

pub struct TestState {
    pub runtime: TransactionSimulator,
    pub test_event_emitter: Pubkey,
    pub test_admin: Pubkey,
    pub test_usdc: Pubkey,
    pub test_user: Pubkey,
    pub test_user_usdc: Pubkey,
    // pub test_user_metadata: Pubkey,
    pub lollys_lotto: Pubkey,
}


impl HasMockRuntime for TestState {
    fn runtime(&self) -> &TransactionSimulator {
        &self.runtime
    }

    fn payer(&self) -> Pubkey {
        TestUser.address()
    }
}

impl TestState {
    pub fn new() -> Self{
        let test_event_emitter = TestEventEmitter.address();
        let test_admin = TestAdmin.address();
        let test_usdc = TestUsdc.address();
        let test_user = TestUser.address();
        let test_user_usdc = TestUserUsdc.address();
        // let test_user_metadata = TestUserMetadata.address();
        let runtime = TryInto::<TransactionSimulator>::try_into(
            PRIMARY_LOCALNET_CONFIG.lock().unwrap().deref(),
        )
        .unwrap();
        runtime.update_clock(Some(DEFAULT_MAX_SLOT_PRICE_STALENESS as u64 + 1), None);
        println!("test_admin: {:?}", test_admin);
        println!("test_user: {:?}", test_user);
        println!("lollys_lotto: LollysLotto::address(test_admin): {:?}", LollysLotto::address(test_admin));
        println!("test_event_emitter: {:?}", test_event_emitter);
        Self {
            runtime,
            test_event_emitter,
            test_admin,
            test_usdc,
            test_user,  
            test_user_usdc,
            // test_user_metadata,
            lollys_lotto: LollysLotto::address(test_admin),
        }
    }

    pub fn get_lollys_lotto(&self, lollys_lotto_pubkey: Pubkey) -> LollysLotto {
        self.get_account_as::<LollysLotto>(&lollys_lotto_pubkey)
            .expect("couldn't find Lollys Lotto account")
    }

    pub fn get_lotto_game(&self, lotto_game_pubkey: Pubkey) -> LottoGame {
        self.get_account_as::<LottoGame>(&lotto_game_pubkey)
            .expect("couldn't find Lotto Game account")
    }
    
    pub fn get_user_metadata(&self, user_metadata_pubkey: Pubkey) -> UserMetadata {
        self.get_account_as::<UserMetadata>(&user_metadata_pubkey)
            .expect("couldn't find User Metadata account")
    }

    pub fn get_lotto_ticket(&self, lotto_ticket_pubkey: Pubkey) -> LottoTicket {
        self.get_account_as::<LottoTicket>(&lotto_ticket_pubkey)
            .expect("couldn't find Lotto Ticket account")
    }

    pub fn get_lolly_burn_state(&self, lolly_burn_state_pubkey: Pubkey) -> LollyBurnState {
        self.get_account_as::<LollyBurnState>(&lolly_burn_state_pubkey)
            .expect("couldn't find Lolly Burn State account")
    }

    pub fn get_ata_balance(&self, lotto_game_vault_pubkey: Pubkey) -> u64 {
        let state: TokenAccount = self.get_account_as(&lotto_game_vault_pubkey)
            .expect("Could not find Lotto Game Vault account");
        state.amount
    }

}
