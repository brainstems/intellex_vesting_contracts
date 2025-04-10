use std::str::FromStr;

use near_sdk::{json_types::U128, AccountId, NearToken};
// use tokio::sync::OwnedMutexGuard;
// use near_sdk_sim::{
//     call, to_yocto, view,
// };
use crate::common::{init::*, types::*};
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
    // let user1 = root.create_user("user1".to_string(), to_yocto("10"));

    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(&contract_info.owner_id, owner.id());

    // session_vault.call("set_owner")
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
    // let out_come = call!(
    //     user1,
    //     session_vault.set_owner(user1.valid_account_id()),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_NOT_ALLOWED"));

    let res = owner
        .call(session_vault.id(), "set_owner")
        .args_json((user1.id().to_owned(),))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.set_owner(user1.valid_account_id()),
    //     deposit = 1
    // )
    // .assert_success();
    let contract_info = session_vault.view("contract_metadata").await.unwrap();
    let contract_info: ContractInfo = contract_info.json().unwrap();
    assert_eq!(contract_info.owner_id, *user1.id());
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    // assert_eq!(contract_info.owner_id, user1.account_id());

    let res = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     user1,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let res = user1
        .call(session_vault.id(), "set_owner")
        .args_json(owner.id())
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     user1,
    //     session_vault.set_owner(owner.valid_account_id()),
    //     deposit = 1
    // )
    // .assert_success();

    let outcome = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(outcome.is_failure());
    let failures = outcome.failures();
    assert_eq!(failures.len(), 1);
    let logs: Vec<String> = failures
        .into_iter()
        .flat_map(|err| err.logs.clone())
        .collect();
    let filtered: Vec<String> = logs
        .clone()
        .into_iter()
        .filter(|log| log.contains(&"ERR_NOT_ALLOWED".to_string()))
        .collect();
    assert!(!filtered.is_empty(), "Logs is: {:?}", logs);

    // let out_come = call!(
    //     user1,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_NOT_ALLOWED"));
}

#[tokio::test]
async fn sim_add_user() {
    let (root, owner, session_vault, token) = setup_vault().await;
    // Block timestamp not set. See if it causes an error.
    // root.borrow_runtime_mut().cur_block.block_timestamp = 0;
    let root_account = root.root_account().unwrap();
    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let user1 = res.result;
    // let user1 = root.create_user("user1".to_string(), to_yocto("10"));
    let res = user1
        .call(token.id(), "storage_deposit")
        .deposit(NearToken::from_near(1))
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Failure logs is: {:?}", res.failures());
    // call!(
    //     user1,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();
    let res = user1
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
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
    // let out_come = call!(
    //     user1,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_NOT_ALLOWED"));

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let res = owner
        .call(user1.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
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

    // let out_come = call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ACCOUNT_IN_SESSION"));

    // Not setting block timestamp, see if it causes an error
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(20);
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128::from(100)))
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

    // let out_come = call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ACCOUNT_NEED_CLAIM"));
}

