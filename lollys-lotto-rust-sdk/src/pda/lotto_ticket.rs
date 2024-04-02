
use lollys_lotto::{pda_identifier::PDAIdentifier, state::LottoTicket};
use solana_program::pubkey::Pubkey;

pub fn get_lotto_ticket_pda(
    lotto_game: &Pubkey,
    user_metadata: &Pubkey,
    numbers: [u8; 6],
    lolly_lotto_program_id: &Pubkey
) -> Pubkey {
    let (lotto_ticket_pda, _) = Pubkey::find_program_address(
        &[
            LottoTicket::IDENT,
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ], 
        lolly_lotto_program_id
    );
    lotto_ticket_pda
}

pub fn get_lotto_ticket_pda_and_bump(
    lotto_game: &Pubkey,
    user_metadata: &Pubkey,
    numbers: [u8; 6],
    lolly_lotto_program_id: &Pubkey
) -> (Pubkey, u8) {
    let (lotto_ticket_pda, lotto_ticket_bump) = Pubkey::find_program_address(
        &[
            LottoTicket::IDENT,
            lotto_game.as_ref(),
            user_metadata.as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ], 
        lolly_lotto_program_id
    );
    (lotto_ticket_pda, lotto_ticket_bump)
}