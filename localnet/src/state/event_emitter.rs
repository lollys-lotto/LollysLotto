use anchor_lang::prelude::Pubkey;
use lollys_lotto::state::EventEmitter;
use solana_devtools_localnet::GeneratedAccount;

pub struct TestEventEmitter;
impl GeneratedAccount for TestEventEmitter {
    type Data = EventEmitter;

    fn address(&self) -> Pubkey {
        EventEmitter::address()
    }

    fn generate(&self) -> Self::Data {
        EventEmitter { event_id: 0 }
    }

    fn owner(&self) -> Pubkey {
        lollys_lotto::ID
    }
}
