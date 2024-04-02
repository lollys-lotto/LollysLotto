use lollys_lotto::{pda_identifier::PDAIdentifier, state::EventEmitter};
use solana_program::pubkey::Pubkey;

pub fn get_event_emitter_pda(lolly_lotto_program_id: &Pubkey) -> Pubkey {
    let (event_emitter_pda, _) = Pubkey::find_program_address(
        &[EventEmitter::IDENT], lolly_lotto_program_id);
    event_emitter_pda
}

pub fn get_event_emitter_pda_and_bump(lolly_lotto_program_id: &Pubkey) -> (Pubkey, u8) {
    let (event_emitter_pda, event_emitter_bump) = Pubkey::find_program_address(
        &[EventEmitter::IDENT], lolly_lotto_program_id);
    (event_emitter_pda, event_emitter_bump)
}

