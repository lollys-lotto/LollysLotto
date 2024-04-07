mod utils;

use lolly_lotto_localnet::{state::TestEventEmitter, traits::HasMockRuntime};
use solana_program::pubkey::Pubkey;

use lollys_lotto::state::{
    LollysLotto, LottoGame, LottoGameState, LottoGameVault, LottoTicket, UserMetadata,
};
use solana_devtools_localnet::GeneratedAccount;
use spl_associated_token_account::get_associated_token_address;

use utils::test_state::TestState;

use lollys_lotto_rust_sdk::instructions::{
    buy_lotto_ticket, crank_lotto_game_winner, create_lollys_lotto, create_user_metadata,
    start_lotto_game, test_emit_winning_numbers,
};

#[test]
fn test_create_lollys_lotto() {
    let test_state = TestState::new();
    // assert!(test_state.get_account(&test_state.lolly_lotto).is_none());
    let (lollys_lotto_pda, lollys_lotto_bump) =
        LollysLotto::address_with_bump(test_state.test_admin);
    println!("lollys_lotto_pda: {:?}", lollys_lotto_pda);
    test_state.execute([create_lollys_lotto(
        &test_state.test_admin,
        &test_state.lollys_lotto,
        &TestEventEmitter.address(),
    )]);

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    assert_eq!(lollys_lotto.authority, test_state.test_admin);
    assert_eq!(lollys_lotto.lotto_game_count, 0);
    assert_eq!(lollys_lotto.bump, lollys_lotto_bump);

    let round1: u64 = 0;
    let ticket_price1: u64 = 100000;
    let game_duration1: u64 = 86400;
    let round_name1 = "Round 1".to_string();

    let (lotto_game_pda1, lotto_game_bump1) =
        LottoGame::address_with_bump(test_state.test_admin, round1);
    let lotto_game_vault_signer1 = LottoGameVault::signer_address(lotto_game_pda1);
    let lotto_game_vault1 = LottoGameVault::vault_address(lotto_game_pda1);
    println!("lotto_game_pda1: {:?}", lotto_game_pda1);
    println!("lotto_game_vault_signer1: {:?}", lotto_game_vault_signer1);
    println!("lotto_game_vault1: {:?}", lotto_game_vault1);

    test_state.execute([start_lotto_game(
        round1,
        ticket_price1,
        game_duration1,
        round_name1,
        &test_state.test_admin,
        &test_state.lollys_lotto,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault1,
        &test_state.test_usdc,
        &test_state.test_event_emitter,
    )]);

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);

    assert_eq!(lollys_lotto.lotto_game_count, 1);
    assert_eq!(lotto_game1.bump, lotto_game_bump1);
    assert_eq!(lotto_game1.authority, test_state.test_admin);
    assert_eq!(lotto_game1.round, round1);
    assert_eq!(
        (lotto_game1.end_date - lotto_game1.start_date) as u64,
        game_duration1
    );
    assert_eq!(lotto_game1.ticket_price, ticket_price1);
    assert_eq!(lotto_game1.tickets_sold, 0);
    assert_eq!(lotto_game1.lotto_game_mint, test_state.test_usdc);
    assert_eq!(lotto_game1.lotto_game_vault, lotto_game_vault1);
    assert_eq!(lotto_game1.winning_numbers, [0, 0, 0, 0, 0, 0]);
    assert_eq!(lotto_game1.winning_ticket, Pubkey::default());
    assert_eq!(lotto_game1.state, LottoGameState::Open);

    let round2: u64 = 1;
    let ticket_price2: u64 = 100000;
    let game_duration2: u64 = 3600;
    let round_name2 = "Round 2".to_string();

    let (lotto_game_pda2, lotto_game_bump2) =
        LottoGame::address_with_bump(test_state.test_admin, round2);
    let lotto_game_vault_signer2 = LottoGameVault::signer_address(lotto_game_pda2);
    let lotto_game_vault2 = LottoGameVault::vault_address(lotto_game_pda2);
    println!("lotto_game_pda2: {:?}", lotto_game_pda2);
    println!("lotto_game_vault_signer2: {:?}", lotto_game_vault_signer2);
    println!("lotto_game_vault2: {:?}", lotto_game_vault2);

    test_state.execute([start_lotto_game(
        round2,
        ticket_price2,
        game_duration2,
        round_name2,
        &test_state.test_admin,
        &test_state.lollys_lotto,
        &lotto_game_pda2,
        &lotto_game_vault_signer2,
        &lotto_game_vault2,
        &test_state.test_usdc,
        &test_state.test_event_emitter,
    )]);

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    let lotto_game2 = test_state.get_lotto_game(lotto_game_pda2);

    assert_eq!(lollys_lotto.lotto_game_count, 2);
    assert_eq!(lotto_game2.bump, lotto_game_bump2);
    assert_eq!(lotto_game2.authority, test_state.test_admin);
    assert_eq!(lotto_game2.round, round2);
    assert_eq!(
        (lotto_game2.end_date - lotto_game2.start_date) as u64,
        game_duration2
    );
    assert_eq!(lotto_game2.ticket_price, ticket_price2);
    assert_eq!(lotto_game2.tickets_sold, 0);
    assert_eq!(lotto_game2.lotto_game_mint, test_state.test_usdc);
    assert_eq!(lotto_game2.lotto_game_vault, lotto_game_vault2);
    assert_eq!(lotto_game2.winning_numbers, [0, 0, 0, 0, 0, 0]);
    assert_eq!(lotto_game2.winning_ticket, Pubkey::default());
    assert_eq!(lotto_game2.state, LottoGameState::Open);

    let (user_metadata_pda, user_metadata_bump) =
        UserMetadata::address_with_bump(test_state.test_user);
    let user_rewards_vault =
        get_associated_token_address(&user_metadata_pda, &test_state.test_usdc);
    println!("user_metadata_pda: {:?}", user_metadata_pda);
    println!("user_rewards_vault: {:?}", user_rewards_vault);

    test_state.execute([create_user_metadata(
        &test_state.test_user,
        &user_metadata_pda,
        &test_state.test_usdc,
        &user_rewards_vault,
        &test_state.test_event_emitter,
    )]);

    let user_metadata = test_state.get_user_metadata(user_metadata_pda);

    assert_eq!(user_metadata.bump, user_metadata_bump);
    assert_eq!(user_metadata.user, test_state.test_user);
    assert_eq!(user_metadata.total_tickets_purchased, 0);
    assert_eq!(user_metadata.total_amount_won, 0);
    assert_eq!(user_metadata.total_amount_claimed, 0);
    assert_eq!(user_metadata.last_claimed_at, 0);
    assert_eq!(user_metadata.referral_count, 0);
    assert_eq!(user_metadata.referral_revenue, 0);

    let round: u64 = 0;
    let numbers1: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let (lotto_ticket_pda1, _lotto_ticket_bump1) =
        LottoTicket::address_with_bump(lotto_game_pda1, user_metadata_pda, numbers1);

    test_state.execute([buy_lotto_ticket(
        round,
        numbers1,
        &test_state.test_admin,
        &test_state.test_user,
        &user_metadata_pda,
        &test_state.test_user_usdc,
        &test_state.test_usdc,
        &lotto_game_pda1,
        &lotto_game_vault1,
        &lotto_ticket_pda1,
        &test_state.test_event_emitter,
    )]);

    let lotto_ticket1 = test_state.get_lotto_ticket(lotto_ticket_pda1);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault1);

    assert_eq!(lotto_ticket1.user, test_state.test_user);
    assert_eq!(lotto_ticket1.ticket_number, lotto_game1.tickets_sold - 1);
    assert_eq!(lotto_ticket1.lotto_game, lotto_game_pda1);
    assert_eq!(lotto_ticket1.round, 0);
    assert_eq!(lotto_ticket1.numbers, [1, 2, 3, 4, 5, 6]);
    assert_eq!(lotto_ticket1.is_winner, 0);
    assert_eq!(lotto_ticket1.prize, 0);
    assert_eq!(lotto_game1.tickets_sold, 1);
    assert_eq!(lotto_game_vault_balance1, ticket_price1);

    let round: u64 = 0;
    let numbers2: [u8; 6] = [1, 2, 3, 4, 5, 7];
    let (lotto_ticket_pda2, _lotto_ticket_bump2) =
        LottoTicket::address_with_bump(lotto_game_pda1, user_metadata_pda, numbers2);

    test_state.execute([buy_lotto_ticket(
        round,
        numbers2,
        &test_state.test_admin,
        &test_state.test_user,
        &user_metadata_pda,
        &test_state.test_user_usdc,
        &test_state.test_usdc,
        &lotto_game_pda1,
        &lotto_game_vault1,
        &lotto_ticket_pda2,
        &test_state.test_event_emitter,
    )]);

    let lotto_ticket2 = test_state.get_lotto_ticket(lotto_ticket_pda2);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault1);

    assert_eq!(lotto_ticket2.user, test_state.test_user);
    assert_eq!(lotto_ticket2.ticket_number, lotto_game1.tickets_sold - 1);
    assert_eq!(lotto_ticket2.lotto_game, lotto_game_pda1);
    assert_eq!(lotto_ticket2.round, 0);
    assert_eq!(lotto_ticket2.numbers, [1, 2, 3, 4, 5, 7]);
    assert_eq!(lotto_ticket2.is_winner, 0);
    assert_eq!(lotto_ticket2.prize, 0);
    assert_eq!(lotto_game1.tickets_sold, 2);
    assert_eq!(lotto_game_vault_balance1, ticket_price1 * 2);

    let intial_slot = test_state.runtime().working_bank().clock().slot;
    let initial_time = test_state.runtime().working_bank().clock().unix_timestamp;
    println!("Initial Slot: {}", intial_slot);
    println!("Initial Time: {}", initial_time);

    let time_advance: i64 = lotto_game1.end_date - lotto_game1.start_date;

    test_state.set_timestamp(
        test_state
            .runtime()
            .working_bank()
            .clock()
            .epoch_start_timestamp
            + time_advance,
    );

    println!("Time Advance: {}", time_advance);

    println!("Current Slot: {:?}", test_state.clock());

    let winning_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
    test_state.execute([test_emit_winning_numbers(
        winning_numbers.clone(),
        &test_state.test_admin,
        &lotto_game_pda1,
        &test_state.test_event_emitter,
    )]);

    let mut winning_numbers_array = [0; 6];
    winning_numbers_array.copy_from_slice(&winning_numbers[..6]);

    let winning_amount = ticket_price1
        .checked_mul(15)
        .unwrap()
        .checked_div(10)
        .unwrap() as u64;
    test_state.execute([crank_lotto_game_winner(
        winning_numbers_array,
        winning_amount,
        &test_state.test_admin,
        &test_state.test_user,
        &user_metadata_pda,
        &user_rewards_vault,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault1,
        &lotto_ticket_pda1,
    )]);

    let user_rewards_vault_balance = test_state.get_ata_balance(user_rewards_vault);

    let lotto_ticket1 = test_state.get_lotto_ticket(lotto_ticket_pda1);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    let user_metadata = test_state.get_user_metadata(user_metadata_pda);

    assert_eq!(lotto_ticket1.is_winner, 1);
    assert_eq!(lotto_ticket1.prize, winning_amount);
    assert_eq!(lotto_game1.state, LottoGameState::Finished);
    assert_eq!(user_metadata.total_amount_won, winning_amount);
    assert_eq!(user_rewards_vault_balance, winning_amount);

    // test_state.set_slot(test_state.runtime().working_bank().clock().slot + lotto_game.end_date as u64 - lotto_game.start_date as u64);

    // assert_eq!(true, false);
}
