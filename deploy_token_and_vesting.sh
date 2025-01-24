#!/bin/bash

# Configuration
export NEAR_ENV=testnet  # Using testnet for deployment
export ROOT="intellex_contract_owner.testnet"  # Contract owner account
export TOKEN="itlx.$ROOT"  # Token contract on testnet
export VAULT="vesting-vault.$ROOT"  # Vault contract on testnet
export ZERO18="000000000000000000"  # 18 zeros for decimal places
export TGAS="000000000000"

echo "Starting ITLX Token and Vesting Deployment Process..."

# ==========================================
# PHASE 1: TOKEN DEPLOYMENT
# ==========================================

echo "Phase 1: Token Contract Deployment"

# 1.1 Create token contract account
echo "Creating token contract account..."
near create-account $TOKEN --masterAccount $ROOT --initialBalance 35

# 1.2 Deploy token contract
echo "Deploying NEP-141 token contract..."
near deploy $TOKEN res/token.wasm --accountId $TOKEN

# 1.3 Initialize token contract with metadata
echo "Initializing token contract..."
near call $TOKEN new_default_meta '{
    "owner_id": "'$ROOT'",
    "total_supply": "10000000'$ZERO18'",  # 10M total supply
    "name": "ITLX Token",
    "symbol": "ITLX",
    "decimals": 18,
    "icon": ""
}' --accountId $TOKEN

echo "Token contract deployed and initialized"

# ==========================================
# PHASE 2: VESTING VAULT DEPLOYMENT
# ==========================================

echo "Phase 2: Vesting Vault Deployment"

# 2.1 Create vault contract account
echo "Creating vault contract account..."
near create-account $VAULT --masterAccount $ROOT --initialBalance 35

# 2.2 Deploy vault contract
echo "Deploying vesting vault contract..."
near deploy $VAULT res/session_vault.wasm --accountId $VAULT

# 2.3 Initialize vault with token address
echo "Initializing vault with ITLX token..."
near call $VAULT new '{
    "owner_id": "'$ROOT'",
    "token_id": "'$TOKEN'"
}' --accountId $VAULT

echo "Vault contract deployed and initialized"

# ==========================================
# PHASE 3: SETUP TOKEN <-> VAULT RELATIONSHIP
# ==========================================

echo "Phase 3: Setting up Token-Vault Relationship"

# 3.1 Register vault in token contract for storage
echo "Registering vault storage in token contract..."
near call $TOKEN storage_deposit '{
    "account_id": "'$VAULT'"
}' --accountId $ROOT --deposit 0.00125

# ==========================================
# PHASE 4: CONFIGURE VESTING SCHEDULES
# ==========================================

echo "Phase 4: Configuring Vesting Schedules"

# 4.1 Team Vesting Setup
echo "Setting up Team vesting..."
near call $VAULT add_account '{
    "account_id": "team_wallet.near",
    "start_timestamp": 1704067200,
    "session_interval": 2592000,
    "session_num": 24,
    "release_per_session": "41666'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# 4.2 Private Sale Setup
echo "Setting up Private Sale vesting..."
near call $VAULT add_account '{
    "account_id": "private_sale_wallet.near",
    "start_timestamp": 1701388800,
    "session_interval": 2592000,
    "session_num": 12,
    "release_per_session": "166666'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# 4.3 Public Sale Setup
echo "Setting up Public Sale vesting..."
near call $VAULT add_account '{
    "account_id": "public_sale_wallet.near",
    "start_timestamp": 1701388800,
    "session_interval": 2592000,
    "session_num": 6,
    "release_per_session": "500000'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# ==========================================
# PHASE 5: TOKEN DISTRIBUTION
# ==========================================

echo "Phase 5: Token Distribution to Vault"

# 5.1 Transfer tokens to vault for team
echo "Transferring team allocation..."
near call $TOKEN ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "1000000'$ZERO18'",
    "msg": "team_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# 5.2 Transfer tokens for private sale
echo "Transferring private sale allocation..."
near call $TOKEN ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "2000000'$ZERO18'",
    "msg": "private_sale_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# 5.3 Transfer tokens for public sale
echo "Transferring public sale allocation..."
near call $TOKEN ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "3000000'$ZERO18'",
    "msg": "public_sale_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# ==========================================
# PHASE 6: VERIFICATION
# ==========================================

echo "Phase 6: Final Verification"

# 6.1 Verify token setup
echo "Verifying token setup..."
near view $TOKEN ft_metadata
near view $TOKEN ft_total_supply

# 6.2 Verify vault setup
echo "Verifying vault setup..."
near view $VAULT contract_metadata

# 6.3 Verify vesting schedules
echo "Verifying vesting schedules..."
echo "Team vesting:"
near view $VAULT get_account '{"account_id": "team_wallet.near"}'
echo "Private sale vesting:"
near view $VAULT get_account '{"account_id": "private_sale_wallet.near"}'
echo "Public sale vesting:"
near view $VAULT get_account '{"account_id": "public_sale_wallet.near"}'

echo "Deployment complete! Important notes:"
echo "1. Token Contract: $TOKEN"
echo "2. Vesting Vault: $VAULT"
echo "3. Total tokens in vesting: 6,000,000 ITLX"
echo "4. Remaining tokens: 4,000,000 ITLX (in owner account)" 