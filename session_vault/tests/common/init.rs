// use std::convert::TryFrom;

use std::str::FromStr;

// use cargo_near_build::BuildOpts;
use near_sdk::AccountId;
use near_sdk::{json_types::U128, NearToken};
// use near_sdk_sim::{call, deploy, init_simulator, to_yocto, view, ContractAccount, UserAccount};

use near_workspaces::network::Sandbox;
// use session_vault::ContractContract as SessionVault;
// use session_vault::Contract as SessionVault;

// use test_token::Contract as TestToken;
// use test_token::ContractContract as TestToken;

// use cargo_near_build::BuildOpts;
use near_workspaces::{Account, Contract, DevNetwork, Worker};
use tokio::sync::OnceCell;
// use std::sync::LazyLock;

// const TEST_TOKEN_WASM_PATH: &str = "../res/test_token.wasm";
// const SESSION_VAULT_WASM_PATH: &str = "../res/session_vault.wasm";

// near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
//     TEST_TOKEN_WASM_BYTES => "../res/test_token.wasm",
//     SESSION_VAULT_WASM_BYTES => "../res/session_vault.wasm",
// }

// static TEST_TOKEN_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
//     // let artifact = cargo_near_build::build(BuildOpts {
//     //     no_abi: true,
//     //     no_embed_abi: true,
//     //     // manifest_path: Some(TEST_TOKEN_WASM_PATH.into()),
//     //     out_dir: Some("../../res/".into()),
//     //     ..Default::default()
//     // })
//     // .expect("Could not compile Test Token contract for tests");

//     std::fs::read(TEST_TOKEN_WASM_PATH).unwrap_or_else(|err| {
//         panic!(
//             "Could not read Fungible Token WASM file from {}\nErr: {err}",
//             TEST_TOKEN_WASM_PATH,
//         )
//     })
// });

// static SESSION_VAULT_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
//     // let artifact = cargo_near_build::build(BuildOpts {
//     //     no_abi: true,
//     //     no_embed_abi: true,
//     //     // out_dir: Some(SESSION_VAULT_WASM_PATH.into()),
//     //     out_dir: Some("../../res/".into()),
//     //     ..Default::default()
//     // })
//     // .expect("Could not compile Session Vault contract for tests");

//     std::fs::read(SESSION_VAULT_WASM_PATH).unwrap_or_else(|err| {
//         panic!(
//             "Could not read Session Vault WASM file from {}\nErr: {err}",
//             SESSION_VAULT_WASM_PATH,
//         )
//     })
// });

static TEST_TOKEN_CONTRACT_WASM: OnceCell<Vec<u8>> = OnceCell::const_new();
static SESSION_VAULT_CONTRACT_WASM: OnceCell<Vec<u8>> = OnceCell::const_new();

async fn test_token_contract_wasm() -> Vec<u8> {
    // let artifact: cargo_near_build::BuildArtifact = cargo_near_build::build(BuildOpts {
    //     no_abi: true,
    //     no_embed_abi: true,
    //     // out_dir: Some("../../../res/".into()),
    //     out_dir: Some("./res/".into()),
    //     // manifest_path: Some("../../../test_token/".into()),
    //     // manifest_path: Some("./test_token/".into()),
    //     // manifest_path: Some("./test_token/Cargo.toml".into()),
    //     manifest_path: Some("../test_token/Cargo.toml".into()),
    //     ..Default::default()
    // })
    // .expect("Could not compile test token contract for tests");
    std::fs::read("../res/test_token.wasm").unwrap_or_else(|err| {
        panic!(
            "Could not read test token WASM file from {}\nErr: {err}",
            "../res/test_token.wasm"
        )
    })
    // near_workspaces::compile_project("./test_token/")
    //     .await
    //     .unwrap()
}

async fn session_vault_contract_wasm() -> Vec<u8> {
    // let artifact: cargo_near_build::BuildArtifact = cargo_near_build::build(BuildOpts {
    //     no_abi: true,
    //     no_embed_abi: true,
    //     // out_dir: Some("../../../res/".into()),
    //     // manifest_path: Some("../../".into()),
    //     out_dir: Some("./res/".into()),
    //     // manifest_path: Some("./session_vault".into()),
    //     // manifest_path: Some("./session_vault/Cargo.toml".into()),
    //     manifest_path: Some("./Cargo.toml".into()),
    //     ..Default::default()
    // })
    // .expect("Could not compile Session Vault contract for tests");
    std::fs::read("../res/session_vault.wasm").unwrap_or_else(|err| {
        panic!(
            "Could not read Session Vault WASM file from {}\nErr: {err}",
            "../res/session_vault.wasm"
        )
    })
    // near_workspaces::compile_project("./session_vault/")
    //     .await
    //     .unwrap()
}

#[tokio::test]
pub async fn test_compile() {
    println!("Compiling...");
    // let compiled: Vec<u8> = near_workspaces::compile_project("./").await.unwrap();
    let compiled = TEST_TOKEN_CONTRACT_WASM
        .get_or_init(test_token_contract_wasm)
        .await;
    println!(
        "Successfully Compiled project with {} bytes",
        compiled.len()
    );
    let worker = near_workspaces::sandbox().await.unwrap();
    let account = worker.root_account().unwrap();
    println!("Deploying account");
    let res = account.deploy(compiled).await.unwrap();
    println!("Result: {:?}", res.result);
    assert!(res.is_success());
}

