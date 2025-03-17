use crate::utils::TimestampSec;
use crate::*;
// use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
// use near_sdk::json_types::WrappedBalance;
use near_sdk::{env, ext_contract, is_promise_success, log, near, AccountId, PromiseOrValue};

#[near]
impl Contract {
    pub fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id.into();
    }

    pub fn remove_account(&mut self, account_id: AccountId) -> bool {
        self.assert_owner();
        self.internal_remove_account(account_id.into())
    }

    pub fn add_account(
        &mut self,
        account_id: AccountId,
        start_timestamp: TimestampSec,
        release_interval: TimestampSec,
        release_rounds: u32,
        release_per_round: U128,
    ) -> bool {
        self.assert_owner();
        self.internal_add_account(
            account_id,
            start_timestamp,
            release_interval,
            release_rounds,
            release_per_round,
        )
    }

    pub fn payment(&mut self, receiver_id: AccountId, amount: U128) -> PromiseOrValue<bool> {
        self.assert_owner();
        let amount: u128 = amount.into();
        let account_id: AccountId = receiver_id;

        let (liquid_balance, unclaimed_balance) = self.cur_funding_balance();
        assert!(
            (amount + unclaimed_balance) <= liquid_balance,
            "The payment amount beyonds liquidity"
        );

        if amount > 0 {
            self.claimed_balance = (self.claimed_balance.0 + amount).into();
            fungible_token::Contract::ext(self.token_account_id.clone())
                .with_attached_deposit(ONE_YOCTO)
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .ft_transfer(
                    account_id.clone(),
                    amount.into(),
                    Some(format!(
                        "Payment {} balance from {}",
                        amount,
                        env::current_account_id()
                    )),
                )
                .then(
                    Self::ext(env::current_account_id())
                        .with_attached_deposit(NO_DEPOSIT)
                        .with_static_gas(GAS_FOR_AFTER_FT_TRANSFER)
                        .after_payment_transfer(account_id, amount.into()),
                )
                .into()
            // ext_fungible_token::ft_transfer(
            //     account_id.clone(),
            //     amount.into(),
            //     Some(format!(
            //         "Payment {} balance from {}",
            //         amount,
            //         env::current_account_id()
            //     )),
            //     &self.token_account_id,
            //     ONE_YOCTO,
            //     GAS_FOR_FT_TRANSFER,
            // )
            // .then(ext_payment::after_payment_transfer(
            //     account_id,
            //     amount.into(),
            //     &env::current_account_id(),
            //     NO_DEPOSIT,
            //     GAS_FOR_AFTER_FT_TRANSFER,
            // ))
            // .into()
        } else {
            PromiseOrValue::Value(true)
        }
    }

    /// Migration function between versions.
    /// For next version upgrades, change this function.
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "ERR_NOT_ALLOWED"
        );
        let contract: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        contract
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERR_NOT_ALLOWED"
        );
    }
}

#[ext_contract(ext_payment)]
trait AccountPaymentCallbacks {
    fn after_payment_transfer(&mut self, account_id: AccountId, amount: U128) -> bool;
}

// trait AccountPaymentCallbacks {
//     fn after_payment_transfer(&mut self, account_id: AccountId, amount: U128) -> bool;
// }

#[near_bindgen]
impl AccountPaymentCallbacks for Contract {
    #[private]
    fn after_payment_transfer(&mut self, account_id: AccountId, amount: U128) -> bool {
        let promise_success = is_promise_success();
        if !promise_success {
            self.claimed_balance = (self.claimed_balance.0 - amount.0).into();
            log!(
                "Payment failed and rollback, account is {}, balance is {}",
                account_id,
                amount.0
            );
        } else {
            log!(
                "Payment succeed, account is {}, balance is {}",
                account_id,
                amount.0
            );
        }
        promise_success
    }
}

#[cfg(target_arch = "wasm32")]
mod upgrade {
    use near_sdk::env::BLOCKCHAIN_INTERFACE;
    use near_sdk::Gas;

    use super::*;

    const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";

    /// Gas for calling migration call.
    pub const GAS_FOR_MIGRATE_CALL: Gas = 10_000_000_000_000;

    /// Self upgrade and call migrate, optimizes gas by not loading into memory the code.
    /// Takes as input non serialized set of bytes of the code.
    #[no_mangle]
    pub extern "C" fn upgrade() {
        env::setup_panic_hook();
        env::set_blockchain_interface(Box::new(near_blockchain::NearBlockchain {}));
        let contract: Contract = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        contract.assert_owner();
        let current_id = env::current_account_id().into_bytes();
        let method_name = "migrate".as_bytes().to_vec();
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input into register 0.
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
                let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_MIGRATE_CALL;
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        method_name.len() as _,
                        method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }
}
