use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{
    env, is_promise_success, log, near_bindgen, AccountId, Balance, PromiseOrValue,
};
use crate::utils::*;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VAccount {
    Current(Account),
}

impl VAccount {
    /// Upgrades from other versions to the currently used version.
    pub fn into_current(self) -> Account {
        match self {
            VAccount::Current(account) => account,
        }
    }

    pub fn new() -> Self {
        Self::Current(Account {
            account_id: "".to_string(),
            start_timestamp: 0,
            session_interval: 0,
            session_num: 0,
            last_claim_session: 0,
            release_per_session: 0,
            claimed_amount: 0,
            deposited_amount: 0,
        })
    }
    
    pub fn deposit(&mut self, amount: Balance) {
        match self {
            VAccount::Current(account) => account.deposited_amount += amount,
        }
    }
    
    pub fn withdraw(&mut self, amount: Balance) -> Balance {
        match self {
            VAccount::Current(account) => {
                assert!(account.deposited_amount >= amount, "Insufficient balance");
                account.deposited_amount -= amount;
                amount
            },
        }
    }
    
    pub fn get_balance(&self) -> Balance {
        match self {
            VAccount::Current(account) => account.deposited_amount,
        }
    }
}

impl From<Account> for VAccount {
    fn from(account: Account) -> Self {
        VAccount::Current(account)
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub account_id: AccountId,
    
    // session start time
    pub start_timestamp: TimestampSec,
    // per session lasts, eg: 90 days 
    pub session_interval: TimestampSec,
    // totally how many session, eg: 1
    pub session_num: u32,
    // the session index of previous claim, start from 1
    pub last_claim_session: u32,
    // expected total amount this time = session_num * release_per_session
    pub release_per_session: Balance,

    // accumulated claimed amount since account created,
    // each time claim would increase this one
    pub claimed_amount: Balance,
    // accumulated deposited amount since account created,
    // each time ft_transfer_call would increase this one
    pub deposited_amount: Balance,
}

impl Account {
    
    pub(crate) fn unclaimed_amount(&self, cur_ts: u64) -> u128 {
        if self.last_claim_session >= self.session_num {
            return 0_u128;
        }

        let cur_session = if cur_ts > to_nano(self.start_timestamp) {
            ((cur_ts - to_nano(self.start_timestamp))
                / to_nano(self.session_interval)) as u32
        } else {
            0
        };

        let times = if cur_session >= self.session_num {
            self.session_num - self.last_claim_session
        } else {
            cur_session - self.last_claim_session
        };

        self.release_per_session * times as u128
    }

    pub fn locking_amount(&self) -> Balance {
        self.deposited_amount - self.claimed_amount
    }
}

impl Contract {
    pub fn internal_deposit_to_account(
        &mut self,
        account_id: &AccountId,
        amount: Balance
    ){
        let mut account = self
                .data()
                .accounts
                .get(&account_id)
                .map(|va| va.into_current())
                .expect("ERR_ACCOUNT_NOT_EXIST");
        assert!(account.locking_amount() == 0 && account.last_claim_session != account.session_num, "ERR_ALREADY_DEPOSITED");
        assert!(account.session_num as Balance * account.release_per_session == amount, "ERR_INCORRECT_AMOUNT");

        account.deposited_amount += amount;
        self.data_mut().accounts.insert(&account_id, &account.into());
        self.data_mut().total_balance += amount;
    }
    
