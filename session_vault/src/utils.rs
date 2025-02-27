use near_sdk::{ext_contract, Timestamp, Gas, Balance};
use near_sdk::json_types::{WrappedBalance};

pub type TimestampSec = u32;

pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;
pub const GAS_FOR_AFTER_FT_TRANSFER: Gas = 10_000_000_000_000;

pub const ONE_YOCTO: Balance = 1;
pub const NO_DEPOSIT: Balance = 0;

pub(crate) fn to_nano(timestamp: TimestampSec) -> Timestamp {
    Timestamp::from(timestamp) * 10u64.pow(9)
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn after_ft_transfer(&mut self, account_id: AccountId, amount: WrappedBalance) -> bool;
}