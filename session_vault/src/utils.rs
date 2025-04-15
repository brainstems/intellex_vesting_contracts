use near_sdk::{
    ext_contract,
    json_types::{U128, U64},
    AccountId, Gas, NearToken, Timestamp,
};

pub type TimestampSec = U64;

pub const GAS_FOR_FT_TRANSFER: Gas = Gas::from_gas(10_000_000_000_000);
#[allow(unused)]
pub const GAS_FOR_AFTER_FT_TRANSFER: Gas = Gas::from_gas(10_000_000_000_000);

pub const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);
#[allow(unused)]
pub const NO_DEPOSIT: NearToken = NearToken::from_yoctonear(0);

pub(crate) fn to_nano(timestamp: TimestampSec) -> Timestamp {
    Timestamp::from(timestamp) * 10u64.pow(9)
}

#[allow(unused)]
#[ext_contract(ext_self)]
trait AccountClaimCallbacks {
    fn after_ft_transfer(&mut self, account_id: AccountId, amount: U128) -> bool;
}

#[allow(unused)]
#[ext_contract(ext_fungible_token)]
trait ExtFungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}