    pub fn internal_add_account(
        &mut self,
        account_id: AccountId,
        start_timestamp: TimestampSec,
        session_interval: TimestampSec,
        session_num: u32,
        release_per_session: Balance,
    ) -> bool {
        if let Some(acc) = self.data().accounts.get(&account_id) {
            let mut account = acc.into_current();
            assert!(to_nano(account.start_timestamp + account.session_num * account.session_interval)
                < env::block_timestamp(), "ERR_ACCOUNT_IN_SESSION");
            assert_eq!(0, account.unclaimed_amount(env::block_timestamp()), "ERR_ACCOUNT_NEED_CLAIM");
            account.start_timestamp = start_timestamp;
            account.session_interval = session_interval;
            account.session_num = session_num;
            account.release_per_session = release_per_session;
            account.last_claim_session = 0;
            self.data_mut().accounts.insert(&account_id, &account.into());
        } else {
            let account = Account {
                account_id: account_id.clone(),
                start_timestamp,
                session_interval,
                session_num,
                last_claim_session: 0,
                release_per_session,
                claimed_amount: 0,
                deposited_amount: 0,
            };
            self.data_mut().accounts.insert(&account_id, &account.into());
        }
        true
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn claim(&mut self, account_id: Option<ValidAccountId>) -> PromiseOrValue<bool> {
        let account_id = account_id.map(|va| va.into()).unwrap_or(env::predecessor_account_id());
        let mut account = self
                .data()
                .accounts
                .get(&account_id)
                .map(|va| va.into_current())
                .expect("ERR_ACCOUNT_NOT_EXIST");
             
        if account.last_claim_session > 0 && account.last_claim_session >= account.session_num {
            // all token has been claimed.
            return PromiseOrValue::Value(false);
        }
        let amount = account.unclaimed_amount(env::block_timestamp());
        if amount == 0 {
            return PromiseOrValue::Value(true);
        }

        assert!(
            amount <= account.locking_amount(),
            "ERR_NOT_ENOUGH_BALANCE"
        );

        let sessions = (amount / account.release_per_session) as u32;
        account.last_claim_session += sessions;
        account.claimed_amount += amount;

        self.data_mut().claimed_balance += amount;
        self.data_mut().accounts.insert(&account_id, &account.into());

        ext_fungible_token::ft_transfer(
            account_id.clone(),
            amount.into(),
            Some(format!(
                "Claiming unlocked {} balance from {}",
                amount,
                env::current_account_id()
            )),
            &self.data().token_account_id,
            ONE_YOCTO,
            GAS_FOR_FT_TRANSFER,
        )
        .then(ext_self::after_ft_transfer(
            account_id,
            amount.into(),
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_AFTER_FT_TRANSFER,
        ))
        .into()
    }

    #[private]
    pub fn after_ft_transfer(&mut self, account_id: AccountId, amount: WrappedBalance) -> bool {
        let promise_success = is_promise_success();
        if !promise_success {
            let mut account = self
                .data()
                .accounts
                .get(&account_id)
                .map(|va| va.into_current())
                .expect("The claim is not found");
            let times = (amount.0 / account.release_per_session) as u32;
            account.last_claim_session -= times;
            account.claimed_amount -= amount.0;

            self.data_mut().claimed_balance -= amount.0;
            self.data_mut().accounts.insert(&account_id, &account.into());
            log!(
                "Account claim failed and rollback, account is {}, balance is {}",
                account_id,
                amount.0
            );
        } else {
            log!(
                "Account claim succeed, account is {}, balance is {}",
                account_id,
                amount.0
            );
        }
        promise_success
    }
}


#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Callback on receiving tokens by this contract.
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: near_sdk::json_types::U128,
        msg: String,
    ) -> PromiseOrValue<near_sdk::json_types::U128> {

        let token_in = env::predecessor_account_id();
        let amount: Balance = amount.into();
        assert_eq!(token_in, self.data().token_account_id, "ERR_ILLEGAL_TOKEN");

        if msg.is_empty() {
            env::panic(b"ERR_MISSING_ACCOUNT_ID");
        } else {
            self.internal_deposit_to_account(&msg, amount);
        }

        let sender: AccountId = sender_id.into();
        log!(
            "{} deposit token to {}, amount: {}",
            sender,
            msg,
            amount
        );
        PromiseOrValue::Value(0.into())
    }
}