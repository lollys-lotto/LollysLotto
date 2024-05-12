use crate::pda_identifier::PDAIdentifier;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug, Copy)]
#[repr(C)]
pub struct LottoTicket {
    /// The user who bought this ticket.
    pub user: Pubkey,
    /// The ticket number of this ticket for the current lotto_game.
    pub ticket_number: u64,
    /// The LottoGame instance this ticket is associated with.
    pub lotto_game: Pubkey,
    /// The round number of the LottoGame instance this ticket is associated with.
    pub round: u64,
    /// The numbers the user has chosen for this ticket.
    pub numbers: LottoTicketNumbers,
    pub _padding1: [u8; 2],
    /// The price of this ticket in USDC.
    pub ticket_price: u64,
    /// The date this ticket was bought.
    pub buy_date: i64,
    /// The date this ticket was checked for winning numbers.
    pub check_date: i64,
    /// A flag to indicate if this ticket has been checked for winning numbers.
    pub is_checked: u8,
    /// A flag to indicate number of times the numbers have been duplicated.
    pub is_duplicated: u32,
    /// A flag to indicate if this ticket is the winning ticket of the round.
    pub is_winner: u8,
    pub _padding2: [u8; 2],
    /// The amount the user has been paid for this ticket if this is the winning ticket.
    pub prize: u64,
}

impl PDAIdentifier for LottoTicket {
    const IDENT: &'static [u8] = b"lotto-ticket";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

// #[account]
// #[derive(Debug, Copy)]
// pub struct LottoTicketList {
//     pub tickets: Vec<LottoTicket>,
// }

impl LottoTicket {
    pub fn address(
        lotto_game: Pubkey,
        user_metadata: Pubkey,
        numbers: LottoTicketNumbers,
    ) -> Pubkey {
        Self::get_address(&[
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers.number1.to_le_bytes().as_ref(),
            numbers.number2.to_le_bytes().as_ref(),
            numbers.number3.to_le_bytes().as_ref(),
            numbers.number4.to_le_bytes().as_ref(),
            numbers.number5.to_le_bytes().as_ref(),
            numbers.jackpot_number.to_le_bytes().as_ref(),
        ])
    }

    pub fn address_with_bump(
        lotto_game: Pubkey,
        user_metadata: Pubkey,
        numbers: LottoTicketNumbers,
    ) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers.number1.to_le_bytes().as_ref(),
            numbers.number2.to_le_bytes().as_ref(),
            numbers.number3.to_le_bytes().as_ref(),
            numbers.number4.to_le_bytes().as_ref(),
            numbers.number5.to_le_bytes().as_ref(),
            numbers.jackpot_number.to_le_bytes().as_ref(),
        ])
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct LottoTicketNumbers {
    pub number1: u8,
    pub number2: u8,
    pub number3: u8,
    pub number4: u8,
    pub number5: u8,
    pub jackpot_number: u8,
}

impl LottoTicketNumbers {
    pub const MAX_SLOT_IN_TICKET_V1: usize = 6;
    pub const MAX_NUMBERS_IN_TICKET_V1: LottoTicketNumbers = LottoTicketNumbers {
        number1: 9,
        number2: 9,
        number3: 9,
        number4: 9,
        number5: 9,
        jackpot_number: 49,
    };
}

pub fn validate_for_max_min_numbers(numbers: LottoTicketNumbers) -> bool {
    if numbers.number1 > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number1
        || numbers.number2 > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number2
        || numbers.number3 > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number3
        || numbers.number4 > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number4
        || numbers.number5 > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number5
        || numbers.jackpot_number > LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.jackpot_number
    {
        return false;
    }
    return true;
}
