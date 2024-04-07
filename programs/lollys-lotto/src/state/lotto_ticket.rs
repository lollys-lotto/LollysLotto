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
    pub numbers: [u8; 6],
    /// A flag to indicate if this ticket is the winning ticket of the round.
    pub is_winner: u16,
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
// #[repr(C)]
// pub struct LottoTicketList {
//     pub tickets: Vec<LottoTicket>,
// }

impl LottoTicket {
    pub fn address(lotto_game: Pubkey, user_metadata: Pubkey, numbers: [u8; 6]) -> Pubkey {
        Self::get_address(&[
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ])
    }

    pub fn address_with_bump(
        lotto_game: Pubkey,
        user_metadata: Pubkey,
        numbers: [u8; 6],
    ) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ])
    }
}
