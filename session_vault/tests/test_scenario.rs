use common::utils::wait_until;
use near_sdk::{
    json_types::{U128, U64},
    AccountId, NearToken,
};
use near_workspaces::{result::ExecutionFinalResult, Account};
// use near_sdk_sim::{
//     call, to_yocto, view,
// };
use crate::common::{init::*, types::*};

pub mod common;

/// add unstarted accounts
#[tokio::test]
async fn sim_one_round_scenario_1() {
    let (root, owner, session_vault, token) = setup_vault().await;

    let root_account: Account = root.root_account().unwrap();
    // Not setting block timestamp for now
    // root.borrow_runtime_mut().cur_block.block_timestamp = 0;

    let alice: Account = root_account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    let bob: Account = root_account
        .create_subaccount("bob")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    let charlie: Account = root_account
        .create_subaccount("charlie")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    // let alice = root.create_user("alice".to_string(), to_yocto("10"));
    // let bob = root.create_user("bob".to_string(), to_yocto("10"));
    // let charlie = root.create_user("charlie".to_string(), to_yocto("10"));

    let res: ExecutionFinalResult = alice
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     alice,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();
    let res: ExecutionFinalResult = bob
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:#?}", res);
    // call!(
    //     bob,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();
    let res: ExecutionFinalResult = charlie
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     charlie,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();

    let timestamp = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec()
        / 10_u64.pow(9);
    println!("timestamp is {timestamp}");
    let res: ExecutionFinalResult = owner
        .call(session_vault.id(), "add_account")
        .args_json((alice.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // start from 100 sec, and release 100 token per 100 sec for 4 times, so the end is 500 sec.
    // call!(
    //     owner,
    //     session_vault.add_account(alice.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();
    let res: ExecutionFinalResult = owner
        .call(session_vault.id(), "add_account")
        .args_json((bob.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(bob.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();
    let res: ExecutionFinalResult = owner
        .call(session_vault.id(), "add_account")
        .args_json((charlie.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(charlie.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let now: u64 = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec();
    let one_second: u64 = 10_u64.pow(9);
    let first: u64 = now + 10 * one_second;
    let second: u64 = now + 20 * one_second;
    let third: u64 = now + 30 * one_second;
    let fifth = now + 50 * one_second;

    let res: ExecutionFinalResult = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            alice.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // fill tokens
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         alice.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let res: ExecutionFinalResult = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            bob.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         bob.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            charlie.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         charlie.account_id()
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
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info: AccountInfo = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);

    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));
    // and claim would got nothing changed

    let res = alice
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);

    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info: AccountInfo = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    // view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance: U128 = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    // Not setting block time at the moment
    // // go to the start time
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(100);
    wait_until(&root, first).await;
    let user_info: AccountInfo = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);

    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    let res = alice
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);

    // // and claim would got nothing changed
    // call!(alice, session_vault.claim(None)).assert_success();

    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    // Not setting timestamp at the moment
    // // pass one interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(200);
    wait_until(&root, second).await;

    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 100);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    let res = alice
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // and claim does something
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    // Not setting timestamp at the moment
    // // go to 1.5 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(250);
    wait_until(&root, second + 5 * 10_u64.pow(9)).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    let res = alice
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim does nothing
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    // Not setting timestamp at the moment
    // // go to 2 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(300);
    wait_until(&root, third).await;

    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 200);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);

    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &bob.account_id()));
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(bob.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);

    // // and claim 2 sessions
    // call!(owner, session_vault.claim(Some(bob.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 300);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 200);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 2);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);
    // assert_eq!(200, balance_of(&token, &bob.account_id()));
    let res = bob
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim again does nothing
    // call!(bob, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 300);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 200);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 2);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);
    // assert_eq!(200, balance_of(&token, &bob.account_id()));

    // Not setting block timestamp at the moment
    // // go to 4 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(500);
    wait_until(&root, fifth).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 300);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    let res = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(alice.id()),))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim does something
    // call!(owner, session_vault.claim(Some(alice.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 600);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &alice.account_id()));
    let res = alice
        .call(session_vault.id(), "claim")
        .args_json((Option::<AccountId>::None,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim again does nothing
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 600);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);

    // assert_eq!(400, balance_of(&token, &alice.account_id()));

    // Not setting timestamp at the moment
    // // go to 5 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(600);
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 400);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &charlie.account_id()));
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(charlie.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim does something
    // call!(owner, session_vault.claim(Some(charlie.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1000);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &charlie.account_id()));
    let res = charlie
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // // and claim again does nothing
    // call!(charlie, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1000);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &charlie.account_id()));

    // // go to 6 interval, end everything
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(700);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.unclaimed_amount.0, 200);
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(alice.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(alice.valid_account_id()))).assert_success();
    let res = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(bob.id()),))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(bob.valid_account_id()))).assert_success();
    let res = owner
        .call(session_vault.id(), "claim")
        .args_json((Some(charlie.id()),))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(charlie.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1200);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &bob.account_id()));
}

