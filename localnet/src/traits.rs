use anchor_lang::{solana_program::instruction::Instruction, AccountDeserialize, AccountSerialize};
use anyhow::anyhow;
use solana_devtools_localnet::{
    localnet_account::TokenAccount, ProcessedMessage, TransactionSimulator,
};
use solana_devtools_tx::TransactionSchema;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    clock::{Clock, Slot},
    pubkey::Pubkey,
};

/// Common test methods, including executions on positive and negative test cases,
/// and account getters.
pub trait HasMockRuntime {
    fn runtime(&self) -> &TransactionSimulator;

    fn payer(&self) -> Pubkey;

    fn execute(&self, ixs: impl Into<Vec<Instruction>>) -> ProcessedMessage {
        let message = (ixs).message(Some(&self.payer()));
        let result = self
            .runtime()
            .process_message_and_update_accounts(message)
            .unwrap();
        assert!(result.success(), "{:#?}", result);
        if std::env::var("SHOW_ALL_TEST_LOGS").is_ok() {
            println!("{:#?}", result.logs);
        }
        result
    }

    fn execute_show_logs(&self, ixs: impl Into<Vec<Instruction>>) -> ProcessedMessage {
        let message = (ixs).message(Some(&self.payer()));
        let result = self
            .runtime()
            .process_message_and_update_accounts(message)
            .unwrap();
        assert!(result.success(), "{:#?}", result);
        println!("{:#?}", result.logs);
        result
    }

    fn execute_expecting_err(
        &self,
        ixs: impl Into<Vec<Instruction>>,
        failed_instruction_index: u8,
        error_code: impl Into<u32>,
    ) -> ProcessedMessage {
        let message = (ixs).message(Some(&self.payer()));
        let result = self
            .runtime()
            .process_message_and_update_accounts(message)
            .unwrap();
        assert!(!result.success(), "{:#?}", result.logs);
        if std::env::var("SHOW_ALL_TEST_LOGS").is_ok() {
            println!("{:#?}", result.logs);
        }
        assert!(
            result
                .check_error_code(failed_instruction_index, error_code)
                .is_ok(),
            "{:?}\n{:#?}",
            result.execution_error,
            result.logs,
        );
        result
    }

    fn get_account(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        self.runtime().get_account(pubkey)
    }

    fn get_account_as<T: AccountDeserialize>(&self, pubkey: &Pubkey) -> Option<T> {
        let act = self.runtime().get_account(pubkey)?.clone();
        T::try_deserialize(&mut act.data()).ok()
    }

    fn set_new_account<T: AccountSerialize>(
        &self,
        pubkey: &Pubkey,
        data: &T,
        owner: &Pubkey,
    ) -> anyhow::Result<()> {
        if self.runtime().get_account(pubkey).is_some() {
            return Err(anyhow!("Account {} already exists", pubkey));
        }
        let mut new_data = vec![];
        data.try_serialize(&mut new_data).unwrap();
        let mut act_shared_data = AccountSharedData::new(1_000_000_000, new_data.len(), owner);
        act_shared_data.set_data_from_slice(&new_data);
        self.runtime().update_account(pubkey, &act_shared_data);
        Ok(())
    }

    fn update_account_data<T: AccountSerialize>(
        &self,
        pubkey: &Pubkey,
        data: &T,
    ) -> anyhow::Result<()> {
        let mut act = self
            .runtime()
            .get_account(pubkey)
            .ok_or(anyhow!(
                "Can't update account {} that doesn't exist",
                pubkey
            ))?
            .clone();
        let mut new_data = vec![];
        data.try_serialize(&mut new_data).unwrap();
        act.set_data_from_slice(&new_data);
        self.runtime().update_account(pubkey, &act);
        Ok(())
    }

    fn token_balance(&self, addr: &Pubkey) -> u64 {
        self.get_account_as::<TokenAccount>(&addr).unwrap().amount
    }

    fn clock(&self) -> Clock {
        self.runtime().working_bank().clock()
    }

    fn set_slot(&self, slot: Slot) {
        self.runtime().update_clock(Some(slot), None);
    }

    fn set_timestamp(&self, timestamp: i64) {
        self.runtime().update_clock(None, Some(timestamp));
    }
}
