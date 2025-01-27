# NEAR Contract Deployment Guide

## Prerequisites

1. Install Rust and required targets:
```bash
# Install Rust 1.69.0
rustup install 1.69.0
rustup default 1.69.0

# Add WASM target
rustup target add wasm32-unknown-unknown
```

2. Install WASM optimization tools:
```bash
# On macOS (using Homebrew)
brew install binaryen
```

3. Install NEAR CLI:
```bash
npm install -g near-cli
```

## Test Accounts Setup

You can create test accounts either manually or using the provided script.

### Option 1: Manual Creation
Create the following independent testnet accounts using the NEAR wallet (https://wallet.testnet.near.org):

1. Team Accounts:
   - `team1.testnet`
   - `team2.testnet`
   - `team3.testnet`

2. Private Sale Accounts:
   - `private1.testnet`
   - `private2.testnet`
   - `private3.testnet`

3. Public Sale Accounts:
   - `public1.testnet`
   - `public2.testnet`
   - `public3.testnet`

4. Reserve Account:
   - `reserve.testnet`

Manual creation steps:
1. Visit https://wallet.testnet.near.org
2. Click "Create Account"
3. Enter the account name (e.g., `team1.testnet`)
4. Complete the account creation process
5. Save the seed phrase securely

### Option 2: Automated Creation
Use the provided script to create all accounts automatically:

```bash
# Make the script executable
chmod +x create_test_accounts.sh

# Run the script
./create_test_accounts.sh
```

The script will:
1. Generate key pairs for each account
2. Create the accounts on testnet
3. Store credentials in `~/.near-credentials/testnet/`
4. Initialize each account with 10 NEAR for testing

## Building the Contract

1. Navigate to the workspace root:
```bash
cd ref-dev-fund
```

2. Clean any previous builds:
```bash
cargo clean
rm -rf res/*.wasm
```

3. Build the contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```

4. Copy the WASM file:
```bash
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
```

## Deployment Steps

### 1. Deploy Token Contract
```bash
# Deploy ITLX token contract
near deploy itlx.intellex_contract_owner.testnet res/token.wasm --initFunction 'new' --initArgs '{"owner_id": "intellex_contract_owner.testnet"}' --accountId intellex_contract_owner.testnet
```

### 2. Deploy Vault Contract
```bash
# Deploy vesting vault contract
near deploy session_vault.intellex_contract_owner.testnet res/session_vault.wasm --initFunction 'new' --initArgs '{"owner_id": "intellex_contract_owner.testnet", "token_id": "itlx.intellex_contract_owner.testnet"}' --accountId intellex_contract_owner.testnet
```

### 3. Register Vault with Token Contract
```bash
# Register vault contract with token contract for storage
near call itlx.intellex_contract_owner.testnet storage_deposit '{"account_id": "session_vault.intellex_contract_owner.testnet"}' --accountId intellex_contract_owner.testnet --amount 0.00125
```

## Setting Up Vesting Schedules

### 1. Add Account to Vault
```bash
# Add account with vesting schedule
near call session_vault.intellex_contract_owner.testnet add_account '{
  "account_id": "team1.testnet",
  "start_timestamp": UNIX_TIMESTAMP,
  "session_interval": 600,
  "session_num": 24,
  "release_per_session": "100000000000000000000000000"
}' --accountId intellex_contract_owner.testnet --amount 0.1
```
Note: Replace UNIX_TIMESTAMP with current time + desired delay (e.g., current time + 600 for 10 minutes)

### 2. Transfer Tokens to Vault
```bash
# Calculate total tokens: release_per_session * session_num
# Example: 100 ITLX * 24 sessions = 2400 ITLX
near call itlx.intellex_contract_owner.testnet ft_transfer_call '{
  "receiver_id": "session_vault.intellex_contract_owner.testnet",
  "amount": "2400000000000000000000000000",
  "msg": "team1.testnet"
}' --accountId intellex_contract_owner.testnet --amount 0.000000000000000000000001 --gas 100000000000000
```

## Claiming Tokens
```bash
# Claim available tokens
near call session_vault.intellex_contract_owner.testnet claim '{"account_id": "team1.testnet"}' --accountId team1.testnet --gas 100000000000000
```

## Checking Account Status
```bash
# View account details
near view session_vault.intellex_contract_owner.testnet get_account '{"account_id": "team1.testnet"}'
```

## Important Notes
1. All amounts are in yoctoITLX (1 ITLX = 10^24 yoctoITLX)
2. Session interval is in seconds
3. Start timestamp must be a Unix timestamp in seconds
4. Ensure accounts are registered with the token contract before transfers
5. Wait for the start timestamp and session interval to pass before claiming tokens

## Troubleshooting

1. If you encounter dependency issues, check the `Cargo.toml` file and ensure these versions:
```toml
[dependencies]
uint = { version = "0.9.0", default-features = false }
near-sdk = "3.1.0"
near-contract-standards = "3.1.0"
serde_json = "=1.0.66"
indexmap = "=1.6.2"
```

2. If the WASM file is too large, make sure to use the optimization flags with `wasm-opt` as shown above.

3. If you get deserialization errors during initialization, verify that the account IDs are correctly formatted and that you're using the proper JSON structure for the arguments.

4. If deployment fails with size error, ensure WASM file is optimized
5. If transfers fail, verify account registration and token balances
6. If claims return 0, check if the vesting schedule has started and a full session has passed
7. For any transaction failures, try increasing the gas limit (max 300 TGas)

## Test Account Setup
For testing purposes, you can create independent testnet accounts:
1. Visit https://wallet.testnet.near.org/
2. Create new accounts with format: team1.testnet, team2.testnet, etc.
3. Each account should have at least 1 NEAR for transaction fees

## Next Steps

After successful deployment, you can:
1. Set up vesting schedules for different accounts
2. Transfer tokens to the vault
3. Configure release parameters
4. Start the vesting process 