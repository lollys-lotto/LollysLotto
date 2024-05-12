use crate::instructions::*;

pub fn create_event_emitter(event_emitter: &Pubkey, authority: &Pubkey) -> Instruction {
    let data = lollys_lotto::instruction::CreateEventEmitter.data();
    let accounts = lollys_lotto::accounts::CreateEventEmitter {
        funder: *authority,
        event_emitter: *event_emitter,
        system_program: system_program::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::id(),
        accounts,
        data,
    }
}
