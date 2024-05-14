use lazy_static::lazy_static;
use lollys_lotto::state::{
    EventEmitter, LollyBurnState, LollysLotto, LottoGame, LottoTicket, LottoTicketNumbers,
    UserMetadata,
};
use lollys_lotto_rust_sdk::instructions::{
    burn_lolly, buy_lotto_ticket, claim_user_rewards, crank_lotto_game_closed, crank_lotto_game_winner, crank_transfer_to_buy_and_burn_vault, crank_transfer_winning_amount_to_user_rewards_vault, create_event_emitter, create_lolly_burn_state, create_lollys_lotto, create_user_metadata, start_lotto_game, test_emit_winning_numbers
};
use solana_devtools_localnet::{
    localnet_account::TokenAccount, GeneratedAccount, LocalnetConfiguration, ProcessedMessage,
    TransactionSimulator,
};
use solana_program::pubkey::Pubkey;
use std::{ops::Deref, sync::Mutex};

use lolly_lotto_localnet::{
    mints::TestUsdc,
    primary_localnet,
    traits::HasMockRuntime,
    user_accounts::{TestAdmin, TestUser1, TestUser2, TestUserUsdc1, TestUserUsdc2},
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
    pub test_admin: Pubkey,
    pub test_usdc: Pubkey,
    pub test_user1: Pubkey,
    pub test_user_usdc1: Pubkey,
    pub test_user2: Pubkey,
    pub test_user_usdc2: Pubkey,
    pub lollys_lotto: Pubkey,
}

impl HasMockRuntime for TestState {
    fn runtime(&self) -> &TransactionSimulator {
        &self.runtime
    }

    fn payer(&self) -> Pubkey {
        TestUser1.address()
    }
}

impl TestState {
    pub fn new() -> Self {
        let test_admin = TestAdmin.address();
        let test_usdc = TestUsdc.address();
        let test_user1 = TestUser1.address();
        let test_user_usdc1 = TestUserUsdc1.address();
        let test_user2 = TestUser2.address();
        let test_user_usdc2 = TestUserUsdc2.address();
        let runtime = TryInto::<TransactionSimulator>::try_into(
            PRIMARY_LOCALNET_CONFIG.lock().unwrap().deref(),
        )
        .unwrap();
        runtime.update_clock(Some(DEFAULT_MAX_SLOT_PRICE_STALENESS as u64 + 1), None);
        println!("test_admin: {:?}", test_admin);
        println!("test_user1: {:?}", test_user1);
        println!(
            "lollys_lotto: LollysLotto::address(test_admin): {:?}",
            LollysLotto::address(test_admin)
        );
        Self {
            runtime,
            test_admin,
            test_usdc,
            test_user1,
            test_user_usdc1,
            test_user2,
            test_user_usdc2,
            lollys_lotto: LollysLotto::address(test_admin),
        }
    }

    pub fn execute_transfer_spl_token(
        &self,
        amount: u64,
        from: Pubkey,
        to: Pubkey,
        authority: Pubkey,
    ) -> ProcessedMessage {
        self.execute([spl_token::instruction::transfer(
            &spl_token::ID,
            &from,
            &to,
            &authority,
            &[],
            amount,
        ).unwrap()])
    }

    pub fn get_ata_balance(&self, associated_token_address: Pubkey) -> u64 {
        let state: TokenAccount = self
            .get_account_as(&associated_token_address)
            .expect("Could not find Associated Token account");
        state.amount
    }