#[tokio::test]
async fn sim_deposit_token() {
    let (root, owner, session_vault, token) = setup_vault().await;
    let root_account = root.root_account().unwrap();
    // let token_id = format!("{}.{}", "test_token", root_account.id());
    let token_id = AccountId::from_str("test_token2").unwrap();
    let other_token = test_token(
        &root,
        &root_account,
        token_id,
        vec![session_vault.id().clone(), owner.id().clone()],
    )
    .await
    .unwrap();
    // let other_token = test_token(
    //     &root,
    //     "other_token".to_string(),
    //     vec![session_vault.account_id(), owner.account_id()],
    // );
    let res = owner
        .call(other_token.id(), "mint")
        .args_json((U128::from(10000),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(owner, other_token.mint(U128(10000))).assert_success();

    // Not setting timestamp at the moment. Check if it causes an error.
    // root.borrow_runtime_mut().cur_block.block_timestamp = 0;

    // let user1 = root.create_user("user1".to_string(), to_yocto("10"));
    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res.result);
    let user1 = res.result;
    // call!(
    //     user1,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    // Not sure if this None is really from an Option<String>
    let res = owner
        .call(other_token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    let failure = format!("{:?}", failures.first());
    assert!(failure.contains("ERR_ILLEGAL_TOKEN"));
    // let out_come = call!(
    //     owner,
    //     other_token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         user1.account_id()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ILLEGAL_TOKEN"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "".to_string(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(failure.contains("ERR_MISSING_ACCOUNT_ID"));
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         "".to_string()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_MISSING_ACCOUNT_ID"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "user2".to_string(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(failure.contains("ERR_ACCOUNT_NOT_EXIST"));
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         "user2".to_string()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ACCOUNT_NOT_EXIST"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(110),
            Option::<String>::None,
            "user1".to_string(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();

    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(failure.contains("ERR_INCORRECT_AMOUNT"));
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(110),
    //         None,
    //         "user1".to_string()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_INCORRECT_AMOUNT"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);

    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         user1.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "user1".to_string(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(failure.contains("ERR_ALREADY_DEPOSITED"));
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         "user1".to_string()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ALREADY_DEPOSITED"));

    // Not setting block timestamp at the moment
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(20);

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(user1, session_vault.claim(None)).assert_success();
    let user_info = session_vault
        .view("get_account")
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    assert_eq!(user_info.unclaimed_amount.0, 0);
    // let user_info =
    //     view!(session_vault.get_account(user1.valid_account_id())).unwrap_json::<AccountInfo>();
    // assert_eq!(user_info.unclaimed_amount.0, 0);

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            "user1".to_string(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure(), "Res is {:?}", res);
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    assert!(failure.contains("ERR_ALREADY_DEPOSITED"));
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         "user1".to_string()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ALREADY_DEPOSITED"));
}

#[tokio::test]
async fn sim_claim() {
    let (root, owner, session_vault, token) = setup_vault().await;
    // println!("block env ----> height: {}", root.borrow_runtime().current_block().block_height);

    // Not setting block timestamp at the moment
    // root.borrow_runtime_mut().cur_block.block_timestamp = 0;
    let root_account = root.root_account().unwrap();
    let res = root_account
        .create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res.result);
    let user1 = res.result;
    // let user1 = root.create_user("user1".to_string(), to_yocto("10"));

    let res = owner
        .call(user1.id(), "add_account")
        .args_json((user1.id(), 10, 10, 1, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 10, 10, 1, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 0);

    // Not setting block timestamp for now to see if it still works
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(20);

    let user_info: AccountInfo = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(user1.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.unclaimed_amount.0, 100);

    let out_come = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(owner.id()),))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_ACCOUNT_NOT_EXIST"),
        "Logs is {:?}",
        failure
    );
    // let out_come = call!(owner, session_vault.claim(Some(owner.valid_account_id())));
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_ACCOUNT_NOT_EXIST"));

    let out_come = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(user1.id()),))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_NOT_ENOUGH_BALANCE"),
        "Logs is {:?}",
        failure
    );
    // let out_come = call!(owner, session_vault.claim(Some(user1.valid_account_id())));

    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_NOT_ENOUGH_BALANCE"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         user1.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 100);

    let out_come = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);

    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("The account user1 is not registered"),
        "Logs is {:?}",
        failure
    );

    // let out_come = call!(user1, session_vault.claim(None));
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("The account user1 is not registered"));

    // println!("{failure_logs:?}");
    // println!("{:?}", get_logs(&out_come));

    let user_info = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(user1.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.unclaimed_amount.0, 100);

    let res = user1
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     user1,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(user1, session_vault.claim(None)).assert_success();

    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 100);
    let user_info = session_vault
        .view("get_account")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();

    // let user_info =
    //     view!(session_vault.get_account(user1.valid_account_id())).unwrap_json::<AccountInfo>();

    assert_eq!(user_info.unclaimed_amount.0, 0);

    let user1_balance = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, user1_balance.0);
    // assert_eq!(100, balance_of(&token, &user1.account_id()));

    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((user1.id(), 20, 20, 2, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(user1.valid_account_id(), 20, 20, 2, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let out_come = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(100),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(out_come.is_failure());
    let failures = out_come.failures();
    assert_eq!(failures.len(), 1);
    // let failure_logs: Vec<String> = failures
    //     .into_iter()
    //     .flat_map(|failure| failure.logs.clone())
    //     .collect();
    let failure = format!("{:?}", failures.first());
    assert!(
        failure.contains("ERR_INCORRECT_AMOUNT"),
        "Logs is {:?}",
        failure
    );
    // let out_come = call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(100),
    //         None,
    //         user1.account_id()
    //     ),
    //     deposit = 1
    // );
    // assert_eq!(get_error_count(&out_come), 1);
    // assert!(get_error_status(&out_come).contains("ERR_INCORRECT_AMOUNT"));

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(200),
            Option::<String>::None,
            user1.id(),
        ))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(200),
    //         None,
    //         user1.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();

    // Not setting block timestamp for now
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(40);

    let res = user1
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(user1, session_vault.claim(None)).assert_success();

    let balance = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);
    // assert_eq!(200, balance_of(&token, &user1.account_id()));

    // Not setting block timestamp for now
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(60);

    let res: ExecutionFinalResult = owner
        .call(session_vault.id(), "claim")
        .args_json((user1.id(),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res);
    // call!(owner, session_vault.claim(Some(user1.valid_account_id()))).assert_success();

    let balance: U128 = token
        .view("ft_balance_of")
        .args_json((user1.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(300, balance.0);
    // assert_eq!(300, balance_of(&token, &user1.account_id()));
}
