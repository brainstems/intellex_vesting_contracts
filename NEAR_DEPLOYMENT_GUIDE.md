## NEAR Contract Deployment Guide

### Prerequisites
- Rust 1.69.0
- NEAR CLI 4.0.13
- wasm-opt (for WASM optimization)

### Building the Contract
```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Optimize the WASM file
wasm-opt -Oz -o res/session_vault_optimized.wasm target/wasm32-unknown-unknown/release/session_vault.wasm
```

### Account Setup
```bash
# Create independent accounts using faucet service
near account create-account sponsor-by-faucet-service itlx_team_1.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_team_2.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_team_3.testnet autogenerate-new-keypair save-to-keychain network-config testnet create

near account create-account sponsor-by-faucet-service itlx_private_1.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_private_2.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_private_3.testnet autogenerate-new-keypair save-to-keychain network-config testnet create

near account create-account sponsor-by-faucet-service itlx_public_1.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_public_2.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
near account create-account sponsor-by-faucet-service itlx_public_3.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
```

### Contract Deployment
```bash
# Deploy and initialize vault contract
near deploy session_vault.intellex_contract_owner.testnet res/session_vault_optimized.wasm --initFunction 'new' --initArgs '{"owner_id": "intellex_contract_owner.testnet", "token_id": "itlx.intellex_contract_owner.testnet"}' --accountId intellex_contract_owner.testnet

# Register vault with token contract
near call itlx.intellex_contract_owner.testnet storage_deposit '{"account_id": "session_vault.intellex_contract_owner.testnet"}' --accountId intellex_contract_owner.testnet --amount 0.00125
```

### Setting Up Vesting Schedules

#### Team Accounts (10-minute intervals)
```bash
# Add team accounts to vault
near call session_vault.intellex_contract_owner.testnet add_account '{"account_id": "team1.testnet", "start_timestamp": 1737987280, "session_interval": 600, "session_num": 24, "release_per_session": "100000000000000000000000000"}' --accountId intellex_contract_owner.testnet --amount 0.1

# Register team accounts with token contract
near call itlx.intellex_contract_owner.testnet storage_deposit '{"account_id": "team1.testnet"}' --accountId team1.testnet --amount 0.00125

# Transfer tokens to vault for team
near call itlx.intellex_contract_owner.testnet ft_transfer_call '{"receiver_id": "session_vault.intellex_contract_owner.testnet", "amount": "2400000000000000000000000000", "msg": "team1.testnet"}' --accountId intellex_contract_owner.testnet --amount 0.000000000000000000000001 --gas 100000000000000
```

#### Private Sale Accounts (5-minute intervals)
```bash
# Add private sale accounts to vault
near call session_vault.intellex_contract_owner.testnet add_account '{"account_id": "private1.testnet", "start_timestamp": 1737987280, "session_interval": 300, "session_num": 12, "release_per_session": "200000000000000000000000000"}' --accountId intellex_contract_owner.testnet --amount 0.1

# Register private accounts with token contract
near call itlx.intellex_contract_owner.testnet storage_deposit '{"account_id": "private1.testnet"}' --accountId private1.testnet --amount 0.00125

# Transfer tokens to vault for private sale
near call itlx.intellex_contract_owner.testnet ft_transfer_call '{"receiver_id": "session_vault.intellex_contract_owner.testnet", "amount": "2400000000000000000000000000", "msg": "private1.testnet"}' --accountId intellex_contract_owner.testnet --amount 0.000000000000000000000001 --gas 100000000000000
```

#### Public Sale Accounts (2-minute intervals)
```bash
# Add public sale accounts to vault
near call session_vault.intellex_contract_owner.testnet add_account '{"account_id": "public1.testnet", "start_timestamp": 1737987280, "session_interval": 120, "session_num": 6, "release_per_session": "500000000000000000000000000"}' --accountId intellex_contract_owner.testnet --amount 0.1

# Register public accounts with token contract
near call itlx.intellex_contract_owner.testnet storage_deposit '{"account_id": "public1.testnet"}' --accountId public1.testnet --amount 0.00125

# Transfer tokens to vault for public sale
near call itlx.intellex_contract_owner.testnet ft_transfer_call '{"receiver_id": "session_vault.intellex_contract_owner.testnet", "amount": "3000000000000000000000000000", "msg": "public1.testnet"}' --accountId intellex_contract_owner.testnet --amount 0.000000000000000000000001 --gas 100000000000000
```

### Claiming Tokens
```bash
# Claim available tokens
near call session_vault.intellex_contract_owner.testnet claim '{"account_id": "team1.testnet"}' --accountId team1.testnet --gas 100000000000000

# Check account status
near view session_vault.intellex_contract_owner.testnet get_account '{"account_id": "team1.testnet"}'
```

### Monitoring Script
```bash
#!/bin/bash

# Monitor vesting progress
while true; do
    echo "Current time: $(date +%s)"
    echo "Team Accounts:"
    near view session_vault.intellex_contract_owner.testnet get_account '{"account_id": "team1.testnet"}' | grep -E "claimed_amount|unclaimed_amount"
    echo "Private Sale Accounts:"
    near view session_vault.intellex_contract_owner.testnet get_account '{"account_id": "private1.testnet"}' | grep -E "claimed_amount|unclaimed_amount"
    echo "Public Sale Accounts:"
    near view session_vault.intellex_contract_owner.testnet get_account '{"account_id": "public1.testnet"}' | grep -E "claimed_amount|unclaimed_amount"
    echo "---"
    sleep 30
done
```

### Troubleshooting
- If a transaction fails with "not enough gas", increase the gas limit to 100000000000000 (100 Tgas)
- Ensure accounts are registered with the token contract before attempting transfers
- Check account status using the `get_account` view function to verify vesting schedules 