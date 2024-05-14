use crate::instructions::*;

pub fn close_user_metadata(
    user: &Pubkey,
    user_metadata: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseUserMetadata {}.data();

    let accounts = lollys_lotto::accounts::CloseUserMetadata {
        user: *user,
        user_metadata: *user_metadata,
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