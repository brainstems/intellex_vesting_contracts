// use std::sync::LazyLock;

// use cargo_near_build::BuildOpts;
// use near_sdk_sim::{deploy, init_simulator, to_yocto, view};

// use cargo_near_build::BuildOpts;
use near_sdk::NearToken;
use near_workspaces::{network::Sandbox, Account, Contract, Worker};
use session_vault::ContractInfo;
use tokio::sync::OnceCell;

// Both are the same path
// near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
//     PREV_SESSION_VAULT_WASM_BYTES => "../res/session_vault.wasm",
//     SESSION_VAULT_WASM_BYTES => "../res/session_vault.wasm",
// }

// pub const PREV_SESSION_VAULT_WASM_PATH: &str = "../res/session_vault.wasm";
// pub const SESSION_VAULT_WASM_PATH: &str = "../res/session_vault.wasm";

// static PREV_SESSION_VAULT_WASM_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| {
//     let artifact = cargo_near_build::build(BuildOpts {
//         no_abi: true,
//         no_embed_abi: true,
//         manifest_path: Some(PREV_SESSION_VAULT_WASM_PATH.into()),
//         ..Default::default()
//     })
//     .expect("Could not compile Previous Session Vault contract for tests");

//     std::fs::read(&artifact.path).unwrap_or_else(|err| {
//         panic!(
//             "Could not read Previous Session WASM file from {}\nErr: {err}",
//             artifact.path,
//         )
//     })
// });

// static SESSION_VAULT_WASM_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| {
//     let artifact = cargo_near_build::build(BuildOpts {
//         no_abi: true,
//         no_embed_abi: true,
//         manifest_path: Some(SESSION_VAULT_WASM_PATH.into()),
//         ..Default::default()
//     })
//     .expect("Could not compile Session Vault contract for tests");

//     std::fs::read(&artifact.path).unwrap_or_else(|err| {
//         panic!(
//             "Could not read Session Vault WASM file from {}\nErr: {err}",
//             artifact.path,
//         )
//     })
// });

static SESSION_VAULT_WASM_BYTES: OnceCell<Vec<u8>> = OnceCell::const_new();

async fn session_vault_wasm_bytes() -> Vec<u8> {
    // near_workspaces::compile_project("./session_vault/")
    //     .await
    //     .unwrap()
    // let artifact: cargo_near_build::BuildArtifact = cargo_near_build::build(BuildOpts {
    //     no_abi: true,
    //     no_embed_abi: true,
    //     // out_dir: Some("../../res/".into()),
    //     // manifest_path: Some("../".into()),
    //     out_dir: Some("./res/".into()),
    //     // manifest_path: Some("./session_vault/Cargo.toml".into()),
    //     manifest_path: Some("./Cargo.toml".into()),
    //     ..Default::default()
    // })
    // .expect("Could not compile Session Vault contract for tests");
    std::fs::read("../res/session_vault.wasm").unwrap_or_else(|err| {
        panic!(
            "Could not read Session Vault WASM file from {}\nErr: {err}",
            "../session_vault.wasm"
        )
    })
}

#[tokio::test]
async fn test_upgrade() {
    let session_vault_wasm_bytes = SESSION_VAULT_WASM_BYTES
        .get_or_init(session_vault_wasm_bytes)
        .await;
    let root: Worker<Sandbox> = near_workspaces::sandbox().await.unwrap();
    let root_account: Account = root.root_account().unwrap();
    // let root = init_simulator(None);
    let test_user = root_account
        .create_subaccount("test")
        .initial_balance(NearToken::from_near(100))
        .transact()
        .await
        .unwrap();
    assert!(test_user.is_success());
    let test_user = test_user.result;
    // let test_user = root.create_user("test".to_string(), to_yocto("100"));
    let session_vault = root_account.deploy(session_vault_wasm_bytes).await.unwrap();
    assert!(session_vault.is_success());
    let session_vault: Contract = session_vault.result;
    // Maybe include gas?
    let res = session_vault
        .call("new")
        .args_json((root_account.id(), root_account.id()))
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // let session_vault = deploy!(
    //     contract: SessionVault,
    //     contract_id: "session_vault".to_string(),
    //     bytes: &PREV_SESSION_VAULT_WASM_BYTES,
    //     signer_account: root,
    //     init_method: new(root.valid_account_id(), root.valid_account_id())
    // );

    let res = test_user
        .call(session_vault.id(), "upgrade")
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure());
    let failures = res.failures();
    assert_eq!(failures.len(), 1);
    let logs = failures[0].logs.clone();
    let in_logs: Vec<String> = logs
        .into_iter()
        .filter(|log| log.contains("ERR_NOT_ALLOWED"))
        .collect();
    assert!(!in_logs.is_empty(), "Logs is {:?}", failures[0].logs);

    // Failed upgrade with no permissions.
    // let result = test_user
    //     .call(
    //         session_vault.user_account.account_id.clone(),
    //         "upgrade",
    //         &SESSION_VAULT_WASM_BYTES,
    //         near_sdk_sim::DEFAULT_GAS,
    //         0,
    //     )
    //     .status();
    // assert!(format!("{:?}", result).contains("ERR_NOT_ALLOWED"));

    let res = root_account
        .call(session_vault.id(), "upgrade")
        .args_json((&session_vault_wasm_bytes[..],))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    // root.call(
    //     session_vault.user_account.account_id.clone(),
    //     "upgrade",
    //     &SESSION_VAULT_WASM_BYTES,
    //     near_sdk_sim::DEFAULT_GAS,
    //     0,
    // )
    // .assert_success();
    let metadata: ContractInfo = session_vault
        .view("contract_metadata")
        .await
        .unwrap()
        .json::<ContractInfo>()
        .unwrap();
    // let metadata = view!(session_vault.contract_metadata()).unwrap_json::<ContractInfo>();
    // println!("{:#?}", metadata);
    assert_eq!(metadata.version, "1.0.0".to_string());

    let res = root_account
        .call(session_vault.id(), "upgrade")
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    // Upgrade to the same code migration is skipped.
    // root.call(
    //     session_vault.user_account.account_id.clone(),
    //     "upgrade",
    //     &SESSION_VAULT_WASM_BYTES,
    //     near_sdk_sim::DEFAULT_GAS,
    //     0,
    // )
    // .assert_success();
}
