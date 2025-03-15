use crate::utils::*;
use crate::*;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
pub struct ContractInfo {
    pub version: String,
    // only onwer can manage accounts
    pub owner_id: AccountId,
    // token kept by this vault
    pub token_account_id: AccountId,
    // the total deposited amount in this vault
    pub total_balance: U128,
    // already claimed balance
    pub claimed_balance: U128,
}

#[near(serializers=[json])]
#[cfg_attr(test, derive(Deserialize, Clone))]
pub struct StorageReport {
    pub storage: U64,
    pub locking_near: U128,
}

#[derive(Clone)]
#[near(serializers = [json])]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
pub struct AccountInfo {
    pub account_id: AccountId,

    // session start time
    pub start_timestamp: TimestampSec,
    // per session lasts, eg: 90 days
    pub session_interval: TimestampSec,
    // totally how many session, eg: 1
    pub session_num: u32,
    // the session index of previous claim, start from 1
    pub last_claim_session: u32,
    // expected total_amount = session_num * release_per_session
    pub release_per_session: U128,

    pub claimed_amount: U128,
    pub deposited_amount: U128,

    pub unclaimed_amount: U128,
}

impl From<VAccount> for AccountInfo {
    fn from(vacc: VAccount) -> Self {
        match vacc {
            VAccount::Current(acc) => Self {
                account_id: acc.account_id.clone(),
                start_timestamp: acc.start_timestamp,
                session_interval: acc.session_interval,
                session_num: acc.session_num,
                last_claim_session: acc.last_claim_session,
                release_per_session: acc.release_per_session.into(),
                claimed_amount: acc.claimed_amount.into(),
                deposited_amount: acc.deposited_amount.into(),
                unclaimed_amount: acc.unclaimed_amount(env::block_timestamp()).into(),
            },
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Return contract basic info
    pub fn contract_metadata(&self) -> ContractInfo {
        let current_state = self.data();
        ContractInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            owner_id: current_state.owner_id.clone(),
            token_account_id: current_state.token_account_id.clone(),
            total_balance: current_state.total_balance.into(),
            claimed_balance: current_state.claimed_balance.into(),
        }
    }

    pub fn get_contract_storage_report(&self) -> StorageReport {
        let su: u64 = env::storage_usage();
        let locking_near: U128 = env::storage_byte_cost()
            .checked_mul(su as u128)
            .expect("ERR_INTEGER_OVERFLOW_WHEN_CALCULATING_LOCKING_NEAR")
            .as_yoctonear()
            .into();
        StorageReport {
            storage: U64(su),
            locking_near,
        }
    }

    pub fn get_account(&self, account_id: AccountId) -> Option<AccountInfo> {
        if let Some(vacc) = self.data().accounts.get(&account_id) {
            Some(vacc.into())
        } else {
            None
        }
    }

    pub fn list_accounts(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<AccountInfo> {
        let keys = self.data().accounts.keys_as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(keys.len());

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.data().accounts.get(&keys.get(index).unwrap()).unwrap())
            .map(|va| va.into())
            .collect()
    }
}
