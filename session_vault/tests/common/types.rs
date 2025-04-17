use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::json_types::WrappedBalance;
// use near_sdk::near;
use near_sdk::AccountId;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
// #[near(serializers = [serde])]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
pub struct ContractInfo {
    // only onwer can manage accounts
    pub owner_id: AccountId,
    // token kept by this vault
    pub token_account_id: AccountId,
    // the total deposited amount in this vault
    pub total_balance: U128,
    // already claimed balance
    pub claimed_balance: U128,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
// #[near(serializers = [serde])]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
pub struct AccountInfo {
    pub account_id: AccountId,
    // session start time
    pub start_timestamp: U64,
    // per session lasts, eg: 90 days
    pub session_interval: U64,
    // totally how many session, eg: 1
    pub session_num: u32,
    // the session index of previous claim, start from 1
    pub last_claim_session: u32,
    // expected total_amount = session_num * release_per_session
    pub release_per_session: U128,

    pub claimed_amount: U128,
    pub deposited_amount: U128,
    // unclaimed amount
    pub unclaimed_amount: U128,
}
