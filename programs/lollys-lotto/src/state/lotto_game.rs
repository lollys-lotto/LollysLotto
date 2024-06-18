use anchor_lang::{prelude::*, Result as AnchorResult};
use anchor_spl::associated_token::get_associated_token_address;
use bytemuck::{Pod, Zeroable};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

use crate::{constants::USDC_MINT_DEVNET, errors::LollysLottoError, pda_identifier::PDAIdentifier};

use super::LottoTicketNumbers;

#[account(zero_copy)]
#[derive(Debug)]
#[repr(C)]
pub struct LottoGame {
    /// The bump seed of this round/LottoGame instance.
    pub bump: u8, //1
    /// The bump seed of this round/LottoGame vault.
    pub lotto_game_vault_bump: u8, //1
    /// Version of LottoGame state,
    pub version: LottoGameVersion, //2
    /// The state of this round/LottoGame instance.
    pub state: LottoGameState, //4
    /// The authority of this LottoGame instance.
    pub authority: Pubkey, //32
    /// The round number of this LottoGame instance.
    pub round: u64, //8
    /// The start date of this LottoGame instance.
    pub start_date: i64, //8
    /// The end date of this LottoGame instance.
    pub end_date: i64, //8
    /// The price of a ticket (in USDC) for this round/LottoGame instance.
    pub ticket_price: u64, //8
    /// The total number of tickets sold for this round/LottoGame instance.
    pub tickets_sold: u64, //8
    /// The mint used for ticket sales (in USDC).
    pub lotto_game_mint: Pubkey, //32
    /// The vault where the USDC ticket sales are stored.
    pub lotto_game_vault: Pubkey, //32
    /// The winning ticket of this round/LottoGame instance.
    pub jackpot_winning_ticket: Pubkey, //32
    /// The maximum numbers in a ticket for this round/LottoGame instance (e.g. 0-9).
    /// Ticket has only 6 numbers. But the last two bytes are reserved for padding.
    pub max_numbers_in_ticket: [u8; 6], //6
    pub _padding1: [u8; 2], //2
    pub randomness_account: Pubkey, // Reference to the Switchboard randomness account
    /// The jackpot winning numbers of this round/LottoGame instance.
    /// 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set.
    /// 8th byte is an indication if jackpot_winning_amount is disbursed or not (0 = not disbursed, 1 = disbursed).
    pub jackpot_winning_numbers: LottoGameWinningNumbers, //8
    /// The tier 1 winning numbers of this round/LottoGame instance.
    /// 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
    pub tier_1_winning_numbers: [LottoGameWinningNumbers; 10], //80
    /// The tier 2 winning numbers of this round/LottoGame instance.
    /// 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
    pub tier_2_winning_numbers: [LottoGameWinningNumbers; 100], //800
    /// The tier 3 winning numbers of this round/LottoGame instance.
    /// 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
    pub tier_3_winning_numbers: [LottoGameWinningNumbers; 1000], //8000
}

impl LottoGame {
    pub const SIZE_V1: usize =
        1 + 1 + 2 + 4 + 32 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 32 + 6 + 2 + 8 + 8 + 80 + 800 + 8000; //9080

    pub const JACKPOT_WINNERS_V1: usize = 1;
    pub const MAX_TIER_1_WINNERS_V1: usize = 10;
    pub const MAX_TIER_2_WINNERS_V1: usize = 100;
    pub const MAX_TIER_3_WINNERS_V1: usize = 1000;
    // Jackpot, Tier 1, Tier 2, Tier 3
    pub const MAX_WINNING_TIERS_V1: usize = 4;
    pub const JACKPOT_WINNING_BPS: usize = 5000;
    pub const TIER_1_WINNING_BPS: usize = 1000;
    pub const TIER_2_WINNING_BPS: usize = 1000;
    pub const TIER_3_WINNING_BPS: usize = 1000;
    pub const JACKPOT_WINNING_AMOUNT_MULTIPLIER: usize = 5000; // JACKPOT_WINNING_BPS/JACKPOT_WINNERS_V1
    pub const TIER_1_WINNING_AMOUNT_MULTIPLIER: usize = 100; // TIER_1_WINNING_BPS/MAX_TIER_1_WINNERS_V1
    pub const TIER_2_WINNING_AMOUNT_MULTIPLIER: usize = 10; // TIER_2_WINNING_BPS/MAX_TIER_2_WINNERS_V1
    pub const TIER_3_WINNING_AMOUNT_MULTIPLIER: usize = 1; // TIER_3_WINNING_BPS/MAX_TIER_3_WINNERS_V1
    pub const BUY_AND_BURN_BPS: usize = 1500;
    pub const DAO_BPS: usize = 450;
    pub const PROTOCOL_FEES_BPS: usize = 50;

