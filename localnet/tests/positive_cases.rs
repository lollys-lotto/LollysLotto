mod utils;

use lolly_lotto_localnet::traits::HasMockRuntime;
use solana_program::pubkey::Pubkey;

use lollys_lotto::state::{
    EventEmitter, LollysLotto, LottoGame, LottoGameState, LottoGameVersion,
    LottoGameWinningNumbers, LottoTicketNumbers, UserTier, WinningAmountDisbursedState,
    WinningNumberUpdateState,
};

use utils::test_state::TestState;

use lollys_lotto_rust_sdk::pda::{
    get_lotto_game_pda_and_bump, get_lotto_game_vault_pda,
    get_lotto_game_vault_signer_pda_and_bump, get_lotto_ticket_pda_and_bump,
    get_user_metadata_pda_and_bump, get_user_rewards_vault_address,
};

#[test]
fn test_create_lollys_lotto() {
    let test_state = TestState::new();
    // assert!(test_state.get_account(&test_state.lolly_lotto).is_none());

    // 1. Create Event Emitter Account with Admin
    let (event_emitter_pda, _event_emitter_bump) = EventEmitter::address_with_bump();
    println!("event_emitter_pda: {:?}", event_emitter_pda);

    test_state.execute_create_event_emitter_ix(event_emitter_pda, test_state.test_admin);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 0);

    // 2. Create Lollys Lotto Account
    let (lollys_lotto_pda, lollys_lotto_bump) =
        LollysLotto::address_with_bump(test_state.test_admin);
    println!("lollys_lotto_pda: {:?}", lollys_lotto_pda);

    test_state.execute_create_lollys_lotto_ix(
        test_state.test_admin,
        lollys_lotto_pda,
        event_emitter_pda,
    );

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    assert_eq!(lollys_lotto.authority, test_state.test_admin);
    assert_eq!(lollys_lotto.lotto_game_count, 0);
    assert_eq!(lollys_lotto.bump, lollys_lotto_bump);
    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 1);

    // 3. Start Lotto Game  Round 1
    let round1: u64 = 0;
    let ticket_price1: u64 = 100000;
    let game_duration1: u64 = 3600;
    let round_name1 = "Round 1".to_string();

    let (lotto_game_pda1, lotto_game_bump1) =
        get_lotto_game_pda_and_bump(&test_state.test_admin, round1);
    let (lotto_game_vault_signer1, lotto_game_vault_signer_bump1) =
        get_lotto_game_vault_signer_pda_and_bump(&lotto_game_pda1);
    let lotto_game_vault_pda1 = get_lotto_game_vault_pda(&lotto_game_pda1);
    println!("lotto_game_pda1: {:?}", lotto_game_pda1);
    println!("lotto_game_vault_signer1: {:?}", lotto_game_vault_signer1);
    println!("lotto_game_vault_pda1: {:?}", lotto_game_vault_pda1);

    test_state.execute_start_lotto_game_ix(
        round1,
        ticket_price1,
        game_duration1,
        round_name1,
        &test_state.test_admin,
        &test_state.lollys_lotto,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault_pda1,
        &test_state.test_usdc,
        &event_emitter_pda,
    );

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    assert_eq!(lollys_lotto.lotto_game_count, 1);

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game1.bump, lotto_game_bump1);
    assert_eq!(
        lotto_game1.lotto_game_vault_bump,
        lotto_game_vault_signer_bump1
    );
    assert_eq!(lotto_game1.version, LottoGameVersion::V1);
    assert_eq!(lotto_game1.state, LottoGameState::Open);
    assert_eq!(lotto_game1.authority, test_state.test_admin);
    assert_eq!(lotto_game1.round, round1);
    assert_eq!(
        (lotto_game1.end_date - lotto_game1.start_date) as u64,
        game_duration1
    );
    assert_eq!(lotto_game1.ticket_price, ticket_price1);
    assert_eq!(lotto_game1.tickets_sold, 0);
    assert_eq!(lotto_game1.lotto_game_mint, test_state.test_usdc);
    assert_eq!(lotto_game1.lotto_game_vault, lotto_game_vault_pda1);
    assert_eq!(lotto_game1.jackpot_winning_ticket, Pubkey::default());
    assert_eq!(lotto_game1.max_numbers_in_ticket, [9, 9, 9, 9, 9, 49]);
    assert_eq!(
        lotto_game1.jackpot_winning_numbers,
        LottoGameWinningNumbers::default()
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_1_WINNERS_V1]
    );
    assert_eq!(
        lotto_game1.tier_2_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_2_WINNERS_V1]
    );
    assert_eq!(
        lotto_game1.tier_3_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_3_WINNERS_V1]
    );

    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault_pda1);
    assert_eq!(lotto_game_vault_balance1, 0);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 2);

    // 4. Start Lotto Game Round 2 while Round 1 is still open
    let round2: u64 = 1;
    let ticket_price2: u64 = 100000;
    let game_duration2: u64 = 7200;
    let round_name2 = "Round 2".to_string();

    let (lotto_game_pda2, lotto_game_bump2) =
        get_lotto_game_pda_and_bump(&test_state.test_admin, round2);
    let (lotto_game_vault_signer2, lotto_game_vault_signer_bump2) =
        get_lotto_game_vault_signer_pda_and_bump(&lotto_game_pda2);
    let lotto_game_vault_pda2 = get_lotto_game_vault_pda(&lotto_game_pda2);
    println!("lotto_game_pda2: {:?}", lotto_game_pda2);
    println!("lotto_game_vault_signer2: {:?}", lotto_game_vault_signer2);
    println!("lotto_game_vault_pda2: {:?}", lotto_game_vault_pda2);

    test_state.execute_start_lotto_game_ix(
        round2,
        ticket_price2,
        game_duration2,
        round_name2,
        &test_state.test_admin,
        &test_state.lollys_lotto,
        &lotto_game_pda2,
        &lotto_game_vault_signer2,
        &lotto_game_vault_pda2,
        &test_state.test_usdc,
        &event_emitter_pda,
    );

    let lollys_lotto = test_state.get_lollys_lotto(lollys_lotto_pda);
    assert_eq!(lollys_lotto.lotto_game_count, 2);

    let lotto_game2 = test_state.get_lotto_game(lotto_game_pda2);
    assert_eq!(lotto_game2.bump, lotto_game_bump2);
    assert_eq!(
        lotto_game2.lotto_game_vault_bump,
        lotto_game_vault_signer_bump2
    );
    assert_eq!(lotto_game2.version, LottoGameVersion::V1);
    assert_eq!(lotto_game2.state, LottoGameState::Open);
    assert_eq!(lotto_game2.authority, test_state.test_admin);
    assert_eq!(lotto_game2.round, round2);
    assert_eq!(
        (lotto_game2.end_date - lotto_game2.start_date) as u64,
        game_duration2
    );
    assert_eq!(lotto_game2.ticket_price, ticket_price2);
    assert_eq!(lotto_game2.tickets_sold, 0);
    assert_eq!(lotto_game2.lotto_game_mint, test_state.test_usdc);
    assert_eq!(lotto_game2.lotto_game_vault, lotto_game_vault_pda2);
    assert_eq!(lotto_game2.jackpot_winning_ticket, Pubkey::default());
    assert_eq!(lotto_game2.max_numbers_in_ticket, [9, 9, 9, 9, 9, 49]);
    assert_eq!(
        lotto_game2.jackpot_winning_numbers,
        LottoGameWinningNumbers::default()
    );
    assert_eq!(
        lotto_game2.tier_1_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_1_WINNERS_V1]
    );
    assert_eq!(
        lotto_game2.tier_2_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_2_WINNERS_V1]
    );
    assert_eq!(
        lotto_game2.tier_3_winning_numbers,
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_3_WINNERS_V1]
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 3);

    let lotto_game_vault_balance2 = test_state.get_ata_balance(lotto_game_vault_pda2);
    assert_eq!(lotto_game_vault_balance2, 0);

    // 5. Create User Metadata for user1
    let (user_metadata_pda1, user_metadata_bump1) =
        get_user_metadata_pda_and_bump(test_state.test_user1);
    let user_rewards_vault1 = get_user_rewards_vault_address(test_state.test_user1);
    println!("user_metadata_pda1: {:?}", user_metadata_pda1);
    println!("user_rewards_vault1: {:?}", user_rewards_vault1);
    test_state.execute_create_user_metadata_ix(
        &test_state.test_user1,
        &user_metadata_pda1,
        &test_state.test_usdc,
        &user_rewards_vault1,
        &event_emitter_pda,
    );

    let user_metadata = test_state.get_user_metadata(user_metadata_pda1);
    assert_eq!(user_metadata.bump, user_metadata_bump1);
    assert_eq!(user_metadata.user, test_state.test_user1);
    assert_eq!(
        user_metadata.created_timestamp,
        test_state.clock().unix_timestamp
    );
    assert_eq!(user_metadata.tier, UserTier::Bronze);
    assert_eq!(user_metadata.total_tickets_purchased, 0);
    assert_eq!(user_metadata.total_amount_won, 0);
    assert_eq!(user_metadata.total_amount_claimed, 0);
    assert_eq!(user_metadata.last_claimed_at, 0);
    assert_eq!(user_metadata.referral_count, 0);
    assert_eq!(user_metadata.referral_revenue, 0);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 4);

    // 6. Buy Lotto Ticket for user1 for Round 1
    let numbers1: LottoTicketNumbers = LottoTicketNumbers {
        number1: 1,
        number2: 2,
        number3: 3,
        number4: 4,
        number5: 5,
        jackpot_number: 6,
    };
    let (lotto_ticket_pda1, _lotto_ticket_bump1) =
        get_lotto_ticket_pda_and_bump(lotto_game_pda1, user_metadata_pda1, numbers1);
    let prev_user_usdc_balance1 = test_state.get_ata_balance(test_state.test_user_usdc1);
    println!("prev_user_usdc_balance1: {:?}", prev_user_usdc_balance1);

    test_state.execute_buy_lotto_ticket_ix(
        round1,
        numbers1,
        &test_state.test_admin,
        &test_state.test_user1,
        &user_metadata_pda1,
        &test_state.test_user_usdc1,
        &test_state.test_usdc,
        &lotto_game_pda1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda1,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game1.tickets_sold, 1);

    let lotto_ticket1 = test_state.get_lotto_ticket(lotto_ticket_pda1);
    assert_eq!(lotto_ticket1.user, test_state.test_user1);
    assert_eq!(lotto_ticket1.ticket_number, lotto_game1.tickets_sold - 1);
    assert_eq!(lotto_ticket1.lotto_game, lotto_game_pda1);
    assert_eq!(lotto_ticket1.round, 0);
    assert_eq!(lotto_ticket1.numbers, numbers1);
    assert_eq!(lotto_ticket1.ticket_price, ticket_price1);
    assert_eq!(lotto_ticket1.is_checked, 0);
    assert_eq!(lotto_ticket1.is_duplicated, 0);
    assert_eq!(lotto_ticket1.is_winner, 0);
    assert_eq!(lotto_ticket1.prize, 0);

    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault_pda1);
    assert_eq!(lotto_game_vault_balance1, ticket_price1);

    let user_rewards_vault_balance1 = test_state.get_ata_balance(user_rewards_vault1);
    assert_eq!(user_rewards_vault_balance1, 0);

    let after_user_usdc_balance1 = test_state.get_ata_balance(test_state.test_user_usdc1);
    assert_eq!(
        after_user_usdc_balance1,
        prev_user_usdc_balance1 - ticket_price1
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 5);

    // 7. Buy another Lotto Ticket for user1 for Round 1
    let numbers2: LottoTicketNumbers = LottoTicketNumbers {
        number1: 1,
        number2: 2,
        number3: 3,
        number4: 4,
        number5: 5,
        jackpot_number: 7,
    };
    let (lotto_ticket_pda2, _lotto_ticket_bump2) =
        get_lotto_ticket_pda_and_bump(lotto_game_pda1, user_metadata_pda1, numbers2);
    let prev_user_usdc_balance1 = test_state.get_ata_balance(test_state.test_user_usdc1);
    println!("prev_user_usdc_balance1: {:?}", prev_user_usdc_balance1);

    test_state.execute_buy_lotto_ticket_ix(
        round1,
        numbers2,
        &test_state.test_admin,
        &test_state.test_user1,
        &user_metadata_pda1,
        &test_state.test_user_usdc1,
        &test_state.test_usdc,
        &lotto_game_pda1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda2,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game1.tickets_sold, 2);

    let lotto_ticket2 = test_state.get_lotto_ticket(lotto_ticket_pda2);
    assert_eq!(lotto_ticket2.user, test_state.test_user1);
    assert_eq!(lotto_ticket2.ticket_number, lotto_game1.tickets_sold - 1);
    assert_eq!(lotto_ticket2.lotto_game, lotto_game_pda1);
    assert_eq!(lotto_ticket2.round, 0);
    assert_eq!(lotto_ticket2.numbers, numbers2);
    assert_eq!(lotto_ticket2.ticket_price, ticket_price1);
    assert_eq!(lotto_ticket2.is_checked, 0);
    assert_eq!(lotto_ticket2.is_duplicated, 0);
    assert_eq!(lotto_ticket2.is_winner, 0);
    assert_eq!(lotto_ticket2.prize, 0);

    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault_pda1);
    assert_eq!(lotto_game_vault_balance1, ticket_price1 * 2);

    let user_rewards_vault_balance1 = test_state.get_ata_balance(user_rewards_vault1);
    assert_eq!(user_rewards_vault_balance1, 0);

    let after_user_usdc_balance1 = test_state.get_ata_balance(test_state.test_user_usdc1);
    assert_eq!(
        after_user_usdc_balance1,
        prev_user_usdc_balance1 - ticket_price1
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 6);

    // 8. Create User Metadata for user2
    let (user_metadata_pda2, user_metadata_bump2) =
        get_user_metadata_pda_and_bump(test_state.test_user2);
    let user_rewards_vault2 = get_user_rewards_vault_address(test_state.test_user2);
    println!("user_metadata_pda2: {:?}", user_metadata_pda2);
    println!("user_rewards_vault2: {:?}", user_rewards_vault2);

    test_state.execute_create_user_metadata_ix(
        &test_state.test_user2,
        &user_metadata_pda2,
        &test_state.test_usdc,
        &user_rewards_vault2,
        &event_emitter_pda,
    );

    let user_metadata2 = test_state.get_user_metadata(user_metadata_pda2);
    assert_eq!(user_metadata2.bump, user_metadata_bump2);
    assert_eq!(user_metadata2.user, test_state.test_user2);
    assert_eq!(
        user_metadata2.created_timestamp,
        test_state.clock().unix_timestamp
    );
    assert_eq!(user_metadata2.tier, UserTier::Bronze);
    assert_eq!(user_metadata2.total_tickets_purchased, 0);
    assert_eq!(user_metadata2.total_amount_won, 0);
    assert_eq!(user_metadata2.total_amount_claimed, 0);
    assert_eq!(user_metadata2.last_claimed_at, 0);
    assert_eq!(user_metadata2.referral_count, 0);
    assert_eq!(user_metadata2.referral_revenue, 0);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 7);

    // 9. Buy Lotto Ticket for user2 for Round 1
    let numbers3: LottoTicketNumbers = LottoTicketNumbers {
        number1: 1,
        number2: 2,
        number3: 3,
        number4: 4,
        number5: 5,
        jackpot_number: 8,
    };
    let (lotto_ticket_pda3, _lotto_ticket_bump3) =
        get_lotto_ticket_pda_and_bump(lotto_game_pda1, user_metadata_pda2, numbers3);
    let prev_user_usdc_balance2 = test_state.get_ata_balance(test_state.test_user_usdc2);
    println!("prev_user_usdc_balance2: {:?}", prev_user_usdc_balance2);

    test_state.execute_buy_lotto_ticket_ix(
        round1,
        numbers3,
        &test_state.test_admin,
        &test_state.test_user2,
        &user_metadata_pda2,
        &test_state.test_user_usdc2,
        &test_state.test_usdc,
        &lotto_game_pda1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda3,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game1.tickets_sold, 3);

    let lotto_ticket3 = test_state.get_lotto_ticket(lotto_ticket_pda3);
    assert_eq!(lotto_ticket3.user, test_state.test_user2);
    assert_eq!(lotto_ticket3.ticket_number, lotto_game1.tickets_sold - 1);
    assert_eq!(lotto_ticket3.lotto_game, lotto_game_pda1);
    assert_eq!(lotto_ticket3.round, 0);
    assert_eq!(lotto_ticket3.numbers, numbers3);
    assert_eq!(lotto_ticket3.ticket_price, ticket_price1);
    assert_eq!(lotto_ticket3.is_checked, 0);
    assert_eq!(lotto_ticket3.is_duplicated, 0);
    assert_eq!(lotto_ticket3.is_winner, 0);
    assert_eq!(lotto_ticket3.prize, 0);

    let lotto_game_vault_balance1 = test_state.get_ata_balance(lotto_game_vault_pda1);
    assert_eq!(lotto_game_vault_balance1, ticket_price1 * 3);

    let user_rewards_vault_balance2 = test_state.get_ata_balance(user_rewards_vault2);
    assert_eq!(user_rewards_vault_balance2, 0);

    let after_user_usdc_balance2 = test_state.get_ata_balance(test_state.test_user_usdc2);
    assert_eq!(
        after_user_usdc_balance2,
        prev_user_usdc_balance2 - ticket_price1
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 8);

    // 10. Move the time to end_date of Round 1
    let intial_slot = test_state.runtime().working_bank().clock().slot;
    let initial_time = test_state.runtime().working_bank().clock().unix_timestamp;
    println!("Initial Slot: {}", intial_slot);
    println!("Initial Time: {}", initial_time);

    let time_advance: i64 = lotto_game1.end_date - lotto_game1.start_date + 1;

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

    // 11. Close Round 1
    test_state.execute_crank_lotto_game_closed_ix(
        round1,
        &test_state.test_admin,
        &lotto_game_pda1,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game1.state, LottoGameState::Closed);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 9);

    // 12. Emit Jackpot Winning Numbers for Round 1
    let winning_numbers1: Vec<u8> = vec![
        numbers1.number1,
        numbers1.number2,
        numbers1.number3,
        numbers1.number4,
        numbers1.number5,
        numbers1.jackpot_number,
    ];
    test_state.execute_test_emit_winning_numbers_ix(
        winning_numbers1.clone(),
        &test_state.test_admin,
        &lotto_game_pda1,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.number1,
        winning_numbers1[0]
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.number2,
        winning_numbers1[1]
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.number3,
        winning_numbers1[2]
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.number4,
        winning_numbers1[3]
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.number5,
        winning_numbers1[4]
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.jackpot_number,
        winning_numbers1[5]
    );
    assert_eq!(
        lotto_game1
            .jackpot_winning_numbers
            .switchboard_random_numbers_updated,
        WinningNumberUpdateState::Updated
    );
    assert_eq!(
        lotto_game1.jackpot_winning_numbers.winning_amount_disbursed,
        WinningAmountDisbursedState::NotDisbursed
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 10);

    // 13. Emit Tier 1 Winning Numbers for Round 1
    let winning_numbers2: Vec<u8> = vec![
        numbers3.number1,
        numbers3.number2,
        numbers3.number3,
        numbers3.number4,
        numbers3.number5,
        numbers3.jackpot_number,
    ];
    test_state.execute_test_emit_winning_numbers_ix(
        winning_numbers2.clone(),
        &test_state.test_admin,
        &lotto_game_pda1,
        &event_emitter_pda,
    );

    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].number1,
        winning_numbers2[0]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].number2,
        winning_numbers2[1]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].number3,
        winning_numbers2[2]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].number4,
        winning_numbers2[3]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].number5,
        winning_numbers2[4]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].jackpot_number,
        winning_numbers2[5]
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].switchboard_random_numbers_updated,
        WinningNumberUpdateState::Updated
    );
    assert_eq!(
        lotto_game1.tier_1_winning_numbers[0].winning_amount_disbursed,
        WinningAmountDisbursedState::NotDisbursed
    );

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 11);

    // 14. Crank LottoGame Winning Numbers for Round 1
    let winning_numbers_array1: LottoTicketNumbers = numbers1;
    let winning_numbers_index1 = [0, -1, -1, -1];

    test_state.execute_crank_lotto_game_winner_ix(
        round1,
        winning_numbers_array1,
        winning_numbers_index1,
        &test_state.test_admin,
        &test_state.test_user1,
        &user_metadata_pda1,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda1,
        &event_emitter_pda,
    );

    let lotto_ticket1 = test_state.get_lotto_ticket(lotto_ticket_pda1);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    let jackpot_winning_amount = lotto_game1.final_jackpot_winning_amount().unwrap();
    assert_eq!(lotto_ticket1.is_winner, 1);
    assert_eq!(lotto_ticket1.prize, jackpot_winning_amount);
    assert_eq!(lotto_game1.state, LottoGameState::Closed);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 12);

    // 15. Crank LottoGame Winning Numbers for Round 1
    let winning_numbers_array2: LottoTicketNumbers = numbers3;
    let winning_numbers_index2 = [-1, 0, -1, -1];

    test_state.execute_crank_lotto_game_winner_ix(
        round1,
        winning_numbers_array2,
        winning_numbers_index2,
        &test_state.test_admin,
        &test_state.test_user2,
        &user_metadata_pda2,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda3,
        &event_emitter_pda,
    );

    let lotto_ticket3 = test_state.get_lotto_ticket(lotto_ticket_pda3);
    let lotto_game1 = test_state.get_lotto_game(lotto_game_pda1);
    let tier_1_winning_amount = lotto_game1.final_tier_1_winning_amount().unwrap();
    assert_eq!(lotto_ticket3.is_winner, 1);
    assert_eq!(lotto_ticket3.prize, tier_1_winning_amount);
    assert_eq!(lotto_game1.state, LottoGameState::Closed);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 13);

    // 16. Crank Transfer Winning Amount to User Rewards Vault of Jackpot winner
    let prev_user_rewards_vault_balance1 = test_state.get_ata_balance(user_rewards_vault1);
    println!("prev_user_rewards_vault_balance1: {:?}", prev_user_rewards_vault_balance1);
    let prev_user_metadata_total_amount_won = user_metadata.total_amount_won; 
    test_state.execute_crank_transfer_winning_amount_to_user_rewards_vault_ix(
        round1,
        numbers1,
        0,
        &test_state.test_admin,
        &test_state.test_user1,
        &user_metadata_pda1,
        &user_rewards_vault1,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda1,
        &event_emitter_pda,
    );

    let after_user_rewards_vault_balance1 = test_state.get_ata_balance(user_rewards_vault1);
    assert_eq!(
        after_user_rewards_vault_balance1,
        prev_user_rewards_vault_balance1 + jackpot_winning_amount
    );

    let lotto_game = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game.state, LottoGameState::Closed);
    assert_eq!(lotto_game.jackpot_winning_numbers.winning_amount_disbursed, WinningAmountDisbursedState::Disbursed);
    assert_eq!(lotto_game.tier_1_winning_numbers[0].winning_amount_disbursed, WinningAmountDisbursedState::NotDisbursed);

    let lotto_ticket = test_state.get_lotto_ticket(lotto_ticket_pda1);
    assert_eq!(lotto_ticket.prize, jackpot_winning_amount);
    assert_eq!(lotto_ticket.is_winner, 1);
    assert_eq!(lotto_ticket.is_duplicated, 0);

    let user_metadata = test_state.get_user_metadata(user_metadata_pda1);
    assert_eq!(user_metadata.total_amount_won, prev_user_metadata_total_amount_won + jackpot_winning_amount);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 14);

    // 17. Crank Transfer Winning Amount to User Rewards Vault of Tier 1 winner
    let prev_user_rewards_vault_balance2 = test_state.get_ata_balance(user_rewards_vault2);
    println!("prev_user_rewards_vault_balance2: {:?}", prev_user_rewards_vault_balance2);
    let prev_user_metadata_total_amount_won = user_metadata2.total_amount_won;
    test_state.execute_crank_transfer_winning_amount_to_user_rewards_vault_ix(
        round1,
        numbers3,
        0,
        &test_state.test_admin,
        &test_state.test_user2,
        &user_metadata_pda2,
        &user_rewards_vault2,
        &lotto_game_pda1,
        &lotto_game_vault_signer1,
        &lotto_game_vault_pda1,
        &lotto_ticket_pda3,
        &event_emitter_pda,
    );

    let after_user_rewards_vault_balance2 = test_state.get_ata_balance(user_rewards_vault2);
    assert_eq!(
        after_user_rewards_vault_balance2,
        prev_user_rewards_vault_balance2 + tier_1_winning_amount
    );

    let lotto_game = test_state.get_lotto_game(lotto_game_pda1);
    assert_eq!(lotto_game.state, LottoGameState::Closed);
    assert_eq!(lotto_game.jackpot_winning_numbers.winning_amount_disbursed, WinningAmountDisbursedState::Disbursed);
    assert_eq!(lotto_game.tier_1_winning_numbers[0].winning_amount_disbursed, WinningAmountDisbursedState::Disbursed);

    let lotto_ticket = test_state.get_lotto_ticket(lotto_ticket_pda3);
    assert_eq!(lotto_ticket.prize, tier_1_winning_amount);
    assert_eq!(lotto_ticket.is_winner, 1);
    assert_eq!(lotto_ticket.is_duplicated, 0);

    let user_metadata = test_state.get_user_metadata(user_metadata_pda2);
    assert_eq!(user_metadata.total_amount_won, prev_user_metadata_total_amount_won + tier_1_winning_amount);

    let event_emitter = test_state.get_event_emitter(event_emitter_pda);
    assert_eq!(event_emitter.event_id, 15);
}
