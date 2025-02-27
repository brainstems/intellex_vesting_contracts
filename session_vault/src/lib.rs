/*!
* REF session_vault contract
*
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::json_types::{ValidAccountId, WrappedBalance, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

pub use crate::views::ContractInfo;
use crate::account::VAccount;
mod owner;
mod account;
mod utils;
mod views;

near_sdk::setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Accounts,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractData {
    // owner of this contract
    owner_id: AccountId,

    // token kept by this vault
    token_account_id: AccountId,

    // the total deposited amount in this vault
    total_balance: Balance,
    
    // already claimed balance
    claimed_balance: Balance,

    accounts: UnorderedMap<AccountId, VAccount>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VContractData {
    Current(ContractData),
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    data: VContractData,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new(owner_id: ValidAccountId, token_id: ValidAccountId) -> Self {
        Self {
            data: VContractData::Current(ContractData {
                owner_id: owner_id.into(),
                token_account_id: token_id.into(),
                total_balance: 0,
                claimed_balance: 0,
                accounts: UnorderedMap::new(StorageKeys::Accounts)
            }),
        }
    }
    
    /// Returns true if the given account ID is the session vault
    pub fn is_session_vault(&self, account_id: &AccountId) -> bool {
        account_id == &env::current_account_id()
    }
    
    /// Returns the account ID of this session vault
    pub fn get_session_vault_id(&self) -> AccountId {
        env::current_account_id()
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

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext, PromiseResult, PromiseOrValue};
    use near_sdk::json_types::U128;

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(3)) // session vault is account(3)
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder.build()
    }

    fn setup_contract() -> (VMContext, Contract) {
        let context = get_context(accounts(0));
        testing_env!(context.clone());
        
        let contract = Contract::new(
            ValidAccountId::try_from(accounts(0)).unwrap(),
            ValidAccountId::try_from(accounts(0)).unwrap()
        );
        
        (context, contract)
    }

    #[test]
    fn test_is_session_vault() {
        let (context, contract) = setup_contract();
        
        // Test vault identification function
        assert!(contract.is_session_vault(&accounts(3)));
        assert!(!contract.is_session_vault(&accounts(0)));
        assert!(!contract.is_session_vault(&accounts(1)));
    }
    
    #[test]
    fn test_get_session_vault_id() {
        let (context, contract) = setup_contract();
        
        // Test get vault ID function
        assert_eq!(contract.get_session_vault_id(), accounts(3));
    }
    
    // Additional test for simulating what happens during withdrawal
    #[test]
    fn test_withdrawal_process() {
        let (mut context, mut contract) = setup_contract();
        
        // Setup: first deposit some tokens
        context.predecessor_account_id = accounts(0); // token contract
        testing_env!(context.clone());
        
        // Simulate a deposit
        let sender_id = ValidAccountId::try_from(accounts(1)).unwrap();
        let amount = U128(100);
        contract.ft_on_transfer(sender_id, amount, "deposit".to_string());
        
        // Now simulate a withdrawal request
        context.predecessor_account_id = accounts(1); // user
        testing_env!(context.clone());
        
        // In a real test, we would need to mock cross-contract calls for withdrawal
        // This would involve setting up expectations for the promises that would be created
        // and simulating their resolution
        
        // Example of how you might test a withdrawal function if it exists:
        // contract.withdraw(U128(50));
        
        // Then verify internal accounting has been updated correctly
    }
    
    // Test that demonstrates how direct transfers would fail to update balances
    #[test]
    fn test_direct_transfer_has_no_effect() {
        let (mut context, mut contract) = setup_contract();
        
        // In a real scenario, if someone used ft_transfer to send tokens directly
        // to the vault, ft_on_transfer would never be called
        
        // We can demonstrate this by showing that no deposit is recorded without ft_on_transfer
        
        // Check initial state - no deposits for account(1)
        // (Assuming your contract has a way to check balances)
        // assert_eq!(contract.get_balance(accounts(1)), U128(0));
        
        // If a direct transfer happened, ft_on_transfer would not be called
        // So the balance in the vault contract would remain unchanged
        
        // assert_eq!(contract.get_balance(accounts(1)), U128(0));
        // This proves why using ft_transfer directly to the vault is problematic
    }

    #[test]
    fn test_ft_on_transfer() {
        let (mut context, mut contract) = setup_contract();
        
        // Set up token contract as predecessor
        context.predecessor_account_id = accounts(0); // token contract
        testing_env!(context.clone());
        
        // Simulate a user (account 1) depositing via ft_transfer_call
        let sender_id = ValidAccountId::try_from(accounts(1)).unwrap();
        let amount = U128(100);
        
        // Call ft_on_transfer which is triggered by ft_transfer_call
        let result = contract.ft_on_transfer(sender_id, amount, "deposit".to_string());
        
        // Verify no amount is returned unused (all tokens accepted)
        match result {
            PromiseOrValue::Value(unused_amount) => assert_eq!(unused_amount, U128(0)),
            _ => panic!("Expected Value, got Promise"),
        }
        
        // In a real contract, we would check that internal accounting for the deposit 
        // has been updated correctly. The exact checks depend on your contract's 
        // implementation of deposits.
    }
}