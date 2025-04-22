//! Implement all the relevant logic for owner of this contract.
use crate::utils::TimestampSec;
use crate::*;
use near_sdk::{assert_one_yocto, json_types::U128, near, NearToken, Promise, StorageUsage};

impl Contract {
    /// Check how much storage taken costs and refund the left over back.
    fn internal_check_storage(&self, prev_storage: StorageUsage) {
        let storage = env::storage_usage()
            .checked_sub(prev_storage)
            .unwrap_or_default() as u128;
        let storage_cost = env::storage_byte_cost().checked_mul(storage).unwrap();

        let msg = format!(
            "ERR_STORAGE_DEPOSIT need {}, attatched {}",
            storage_cost,
            env::attached_deposit()
        );
        let refund = env::attached_deposit()
            .checked_sub(storage_cost)
            .expect(&msg);
        if refund > NearToken::from_yoctonear(0) {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

#[near]
impl Contract {
    /// Change owner. Only can be called by owner.
    #[payable]
    pub fn set_owner(&mut self, owner_id: String) {
        let owner_id: AccountId = owner_id.parse().expect("ERR_OWNER_ID_IS_INVALID");
        self.assert_owner();
        assert_one_yocto();
        self.data_mut().owner_id = owner_id.clone();
    }

    /// Get the owner of this account.
    pub fn get_owner(&self) -> AccountId {
        self.data().owner_id.clone()
    }

    #[payable]
    pub fn add_account(
        &mut self,
        account_id: String,
        start_timestamp: TimestampSec,
        session_interval: TimestampSec,
        session_num: u32,
        release_per_session: U128,
    ) -> bool {
        let account_id: AccountId = account_id.parse().expect("ERR_ACCOUNT_ID_IS_INVALID");
        let prev_storage = env::storage_usage();
        self.assert_owner();
        let ret = self.internal_add_account(
            account_id,
            start_timestamp,
            session_interval,
            session_num,
            release_per_session,
        );
        self.internal_check_storage(prev_storage);
        ret
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.data().owner_id,
            "ERR_NOT_ALLOWED"
        );
    }

    /// Migration function.
    /// For next version upgrades, change this function.
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        prev
    }
}
