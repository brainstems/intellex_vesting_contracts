use near_sdk::{ext_contract, json_types::U128, AccountId, Gas, NearToken, Timestamp};

pub type TimestampSec = u32;

pub const GAS_FOR_FT_TRANSFER: Gas = Gas::from_gas(10_000_000_000_000);
pub const GAS_FOR_AFTER_FT_TRANSFER: Gas = Gas::from_gas(10_000_000_000_000);

pub const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);
pub const NO_DEPOSIT: NearToken = NearToken::from_yoctonear(0);

pub(crate) fn to_nano(timestamp: TimestampSec) -> Timestamp {
    Timestamp::from(timestamp) * 10u64.pow(9)
}

#[ext_contract(ext_self)]
trait AccountClaimCallbacks {
    fn after_ft_transfer(&mut self, account_id: AccountId, amount: U128) -> bool;
}
