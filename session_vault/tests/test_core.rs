use std::str::FromStr;

use crate::common::{init::*, types::*};
use common::utils::wait_seconds;
use near_sdk::{
    json_types::{U128, U64},
    AccountId, NearToken,
};
use near_workspaces::result::ExecutionFinalResult;

pub mod common;

#[tokio::test]
async fn sim_set_owner() {
    let (root, owner, session_vault, _) = setup_vault().await;
    let root_account = root.root_account().unwrap();
    let user1 = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;

    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(&contract_info.owner_id, owner.id());

    let res = user1
        .call(session_vault.id(), "set_owner")
        .args_json((user1.id().to_owned(),))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure());
    let failures = res.failures();
    assert_eq!(failures.len(), 1);

    let failure = failures.first().unwrap();
    let failure = format!("{failure:?}");
    assert!(
        failure.contains(&"ERR_NOT_ALLOWED".to_string()),
        "Failure is {:?}",
        failure
    );

    let res = owner
        .call(session_vault.id(), "set_owner")
        .args_json((user1.id().to_owned(),))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    let contract_info = session_vault.view("contract_metadata").await.unwrap();
    let contract_info: ContractInfo = contract_info.json().unwrap();
    assert_eq!(contract_info.owner_id, *user1.id());

    let res = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(10), U64(10), 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let res = user1
        .call(session_vault.id(), "set_owner")
        .args_json((owner.id(),))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let outcome = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(10), U64(10), 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(outcome.is_failure());
    let failures = outcome.failures();
    assert_eq!(failures.len(), 1);

    let failure = failures.first().unwrap();
    let failure = format!("{failure:?}");

    assert!(
        failure.contains("ERR_NOT_ALLOWED"),
        "Failure is {}",
        failure
    );
}

#[tokio::test]
async fn sim_add_user() {
    let (root, owner, session_vault, token) = setup_vault().await;
    let root_account = root.root_account().unwrap();
    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let user1 = res.result;
    let res = user1
        .call(token.id(), "storage_deposit")
        .deposit(NearToken::from_near(1))
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Failure logs is: {:?}", res.failures());
    let res = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(10), U64(10), 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure());
    let failures = res.failures();
    assert_eq!(failures.len(), 1);

    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_NOT_ALLOWED"),
        "Logs is {:?}",
        failures
    );

    // Get current timestamp
    let timestamp = root.view_block().await.unwrap().timestamp();
    println!("Current Timestamp is {timestamp}");
    let timestamp = timestamp / 10u64.pow(9);
    println!("Current Timestamp in seconds is {timestamp}");

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(timestamp + 1), U64(1), 2, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let timestamp = root.view_block().await.unwrap().timestamp();
    println!("Current Timestamp is {timestamp}");
    let timestamp = timestamp / 10u64.pow(9);
    println!("Current Timestamp in seconds is {timestamp}");
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(timestamp), U64(1), 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);

    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_ACCOUNT_IN_SESSION"),
        "got {:?}",
        failure
    );

    // Advancing block by 4 seconds so that there is enough time for session to complete
    root.fast_forward(4).await.unwrap();
    let timestamp = root.view_block().await.unwrap().timestamp();
    println!("After waiting Timestamp is {timestamp}");
    let timestamp = timestamp / 10u64.pow(9);
    println!("Current Timestamp in seconds is {timestamp}");
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(10), U64(10), 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);

    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_ACCOUNT_NEED_CLAIM"),
        "Logs are {:?}",
        failure
    );
}

