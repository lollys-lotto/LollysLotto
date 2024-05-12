use crate::instructions::*;

pub fn close_event_emitter(event_emitter: &Pubkey, authority: &Pubkey) -> Instruction {
    let data = lollys_lotto::instruction::CloseEventEmitter.data();
    let accounts = lollys_lotto::accounts::CloseEventEmitter {
        authority: *authority,
        event_emitter: *event_emitter,
        system_program: system_program::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