    pub fn signer_address(authority: Pubkey, round: u64) -> Pubkey {
        Self::get_address(&[authority.as_ref(), &round.to_le_bytes()])
    }

    pub fn address(authority: Pubkey, round: u64) -> Pubkey {
        Self::get_address(&[authority.as_ref(), &round.to_le_bytes()])
    }

    pub fn address_with_bump(authority: Pubkey, round: u64) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[authority.as_ref(), &round.to_le_bytes()])
    }

    pub fn get_tier_1_winning_numbers_by_index(
        &self,
        index: usize,
    ) -> Result<LottoGameWinningNumbers> {
        if index > LottoGame::MAX_TIER_1_WINNERS_V1 - 1 {
            return Err(LollysLottoError::InvalidWinningNumberIndex.into());
        }
        Ok(self.tier_1_winning_numbers[index])
    }

    pub fn get_tier_2_winning_numbers_by_index(
        &self,
        index: usize,
    ) -> Result<LottoGameWinningNumbers> {
        if index > LottoGame::MAX_TIER_2_WINNERS_V1 - 1 {
            return Err(LollysLottoError::InvalidWinningNumberIndex.into());
        }
        Ok(self.tier_2_winning_numbers[index])
    }

    pub fn get_tier_3_winning_numbers_by_index(
        &self,
        index: usize,
    ) -> Result<LottoGameWinningNumbers> {
        if index > LottoGame::MAX_TIER_3_WINNERS_V1 - 1 {
            return Err(LollysLottoError::InvalidWinningNumberIndex.into());
        }
        Ok(self.tier_3_winning_numbers[index])
    }

    // write a function to get the winning numbers by tier and index
    pub fn get_winning_numbers_by_tier_and_index(
        &self,
        tier: u8,
        index: usize,
    ) -> Result<LottoGameWinningNumbers> {
        match tier {
            0 => Ok(self.jackpot_winning_numbers),
            1 => self.get_tier_1_winning_numbers_by_index(index),
            2 => self.get_tier_2_winning_numbers_by_index(index),
            3 => self.get_tier_3_winning_numbers_by_index(index),
            _ => Err(LollysLottoError::InvalidWinningNumberIndex.into()),
        }
    }

    // write a function to get the index and tier when the winning_numbers are given as input params
    pub fn get_tier_and_index_by_winning_numbers(
        &self,
        winning_numbers: LottoTicketNumbers,
    ) -> Result<(u8, usize)> {
        if self
            .jackpot_winning_numbers
            .validate_winning_numbers(winning_numbers)
        {
            return Ok((0, 0));
        }

        for (index, slot) in self.tier_1_winning_numbers.iter().enumerate() {
            if slot.validate_winning_numbers(winning_numbers) {
                return Ok((1, index));
            }
        }

        for (index, slot) in self.tier_2_winning_numbers.iter().enumerate() {
            if slot.validate_winning_numbers(winning_numbers) {
                return Ok((2, index));
            }
        }

        for (index, slot) in self.tier_3_winning_numbers.iter().enumerate() {
            if slot.validate_winning_numbers(winning_numbers) {
                return Ok((3, index));
            }
        }

        Err(LollysLottoError::InvalidWinningTicket.into())
    }

    pub fn check_game_state_closed(&self) -> bool {
        self.state == LottoGameState::Closed
    }

    pub fn current_jackpot_winning_amount(&self) -> u64 {
        (LottoGame::JACKPOT_WINNING_BPS as u64)
            .checked_mul(self.ticket_price)
            .unwrap()
            .checked_div(LottoGame::JACKPOT_WINNERS_V1 as u64)
            .unwrap()
            .checked_mul(self.tickets_sold)
            .unwrap()
            .checked_div(10000)
            .unwrap()
    }

    pub fn final_jackpot_winning_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok(self.current_jackpot_winning_amount())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn current_tier_1_winning_amount(&self) -> u64 {
        (LottoGame::TIER_1_WINNING_BPS as u64)
            .checked_mul(self.ticket_price)
            .unwrap()
            .checked_div(LottoGame::MAX_TIER_1_WINNERS_V1 as u64)
            .unwrap()
            .checked_mul(self.tickets_sold)
            .unwrap()
            .checked_div(10000)
            .unwrap()
    }

    pub fn final_tier_1_winning_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok(self.current_tier_1_winning_amount())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn current_tier_2_winning_amount(&self) -> u64 {
        (LottoGame::TIER_2_WINNING_BPS as u64)
            .checked_mul(self.ticket_price)
            .unwrap()
            .checked_div(LottoGame::MAX_TIER_2_WINNERS_V1 as u64)
            .unwrap()
            .checked_mul(self.tickets_sold)
            .unwrap()
            .checked_div(10000)
            .unwrap()
    }

    pub fn final_tier_2_winning_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok(self.current_tier_2_winning_amount())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn current_tier_3_winning_amount(&self) -> u64 {
        (LottoGame::TIER_3_WINNING_BPS as u64)
            .checked_mul(self.ticket_price)
            .unwrap()
            .checked_div(LottoGame::MAX_TIER_3_WINNERS_V1 as u64)
            .unwrap()
            .checked_mul(self.tickets_sold)
            .unwrap()
            .checked_div(10000)
            .unwrap()
    }

    pub fn final_tier_3_winning_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok(self.current_tier_3_winning_amount())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn final_buy_and_burn_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok((LottoGame::BUY_AND_BURN_BPS as u64)
                .checked_mul(self.ticket_price)
                .unwrap()
                .checked_div(10000)
                .unwrap()
                .checked_mul(self.tickets_sold)
                .unwrap())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn final_dao_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok((LottoGame::DAO_BPS as u64)
                .checked_mul(self.ticket_price)
                .unwrap()
                .checked_div(10000)
                .unwrap()
                .checked_mul(self.tickets_sold)
                .unwrap())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn final_protocol_fees_amount(&self) -> Result<u64> {
        if self.check_game_state_closed() {
            Ok((LottoGame::PROTOCOL_FEES_BPS as u64)
                .checked_mul(self.ticket_price)
                .unwrap()
                .checked_div(10000)
                .unwrap()
                .checked_mul(self.tickets_sold)
                .unwrap())
        } else {
            return Err(LollysLottoError::LottoGameIsStillOpen.into());
        }
    }

    pub fn update_winning_numbers(
        &mut self,
        numbers: [u8; 6],
    ) -> AnchorResult<(bool, bool, [i64; 4])> {
        let is_updated: bool = false;
        let (is_duplicate, duplicate_indices) = self.check_duplicate(numbers);
        if is_duplicate {
            return Ok((is_duplicate, is_updated, duplicate_indices));
        }

        let (is_updated, update_indices) = self.mark_as_updated(numbers);
        Ok((is_duplicate, is_updated, update_indices))
    }

    fn check_duplicate(&self, numbers: [u8; 6]) -> (bool, [i64; 4]) {
        let mut duplicate_indices = [-1; 4]; // Indices for jackpot, tier1, tier2, tier3

        // Check the jackpot separately
        if self
            .jackpot_winning_numbers
            .switchboard_random_numbers_updated
            == WinningNumberUpdateState::Updated
            && self.jackpot_winning_numbers.number1 == numbers[0]
            && self.jackpot_winning_numbers.number2 == numbers[1]
            && self.jackpot_winning_numbers.number3 == numbers[2]
            && self.jackpot_winning_numbers.number4 == numbers[3]
            && self.jackpot_winning_numbers.number5 == numbers[4]
            && self.jackpot_winning_numbers.jackpot_number == numbers[5]
        {
            duplicate_indices[0] = 0; // Jackpot index set as 0 because there's only one jackpot
            return (true, duplicate_indices);
        }

        // Check Tier 1 for duplicates
        for (slot_index, slot) in self.tier_1_winning_numbers.iter().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::Updated
                && slot.number1 == numbers[0]
                && slot.number2 == numbers[1]
                && slot.number3 == numbers[2]
                && slot.number4 == numbers[3]
                && slot.number5 == numbers[4]
                && slot.jackpot_number == numbers[5]
            {
                duplicate_indices[1] = slot_index as i64;
                return (true, duplicate_indices);
            }
        }

        // Check Tier 2 for duplicates
        for (slot_index, slot) in self.tier_2_winning_numbers.iter().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::Updated
                && slot.number1 == numbers[0]
                && slot.number2 == numbers[1]
                && slot.number3 == numbers[2]
                && slot.number4 == numbers[3]
                && slot.number5 == numbers[4]
                && slot.jackpot_number == numbers[5]
            {
                duplicate_indices[2] = slot_index as i64;
                return (true, duplicate_indices);
            }
        }

        // Check Tier 3 for duplicates
        for (slot_index, slot) in self.tier_3_winning_numbers.iter().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::Updated
                && slot.number1 == numbers[0]
                && slot.number2 == numbers[1]
                && slot.number3 == numbers[2]
                && slot.number4 == numbers[3]
                && slot.number5 == numbers[4]
                && slot.jackpot_number == numbers[5]
            {
                duplicate_indices[3] = slot_index as i64;
                return (true, duplicate_indices);
            }
        }

        // If no duplicates are found
        (false, duplicate_indices)
    }

    fn mark_as_updated(&mut self, numbers: [u8; 6]) -> (bool, [i64; 4]) {
        let mut update_indices = [-1; 4]; // Indices for jackpot, tier1, tier2, tier3

        // Attempt to update the jackpot
        if self
            .jackpot_winning_numbers
            .switchboard_random_numbers_updated
            == WinningNumberUpdateState::NotUpdated
        {
            // Not updated
            self.jackpot_winning_numbers.number1 = numbers[0];
            self.jackpot_winning_numbers.number2 = numbers[1];
            self.jackpot_winning_numbers.number3 = numbers[2];
            self.jackpot_winning_numbers.number4 = numbers[3];
            self.jackpot_winning_numbers.number5 = numbers[4];
            self.jackpot_winning_numbers.jackpot_number = numbers[5];
            self.jackpot_winning_numbers
                .switchboard_random_numbers_updated = WinningNumberUpdateState::Updated; // Mark as updated
            update_indices[0] = 0; // Jackpot index set as 0 because there's only one jackpot
            return (true, update_indices);
        }

        // Attempt to update Tier 1
        for (slot_index, slot) in self.tier_1_winning_numbers.iter_mut().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::NotUpdated {
                // Not updated
                slot.number1 = numbers[0];
                slot.number2 = numbers[1];
                slot.number3 = numbers[2];
                slot.number4 = numbers[3];
                slot.number5 = numbers[4];
                slot.jackpot_number = numbers[5];
                slot.switchboard_random_numbers_updated = WinningNumberUpdateState::Updated; // Mark as updated
                update_indices[1] = slot_index as i64;
                return (true, update_indices);
            }
        }

        // Attempt to update Tier 2
        for (slot_index, slot) in self.tier_2_winning_numbers.iter_mut().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::NotUpdated {
                // Not updated
                slot.number1 = numbers[0];
                slot.number2 = numbers[1];
                slot.number3 = numbers[2];
                slot.number4 = numbers[3];
                slot.number5 = numbers[4];
                slot.jackpot_number = numbers[5];
                slot.switchboard_random_numbers_updated = WinningNumberUpdateState::Updated; // Mark as updated
                update_indices[2] = slot_index as i64;
                return (true, update_indices);
            }
        }

        // Attempt to update Tier 3
        for (slot_index, slot) in self.tier_3_winning_numbers.iter_mut().enumerate() {
            if slot.switchboard_random_numbers_updated == WinningNumberUpdateState::NotUpdated {
                // Not updated
                slot.number1 = numbers[0];
                slot.number2 = numbers[1];
                slot.number3 = numbers[2];
                slot.number4 = numbers[3];
                slot.number5 = numbers[4];
                slot.jackpot_number = numbers[5];
                slot.switchboard_random_numbers_updated = WinningNumberUpdateState::Updated; // Mark as updated
                update_indices[3] = slot_index as i64;
                return (true, update_indices);
            }
        }

        // If no slots were available for update
        (false, update_indices)
    }

    pub fn get_amount_to_be_disbursed(
        &mut self,
        winning_tier: i64,
        winning_number_index: i64,
        winning_numbers: LottoTicketNumbers,
        lotto_game_vault_amount: u64,
    ) -> Result<Decimal> {
        let total_winning_pool =
            Decimal::new(self.ticket_price as i64, 0) * Decimal::new(self.tickets_sold as i64, 0);

        let winning_numbers_data = match winning_tier {
            0 => &mut self.jackpot_winning_numbers,
            1..=3 => match winning_tier {
                1 => &mut self.tier_1_winning_numbers[winning_number_index as usize],
                2 => &mut self.tier_2_winning_numbers[winning_number_index as usize],
                3 => &mut self.tier_3_winning_numbers[winning_number_index as usize],
                _ => return Err(LollysLottoError::InvalidWinningTicket.into()),
            },
            _ => return Err(LollysLottoError::InvalidWinningTicket.into()),
        };

        if !winning_numbers_data.validate_if_winning_numbers_set() {
            return Err(LollysLottoError::WinningNumbersNotSet.into());
        }

        if !winning_numbers_data.validate_winning_numbers(winning_numbers) {
            return Err(LollysLottoError::InvalidWinningTicket.into());
        }

        let amount_to_be_disbursed = match winning_tier {
            0 => calculate_winning_amount(
                total_winning_pool,
                LottoGame::JACKPOT_WINNING_BPS as i64,
                1,
                lotto_game_vault_amount,
            ),
            1 => calculate_winning_amount(
                total_winning_pool,
                LottoGame::TIER_1_WINNING_BPS as i64,
                LottoGame::MAX_TIER_1_WINNERS_V1 as i64,
                lotto_game_vault_amount,
            ),
            2 => calculate_winning_amount(
                total_winning_pool,
                LottoGame::TIER_2_WINNING_BPS as i64,
                LottoGame::MAX_TIER_2_WINNERS_V1 as i64,
                lotto_game_vault_amount,
            ),
            3 => calculate_winning_amount(
                total_winning_pool,
                LottoGame::TIER_3_WINNING_BPS as i64,
                LottoGame::MAX_TIER_3_WINNERS_V1 as i64,
                lotto_game_vault_amount,
            ),
            _ => return Err(LollysLottoError::InvalidWinningTicket.into()),
        }?;

        // winning_numbers_data.winning_amount_disbursed = WinningAmountDisbursedState::Disbursed;

        Ok(amount_to_be_disbursed)
    }
}