#[tokio::test]
async fn sim_deposit_token() {
    let (root, owner, session_vault, token) = setup_vault().await;
    let root_account = root.root_account().unwrap();

    let token_id = AccountId::from_str("other_token").unwrap();
    let other_token = test_token(
        &root,
        &root_account,
        token_id,
        vec![session_vault.id().clone(), owner.id().clone()],
    )
    .await
    .unwrap();

    let res = owner
        .call(other_token.id(), "mint")
        .args_json((U128::from(10000),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res.result);
    let user1 = res.result;
    let res = user1
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(10), U64(10), 1, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    println!(
        "owner id is {}\nother token id is {}\nuser1 id is {}",
        owner.id(),
        other_token.id(),
        user1.id(),
    );
    // Not sure if this None is really from an Option<String>
    let res = owner
        .call(other_token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();

    let receipts = res.failures();
    assert_eq!(receipts.len(), 1, "receipts is {:#?}", receipts);
    let first = receipts.first().unwrap();
    let first = format!("{first:#?}");
    assert!(first.contains("ERR_ILLEGAL_TOKEN"), "first is {:#?}", first);

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .max_gas()
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "".to_string(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{first:#?}");
    assert!(
        first.contains("ERR_MISSING_ACCOUNT_ID"),
        "first is {:#?}",
        first
    );

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "user2".to_string(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();

    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{:#?}", first);
    assert!(
        first.contains("ERR_ACCOUNT_NOT_EXIST"),
        "first is {:#?}",
        first
    );

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(110),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();

    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{first:#?}");
    assert!(
        first.contains("ERR_INCORRECT_AMOUNT"),
        "first is {:#?}",
        first
    );

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();

    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{first:?}");
    assert!(
        first.contains("ERR_ALREADY_DEPOSIT"),
        "first is {:#?}",
        first
    );

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let user_info = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    assert_eq!(user_info.unclaimed_amount.0, 0);

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();

    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("first is {first:#?}");
    assert!(
        first.contains("ERR_ALREADY_DEPOSITED"),
        "first is {:#?}",
        first
    );
}

#[tokio::test]
async fn sim_claim() {
    let (root, owner, session_vault, token) = setup_vault().await;

    let root_account = root.root_account().unwrap();
    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res.result);
    let user1 = res.result;

    let timestamp_1 = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec();
    let timestamp_1 = timestamp_1 / 10_u64.pow(9);
    println!("timestamp_1 is {timestamp_1}");
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((
            user1.id(),
            U64(timestamp_1.max(2) - 2),
            U64(2),
            1,
            U128(100),
        ))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();

    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 0);

    let timestamp_2 = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec();
    let timestamp_2 = timestamp_2 / 10_u64.pow(9);
    println!("timestamp_2 is {timestamp_2}");

    // Advancing time until the claim was yielded
    let wait: u64 = 5 - (timestamp_2 - timestamp_1).min(5) + 1;
    println!("Waiting {wait} seconds");
    root.fast_forward(wait).await.unwrap();

    let user_info: AccountInfo = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();

    assert_eq!(
        user_info.unclaimed_amount.0, 100,
        "user_info is {:?}",
        user_info
    );

    let out_come = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(owner.id()),))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);

    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_ACCOUNT_NOT_EXIST"),
        "Logs is {:?}",
        failure
    );

    let out_come = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(user1.id()),))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);

    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_NOT_ENOUGH_BALANCE"),
        "Logs is {:?}",
        failure
    );

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();

    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 100);

    let out_come = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    // Since one of the calls failed, it will be successful while returning false
    let receipt_failures = out_come.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{first:?}");
    assert!(
        first.contains("Smart contract panicked: The account user1.test.near is not registered"),
        "first is {}",
        first
    );

    let user_info = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    assert_eq!(
        user_info.unclaimed_amount.0, 100,
        "user_info is {:?}",
        user_info
    );

    let res = user1
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 100);
    let user_info = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();

    assert_eq!(user_info.unclaimed_amount.0, 0);

    let user1_balance = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, user1_balance.0);

    let timestamp: u64 = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec()
        / 10_u64.pow(9);

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), U64(timestamp.max(1) - 1), U64(5), 2, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let out_come = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await
        .unwrap();
    let receipt_failures = out_come.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{:#?}", first);
    assert!(
        first.contains("ERR_INCORRECT_AMOUNT"),
        "first is {:#?}",
        first
    );

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .max_gas()
        .args_json((
            session_vault.id(),
            U128(200),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let timestamp = wait_seconds(&root, 4).await;
    println!("Finished waiting at {timestamp}");

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let balance = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);

    let timestamp = wait_seconds(&root, 6).await;
    println!("Finished waiting at {timestamp}");

    let res: ExecutionFinalResult = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((user1.id(),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    let balance: U128 = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(300, balance.0);
}
