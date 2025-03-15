/*!
* REF session_vault contract
*
*/
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

use crate::account::VAccount;
pub use crate::views::ContractInfo;
mod account;
mod owner;
mod utils;
mod views;

// near_sdk::setup_alloc!();

#[near(serializers = [borsh])]
#[derive(BorshStorageKey)]
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

    accounts: UnorderedMap<AccountId, VAccount>,
}

#[near(serializers = [borsh])]
pub enum VContractData {
    Current(ContractData),
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    data: VContractData,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, token_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let total_balance: U128 = U128::from(0);
        let claimed_balance: U128 = U128::from(0);
        Self {
            data: VContractData::Current(ContractData {
                owner_id: owner_id.into(),
                token_account_id: token_id.into(),
                total_balance,
                claimed_balance,
                accounts: UnorderedMap::new(StorageKeys::Accounts),
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