fn calculate_winning_amount(
    total_winning_pool: Decimal,
    winning_bps: i64,
    max_winners: i64,
    vault_amount: u64,
) -> Result<Decimal> {
    let winning_pool = Decimal::new(winning_bps, 4)
        .checked_mul(Decimal::new(total_winning_pool.to_i64().unwrap(), 0))
        .ok_or(LollysLottoError::MathError)?
        .checked_div(Decimal::new(max_winners, 0))
        .ok_or(LollysLottoError::MathError)?;

    if vault_amount < winning_pool.to_u64().unwrap() {
        return Err(LollysLottoError::InsufficientFunds.into());
    } else {
        Ok(winning_pool)
    }
}

/// Compile-time size check.
// const _: [u8; 9072] = [0u8; std::mem::size_of::<LottoGame>()];

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct LottoGameWinningNumbers {
    pub number1: u8,
    pub number2: u8,
    pub number3: u8,
    pub number4: u8,
    pub number5: u8,
    pub jackpot_number: u8,
    pub switchboard_random_numbers_updated: WinningNumberUpdateState,
    pub winning_amount_disbursed: WinningAmountDisbursedState,
}
unsafe impl Pod for LottoGameWinningNumbers {}
unsafe impl Zeroable for LottoGameWinningNumbers {}

