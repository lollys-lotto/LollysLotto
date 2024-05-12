use lollys_lotto::state::{LottoTicket, LottoTicketNumbers};
use solana_program::pubkey::Pubkey;

pub fn get_lotto_ticket_pda(
    lotto_game_pda: Pubkey,
    user_metadata_pda: Pubkey,
    numbers: LottoTicketNumbers,
) -> Pubkey {
    LottoTicket::address(lotto_game_pda, user_metadata_pda, numbers)
}

pub fn get_lotto_ticket_pda_and_bump(
    lotto_game_pda: Pubkey,
    user_metadata_pda: Pubkey,
    numbers: LottoTicketNumbers,
) -> (Pubkey, u8) {
    LottoTicket::address_with_bump(lotto_game_pda, user_metadata_pda, numbers)
}