    pub fn get_event_emitter(&self, event_emitter_pubkey: Pubkey) -> EventEmitter {
        self.get_account_as::<EventEmitter>(&event_emitter_pubkey)
            .expect("couldn't find Event Emitter account")
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

    pub fn execute_create_event_emitter_ix(
        &self,
        event_emitter_pda: Pubkey,
        authority: Pubkey,
    ) -> ProcessedMessage {
        self.execute([create_event_emitter(&event_emitter_pda, &authority)])
    }

    pub fn execute_create_lollys_lotto_ix(
        &self,
        authority: Pubkey,
        lollys_lotto_pda: Pubkey,
        event_emitter_pda: Pubkey,
    ) -> ProcessedMessage {
        self.execute([create_lollys_lotto(
            &authority,
            &lollys_lotto_pda,
            &event_emitter_pda,
        )])
    }

    pub fn execute_start_lotto_game_ix(
        &self,
        round: u64,
        ticket_price: u64,
        game_duration: u64,
        round_name: String,
        authority: &Pubkey,
        lollys_lotto: &Pubkey,
        lotto_game: &Pubkey,
        lotto_game_vault_signer: &Pubkey,
        lotto_game_vault: &Pubkey,
        lotto_game_mint: &Pubkey,
        event_emitter_pda: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([start_lotto_game(
            round,
            ticket_price,
            game_duration,
            round_name,
            authority,
            lollys_lotto,
            lotto_game,
            lotto_game_vault_signer,
            lotto_game_vault,
            lotto_game_mint,
            event_emitter_pda,
        )])
    }

    pub fn execute_create_user_metadata_ix(
        &self,
        user: &Pubkey,
        user_metadata_pda: &Pubkey,
        usdc_mint: &Pubkey,
        user_rewards_vault: &Pubkey,
        event_emitter_pda: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([create_user_metadata(
            user,
            user_metadata_pda,
            usdc_mint,
            user_rewards_vault,
            event_emitter_pda,
        )])
    }

    pub fn execute_buy_lotto_ticket_ix(
        &self,
        round: u64,
        numbers: LottoTicketNumbers,
        authority: &Pubkey,
        user: &Pubkey,
        user_metadata_pda: &Pubkey,
        user_usdc_token_account: &Pubkey,
        lotto_game_mint: &Pubkey,
        lotto_game: &Pubkey,
        lotto_game_vault: &Pubkey,
        lotto_ticket_pda: &Pubkey,
        event_emitter_pda: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([buy_lotto_ticket(
            round,
            numbers,
            authority,
            user,
            user_metadata_pda,
            user_usdc_token_account,
            lotto_game_mint,
            lotto_game,
            lotto_game_vault,
            lotto_ticket_pda,
            event_emitter_pda,
        )])
    }

    pub fn execute_crank_lotto_game_closed_ix(
        &self,
        round: u64,
        authority: &Pubkey,
        lotto_game: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([crank_lotto_game_closed(
            round,
            authority,
            lotto_game,
            event_emitter,
        )])
    }

    pub fn execute_test_emit_winning_numbers_ix(
        &self,
        result: Vec<u8>,
        authority: &Pubkey,
        lotto_game: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([test_emit_winning_numbers(
            result,
            authority,
            lotto_game,
            event_emitter,
        )])
    }

    pub fn execute_crank_lotto_game_winner_ix(
        &self,
        round: u64,
        winning_numbers: LottoTicketNumbers,
        winning_numbers_index: [i64; 4],
        authority: &Pubkey,
        user: &Pubkey,
        user_metadata_pda: &Pubkey,
        lotto_game: &Pubkey,
        lotto_game_vault_signer: &Pubkey,
        lotto_game_vault: &Pubkey,
        lotto_ticket: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([crank_lotto_game_winner(
            round,
            winning_numbers,
            winning_numbers_index,
            authority,
            user,
            user_metadata_pda,
            lotto_game,
            lotto_game_vault_signer,
            lotto_game_vault,
            lotto_ticket,
            event_emitter,
        )])
    }

    pub fn execute_crank_transfer_winning_amount_to_user_rewards_vault_ix(
        &self,
        round: u64,
        winning_numbers: LottoTicketNumbers,
        number_of_tickets_with_duplicate_numbers: u32,
        authority: &Pubkey,
        user: &Pubkey,
        user_metadata_pda: &Pubkey,
        user_rewards_vault: &Pubkey,
        lotto_game: &Pubkey,
        lotto_game_vault_signer: &Pubkey,
        lotto_game_vault: &Pubkey,
        lotto_ticket: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([crank_transfer_winning_amount_to_user_rewards_vault(
            round,
            winning_numbers,
            number_of_tickets_with_duplicate_numbers,
            authority,
            user,
            user_metadata_pda,
            user_rewards_vault,
            lotto_game,
            lotto_game_vault_signer,
            lotto_game_vault,
            lotto_ticket,
            event_emitter,
        )])
    }

    pub fn execute_create_lolly_burn_state_ix(
        &self,
        authority: &Pubkey,
        lolly_burn_state_pda: &Pubkey,
        lolly_mint: &Pubkey,
        lolly_burn_state_lolly_vault: &Pubkey,
        usdc_mint: &Pubkey,
        lolly_burn_state_usdc_vault: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([create_lolly_burn_state(
            authority,
            lolly_burn_state_pda,
            lolly_mint,
            lolly_burn_state_lolly_vault,
            usdc_mint,
            lolly_burn_state_usdc_vault,
            event_emitter,
        )])
    }

    pub fn execute_crank_transfer_to_buy_and_burn_vault_ix(
        &self,
        round: u64,
        authority: &Pubkey,
        lotto_game: &Pubkey,
        lotto_game_vault_signer: &Pubkey,
        lotto_game_vault: &Pubkey,
        lolly_burn_state: &Pubkey,
        lolly_burn_state_usdc_vault: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([crank_transfer_to_buy_and_burn_vault(
            round,
            authority,
            lotto_game,
            lotto_game_vault_signer,
            lotto_game_vault,
            lolly_burn_state,
            lolly_burn_state_usdc_vault,
            event_emitter,
        )])
    }

    pub fn execute_claim_user_rewards_ix(
        &self,
        amount_to_be_claimed: u64,
        user: &Pubkey,
        user_usdc_token_account: &Pubkey,
        user_metadata_pda: &Pubkey,
        usdc_mint: &Pubkey,
        user_rewards_vault: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([claim_user_rewards(
            amount_to_be_claimed,
            user,
            user_usdc_token_account,
            user_metadata_pda,
            usdc_mint,
            user_rewards_vault,
            event_emitter,
        )])
    }

    pub fn execute_burn_lolly_ix(
        &self,
        lolly_mint: &Pubkey,
        authority: &Pubkey,
        lolly_burn_state: &Pubkey,
        lolly_burn_state_lolly_vault: &Pubkey,
        event_emitter: &Pubkey,
    ) -> ProcessedMessage {
        self.execute([burn_lolly(
            lolly_mint,
            authority,
            lolly_burn_state,
            lolly_burn_state_lolly_vault,
            event_emitter,
        )])
    }
}
