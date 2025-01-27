#!/bin/bash

# Configuration
export NEAR_ENV=testnet  # Change to testnet for testing
export ROOT="intellex_contract_owner.testnet"  # Your main account that will be the owner
export VAULT="vesting-vault.intellex_contract_owner.near"  # The vault contract account
export FT="itlx.intellex_contract_owner.near"  # Your ITLX token contract
export ZERO18="000000000000000000000000"  # 18 zeros for decimal places
export TGAS="000000000000"

echo "Setting up vesting vault for ITLX token..."

# 1. Create and deploy contract
echo "Creating contract account..."
near create-account $VAULT --masterAccount $ROOT --initialBalance 35

echo "Deploying contract..."
near deploy $VAULT res/session_vault.wasm --accountId $VAULT

echo "Initializing contract..."
near call $VAULT new '{"owner_id": "'$ROOT'", "token_id": "'$FT'"}' --accountId $VAULT

# 2. Setup Team Vesting
# Parameters:
# - Starts: January 1, 2024 00:00:00 UTC (1704067200)
# - 24 month vesting (2 years)
# - Monthly unlocks (2592000 seconds)
# - 24 sessions (monthly unlocks)
# - 1,000,000 ITLX total (41,666.66 ITLX per month)
echo "Setting up Team vesting..."
near call $VAULT add_account '{
    "account_id": "team_wallet.near",
    "start_timestamp": 1704067200,
    "session_interval": 2592000,
    "session_num": 24,
    "release_per_session": "41666'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# 3. Setup Private Sale Vesting
# Parameters:
# - Starts: December 1, 2023 00:00:00 UTC (1701388800)
# - 12 month vesting (1 year)
# - Monthly unlocks (2592000 seconds)
# - 12 sessions
# - 2,000,000 ITLX total (166,666.66 ITLX per month)
echo "Setting up Private Sale vesting..."
near call $VAULT add_account '{
    "account_id": "private_sale_wallet.near",
    "start_timestamp": 1701388800,
    "session_interval": 2592000,
    "session_num": 12,
    "release_per_session": "166666'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# 4. Setup Public Sale Vesting
# Parameters:
# - Starts: December 1, 2023 00:00:00 UTC (1701388800)
# - 6 month vesting
# - Monthly unlocks (2592000 seconds)
# - 6 sessions
# - 3,000,000 ITLX total (500,000 ITLX per month)
echo "Setting up Public Sale vesting..."
near call $VAULT add_account '{
    "account_id": "public_sale_wallet.near",
    "start_timestamp": 1701388800,
    "session_interval": 2592000,
    "session_num": 6,
    "release_per_session": "500000'$ZERO18'"
}' --accountId $ROOT --deposit 0.1

# 5. Verify setup
echo "Verifying setup..."
echo "Team vesting details:"
near view $VAULT get_account '{"account_id": "team_wallet.near"}'
echo "Private sale vesting details:"
near view $VAULT get_account '{"account_id": "private_sale_wallet.near"}'
echo "Public sale vesting details:"
near view $VAULT get_account '{"account_id": "public_sale_wallet.near"}'

echo "Setup complete! Next steps:"
echo "1. Review the vesting parameters above and adjust as needed"
echo "2. Deploy the contract and run this script"
echo "3. Deposit the following amounts of ITLX for each group:"
echo "   - Team: 1,000,000 ITLX"
echo "   - Private Sale: 2,000,000 ITLX"
echo "   - Public Sale: 3,000,000 ITLX"

# 6. Token Deposits
echo "Depositing tokens for each group..."
echo "Note: Make sure you have approved the vault contract to transfer ITLX tokens"

# First, register the vault in the ITLX token contract
echo "Registering vault in ITLX token contract..."
near call $FT storage_deposit '{
    "account_id": "'$VAULT'"
}' --accountId $ROOT --deposit 0.00125

# Deposit for Team
echo "Depositing team tokens..."
near call $FT ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "1000000'$ZERO18'",
    "msg": "team_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# Deposit for Private Sale
echo "Depositing private sale tokens..."
near call $FT ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "2000000'$ZERO18'",
    "msg": "private_sale_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# Deposit for Public Sale
echo "Depositing public sale tokens..."
near call $FT ft_transfer_call '{
    "receiver_id": "'$VAULT'",
    "amount": "3000000'$ZERO18'",
    "msg": "public_sale_wallet.near"
}' --accountId $ROOT --depositYocto=1 --gas=100$TGAS

# Final verification
echo "Verifying final balances..."
near view $VAULT contract_metadata 