impl LottoGameWinningNumbers {
    pub const SIZE: usize = 8;
    pub fn validate_winning_numbers(&self, numbers: LottoTicketNumbers) -> bool {
        self.number1 == numbers.number1
            && self.number2 == numbers.number2
            && self.number3 == numbers.number3
            && self.number4 == numbers.number4
            && self.number5 == numbers.number5
            && self.jackpot_number == numbers.jackpot_number
    }
    pub fn validate_if_winning_numbers_set(&self) -> bool {
        self.switchboard_random_numbers_updated == WinningNumberUpdateState::Updated
    }
    pub fn validate_if_winning_amount_disbursed(&self) -> bool {
        self.winning_amount_disbursed == WinningAmountDisbursedState::Disbursed
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
#[repr(u16)]
pub enum LottoGameVersion {
    V1,
}
unsafe impl Pod for LottoGameVersion {}
unsafe impl Zeroable for LottoGameVersion {}

impl Default for LottoGameVersion {
    fn default() -> Self {
        LottoGameVersion::V1
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum LottoGameState {
    NotStarted,
    Open,
    Closed,
    Finished,
}
unsafe impl Pod for LottoGameState {}
unsafe impl Zeroable for LottoGameState {}

impl Default for LottoGameState {
    fn default() -> Self {
        LottoGameState::NotStarted
    }
}

impl PDAIdentifier for LottoGame {
    const IDENT: &'static [u8] = b"lotto-game";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum WinningNumberUpdateState {
    NotUpdated,
    Updated,
}

impl Default for WinningNumberUpdateState {
    fn default() -> Self {
        WinningNumberUpdateState::NotUpdated
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum WinningAmountDisbursedState {
    NotDisbursed,
    Disbursed,
}

impl Default for WinningAmountDisbursedState {
    fn default() -> Self {
        WinningAmountDisbursedState::NotDisbursed
    }
}

impl LottoGame {}
/// Does not need to be initialized, only to act as a signer.
/// This signer owns the USDC token account where ticket sales in USDC is stored.
#[account]
#[derive(Debug)]
pub struct LottoGameVault {}

impl PDAIdentifier for LottoGameVault {
    const IDENT: &'static [u8] = b"lotto-game-vault";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl LottoGameVault {
    /// This PDA signer's USDC associated token account.
    pub fn vault_address(lotto_game: Pubkey) -> Pubkey {
        get_associated_token_address(&Self::signer_address(lotto_game), &USDC_MINT_DEVNET)
    }

    pub fn signer_address(lotto_game: Pubkey) -> Pubkey {
        Self::get_address(&[lotto_game.as_ref()])
    }

    pub fn address(lotto_game: Pubkey) -> Pubkey {
        Self::get_address(&[lotto_game.as_ref()])
    }

    pub fn address_with_bump(lotto_game: Pubkey) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[lotto_game.as_ref()])
    }
}