/// add already started accounts
#[tokio::test]
async fn sim_one_round_scenario_2() {
    let (root, owner, session_vault, token) = setup_vault().await;

    let root_account = root.root_account().unwrap();
    // Not setting block timestamp at the moment
    // root.borrow_runtime_mut().cur_block.block_timestamp = 150;

    let alice = root_account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    let bob = root_account
        .create_subaccount("bob")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    let charlie = root_account
        .create_subaccount("charlie")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap()
        .result;
    // let alice = root.create_user("alice".to_string(), to_yocto("10"));
    // let bob = root.create_user("bob".to_string(), to_yocto("10"));
    // let charlie = root.create_user("charlie".to_string(), to_yocto("10"));

    let res = alice
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     alice,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();
    let res = bob
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     bob,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();
    let res = charlie
        .call(token.id(), "storage_deposit")
        .args_json((Option::<AccountId>::None, Option::<bool>::None))
        .deposit(NearToken::from_near(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     charlie,
    //     token.storage_deposit(None, None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();

    let timestamp = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec()
        / 10_u64.pow(9);
    println!("timestamp is {timestamp}");
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((alice.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // start from 100 sec, and release 100 token per 100 sec for 4 times, so the end is 500 sec.
    // call!(
    //     owner,
    //     session_vault.add_account(alice.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((bob.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     session_vault.add_account(bob.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();
    let res = owner
        .call(session_vault.id(), "add_account")
        .args_json((charlie.id(), U64(timestamp + 10), U64(10), 4, U128(100)))
        .deposit(NearToken::from_millinear(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    let now = root
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec();
    let second_duration = 10_u64.pow(9);
    let first: u64 = now + 10 * second_duration;
    let second: u64 = now + 20 * second_duration;
    let third: u64 = now + 30 * second_duration;
    let fifth: u64 = now + 50 * second_duration;
    // call!(
    //     owner,
    //     session_vault.add_account(charlie.valid_account_id(), 100, 100, 4, 100.into()),
    //     deposit = to_yocto("0.1")
    // )
    // .assert_success();

    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            alice.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // fill tokens
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         alice.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            bob.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         bob.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let res = owner
        .call(token.id(), "ft_transfer_call")
        .args_json((
            session_vault.id(),
            U128(400),
            Option::<String>::None,
            charlie.id(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(
    //     owner,
    //     token.ft_transfer_call(
    //         session_vault.valid_account_id(),
    //         U128(400),
    //         None,
    //         charlie.account_id()
    //     ),
    //     deposit = 1
    // )
    // .assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    let res = alice
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // and claim would got nothing changed
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 0);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));

    // pass one interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(200);
    wait_until(&root, second).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 100);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &alice.account_id()));
    // and claim does something
    let res = alice
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    // go to 1.5 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(250);
    wait_until(&root, first + 5 * second_duration).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));
    // and claim does nothing
    let res = alice
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 100);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));

    // go to 2 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(300);
    wait_until(&root, third).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 200);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &bob.account_id()));
    // and claim 2 sessions
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(bob.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(bob.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 300);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 200);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 2);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);
    // assert_eq!(200, balance_of(&token, &bob.account_id()));
    // and claim again does nothing
    let res = bob
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(bob, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 300);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 200);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 2);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(200, balance.0);
    // assert_eq!(200, balance_of(&token, &bob.account_id()));

    // go to 4 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(500);
    wait_until(&root, fifth).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 100);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 300);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 1);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(100, balance.0);
    // assert_eq!(100, balance_of(&token, &alice.account_id()));
    // and claim does something
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(alice.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(alice.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 600);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &alice.account_id()));
    // and claim again does nothing
    let res = alice
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(alice, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 600);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(alice.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((alice.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &alice.account_id()));

    // go to 5 interval
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(600);
    wait_until(&root, fifth + 10 * second_duration).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 0);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 400);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 0);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(0, balance.0);
    // assert_eq!(0, balance_of(&token, &charlie.account_id()));
    // and claim does something
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(charlie.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(charlie.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1000);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &charlie.account_id()));
    // and claim again does nothing
    let res = charlie
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Option::<AccountId>::None,))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(charlie, session_vault.claim(None)).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1000);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(charlie.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((charlie.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &charlie.account_id()));

    // go to 6 interval, end everything
    // root.borrow_runtime_mut().cur_block.block_timestamp = to_nano(700);
    wait_until(&root, fifth + 20 * second_duration).await;
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.unclaimed_amount.0, 200);
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(alice.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(alice.valid_account_id()))).assert_success();
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(bob.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(bob.valid_account_id()))).assert_success();
    let res = owner
        .call(session_vault.id(), "claim")
        .max_gas()
        .args_json((Some(charlie.id()),))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "res is {:?}", res);
    // call!(owner, session_vault.claim(Some(charlie.valid_account_id()))).assert_success();
    let contract_info = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let contract_info = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    assert_eq!(contract_info.claimed_balance.0, 1200);
    assert_eq!(contract_info.total_balance.0, 1200);
    let user_info = session_vault
        .view("get_account")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<AccountInfo>()
        .unwrap();
    // let user_info =
    //     view!(session_vault.get_account(bob.valid_account_id())).unwrap_json::<AccountInfo>();
    assert_eq!(user_info.claimed_amount.0, 400);
    assert_eq!(user_info.deposited_amount.0, 400);
    assert_eq!(user_info.unclaimed_amount.0, 0);
    assert_eq!(user_info.start_timestamp.0, timestamp + 10);
    assert_eq!(user_info.session_interval.0, 10);
    assert_eq!(user_info.session_num, 4);
    assert_eq!(user_info.release_per_session.0, 100);
    assert_eq!(user_info.last_claim_session, 4);
    let balance = token
        .view("ft_balance_of")
        .args_json((bob.id(),))
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(400, balance.0);
    // assert_eq!(400, balance_of(&token, &bob.account_id()));
}
