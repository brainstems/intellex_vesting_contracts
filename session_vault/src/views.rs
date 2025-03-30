use crate::utils::*;
use crate::*;

#[derive(Clone)]
#[near(serializers = [json])]
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
#[cfg_attr(test, derive(Clone))]
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
                release_per_session: acc.release_per_session,
                claimed_amount: acc.claimed_amount,
                deposited_amount: acc.deposited_amount,
                unclaimed_amount: acc.unclaimed_amount(env::block_timestamp()).into(),
            },
        }
    }
}

impl From<&VAccount> for AccountInfo {
    fn from(vacc: &VAccount) -> Self {
        vacc.clone().into()
    }
}

#[near]
impl Contract {
    /// Return contract basic info
    pub fn contract_metadata(&self) -> ContractInfo {
        let current_state = self.data();
        ContractInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            owner_id: current_state.owner_id.clone(),
            token_account_id: current_state.token_account_id.clone(),
            total_balance: current_state.total_balance,
            claimed_balance: current_state.claimed_balance,
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
        self.data().accounts.get(&account_id).map(Into::into)
    }

    pub fn list_accounts(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<AccountInfo> {
        let mut keys = self.data().accounts.keys();
        let from_index: usize = from_index.unwrap_or(U64(0)).0 as usize;
        let limit: usize = limit.unwrap_or(U64(keys.len() as u64)).0 as usize;

        let mut account_infos: Vec<AccountInfo> = Vec::new();
        match keys.nth(from_index) {
            None => return account_infos,
            Some(account_id) => {
                // Will always be Some
                let account_info = self.data().accounts.get(account_id).unwrap();
                account_infos.push(account_info.into());
            }
        }
        let keys = keys.enumerate();
        for (id, account) in keys {
            if id >= limit - 1 {
                break;
            }
            let account_info = self.data().accounts.get(account).unwrap();
            account_infos.push(account_info.into());
        }
        account_infos
        // (from_index..std::cmp::min(from_index + limit, keys.len() as u64))
        //     .map(|index| self.data().accounts.get(&keys.get(index).unwrap()).unwrap())
        //     .map(|va| va.into())
        //     .collect()
    }
}