pub async fn test_token(
    // root: &UserAccount,
    _worker: &Worker<impl DevNetwork>,
    root: &Account,
    // root: &Account,
    token_id: AccountId,
    accounts_to_register: Vec<AccountId>,
    // ) -> ContractAccount<TestToken> {
) -> anyhow::Result<Contract> {
    let test_token_contract_wasm = TEST_TOKEN_CONTRACT_WASM
        .get_or_init(test_token_contract_wasm)
        .await;
    let res = root
        .create_subaccount(token_id.as_ref())
        .initial_balance(NearToken::from_near(100))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res is {:?}", res.details);
    let t = res.result;
    let res = t.deploy(test_token_contract_wasm).await.unwrap();
    assert!(res.is_success());
    let t = res.result;

    // let t = deploy!(
    //     contract: TestToken,
    //     contract_id: token_id,
    //     bytes: &TEST_TOKEN_WASM_BYTES,
    //     signer_account: root
    // );
    // call!(root, t.new()).assert_success();

    let res = root
        .call(t.id(), "new")
        .args_json(())
        .max_gas()
        .transact()
        .await?;

    // let res = t.call("new").args_json(()).max_gas().transact().await?;
    assert!(res.is_success(), "Res is {:?}", res);

    for account_id in accounts_to_register {
        println!("Calling storage_deposit for {account_id}");
        let res = root
            .call(t.id(), "storage_deposit")
            .args_json((Some(account_id), Option::<bool>::None))
            .deposit(NearToken::from_near(1))
            .transact()
            .await?;

        assert!(res.is_success(), "Res is {:?}", res);
        // call!(
        //     root,
        //     t.storage_deposit(Some(to_va(account_id)), None),
        //     deposit = to_yocto("1")
        // )
        // .assert_success();
    }
    Ok(t)
}

// pub fn balance_of(token: &ContractAccount<TestToken>, account_id: &AccountId) -> u128 {
pub async fn balance_of(
    worker: &Worker<impl DevNetwork>,
    token: &Contract,
    account_id: &AccountId,
) -> anyhow::Result<u128> {
    let details = worker
        .view(token.id(), "ft_balance_of")
        .args_json((account_id.clone(),))
        .await?;
    let balance: u128 = details.json::<U128>().unwrap().0;
    Ok(balance)
    // view!(token.ft_balance_of(to_va(account_id.clone())))
    //     .unwrap_json::<U128>()
    //     .0
}

// pub fn assert_stats(stats: &Stats, current_round: u32, claimed_balance: u128, locked_balance: u128, liquid_balance: u128, unclaimed_balance: u128) {
//     assert_eq!(stats.current_round, current_round);
//     assert_eq!(stats.claimed_balance.0, claimed_balance);
//     assert_eq!(stats.locked_balance.0, locked_balance);
//     assert_eq!(stats.liquid_balance.0, liquid_balance);
//     assert_eq!(stats.unclaimed_balance.0, unclaimed_balance);
// }

// pub fn assert_userinfo(info: &AccountOutput, last_claim_round: u32, unclaimed_amount: u128) {
//     assert_eq!(info.last_claim_round, last_claim_round);
//     assert_eq!(info.unclaimed_amount.0, unclaimed_amount);
// }

// pub fn setup_vault() -> (
//     UserAccount,
//     UserAccount,
//     ContractAccount<SessionVault>,
//     ContractAccount<TestToken>,
// ) {

pub async fn setup_vault() -> (Worker<Sandbox>, Account, Contract, Contract) {
    // let root = init_simulator(None);
    let defi_contract_bytes = SESSION_VAULT_CONTRACT_WASM
        .get_or_init(session_vault_contract_wasm)
        .await;
    let root = near_workspaces::sandbox().await.unwrap();
    let root_account = root.root_account().unwrap();
    let res = root_account
        .create_subaccount("owner")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success(), "Res details is {:?}", res.details);
    let owner = res.unwrap();
    // let vault = root
    //     .dev_deploy(SESSION_VAULT_WASM_BYTES.as_bytes())
    //     .await
    //     .unwrap();
    let vault = root_account
        .create_subaccount("session_vault")
        .initial_balance(NearToken::from_near(20))
        .transact()
        .await
        .unwrap()
        .result;
    let vault = vault.deploy(defi_contract_bytes).await.unwrap().result;
    let res = vault
        .call("new")
        .args_json((
            "owner.test.near".to_string(),
            "test_token.test.near".to_string(),
        ))
        .max_gas()
        .transact()
        .await
        .unwrap();

    assert!(res.is_success(), "Failure logs is: {:?}", res.failures());

    // let owner = root.create_user("owner".to_string(), to_yocto("100"));
    // let vault = deploy!(
    //     contract: SessionVault,
    //     contract_id: "session_vault".to_string(),
    //     bytes: &SESSION_VAULT_WASM_BYTES,
    //     signer_account: root,
    //     init_method: new(
    //         to_va("owner".to_string()),
    //         to_va("test_token".to_string())
    //     )
    // );
    let token_id = AccountId::from_str("test_token").unwrap();

    let token = test_token(
        &root,
        &root_account,
        token_id,
        vec![vault.id().clone(), owner.id().clone()],
    )
    .await
    .unwrap();
    // let token = test_token(
    //     &root,
    //     "test_token".to_string(),
    //     vec!["session_vault".to_string(), owner.account_id()],
    // );

    let res = owner
        .call(token.id(), "mint")
        .args_json((U128(10000),))
        .max_gas()
        .transact()
        .await
        .unwrap();
    // let res = token
    //     .call("mint")
    //     .args_json((U128(10000),))
    //     .max_gas()
    //     .transact()
    //     .await
    //     .unwrap();
    assert!(res.is_success());
    // call!(owner, token.mint(U128(10000))).assert_success();

    (root, owner, vault, token)
}
