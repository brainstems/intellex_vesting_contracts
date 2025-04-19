/*!
* REF session_vault contract
*
*/

use near_sdk::borsh::BorshSerialize;
// use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::store::IterableMap;
use near_sdk::{env, near, AccountId, BorshStorageKey, PanicOnDefault};

use crate::account::VAccount;
pub use crate::views::ContractInfo;
mod account;
mod owner;
mod utils;
mod views;

// near_sdk::setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKeys {
    Accounts,
}

#[near(serializers = [borsh])]
pub struct ContractData {
    // owner of this contract
    owner_id: AccountId,

    // token kept by this vault
    token_account_id: AccountId,

    // the total deposited amount in this vault
    total_balance: U128,

    // already claimed balance
    claimed_balance: U128,

    accounts: IterableMap<AccountId, VAccount>,
}

#[near(serializers = [borsh])]
pub enum VContractData {
    Current(ContractData),
}

#[derive(PanicOnDefault)]
#[near(serializers=[borsh], contract_state)]
pub struct Contract {
    data: VContractData,
}

#[near]
impl Contract {
    #[private]
    #[init]
    pub fn new(owner_id: String, token_id: String) -> Self {
        let owner_id: AccountId = owner_id.parse().expect("ERR_INVALID_ACCOUNT_ID_OWNER");
        let token_id: AccountId = token_id.parse().expect("ERR_INVALID_ACCOUNT_ID_TOKEN");
        let total_balance: U128 = U128::from(0);
        let claimed_balance: U128 = U128::from(0);
        Self {
            data: VContractData::Current(ContractData {
                owner_id,
                token_account_id: token_id,
                total_balance,
                claimed_balance,
                accounts: IterableMap::new(StorageKeys::Accounts),
            }),
        }
    }
}

impl Contract {
    fn data(&self) -> &ContractData {
        match &self.data {
            VContractData::Current(data) => data,
        }
    }

    fn data_mut(&mut self) -> &mut ContractData {
        match &mut self.data {
            VContractData::Current(data) => data,
        }
    }
}
