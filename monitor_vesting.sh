#!/bin/bash

# Array of accounts to monitor
TEAM_ACCOUNTS=("team1.testnet" "team2.testnet" "team3.testnet")
PRIVATE_ACCOUNTS=("private1.testnet" "private2.testnet" "private3.testnet")
PUBLIC_ACCOUNTS=("public1.testnet" "public2.testnet" "public3.testnet")

while true; do
    clear
    echo "Current time: $(date +%s)"
    echo "============================================"
    
    echo "Team Accounts (10-min intervals, 24 sessions):"
    echo "--------------------------------------------"
    for account in "${TEAM_ACCOUNTS[@]}"; do
        echo "Account: $account"
        near view session_vault.intellex_contract_owner.testnet get_account "{\"account_id\": \"$account\"}" | grep -E "claimed_amount|unclaimed_amount|deposited_amount"
        echo "---"
    done
    
    echo "Private Sale Accounts (5-min intervals, 12 sessions):"
    echo "--------------------------------------------"
    for account in "${PRIVATE_ACCOUNTS[@]}"; do
        echo "Account: $account"
        near view session_vault.intellex_contract_owner.testnet get_account "{\"account_id\": \"$account\"}" | grep -E "claimed_amount|unclaimed_amount|deposited_amount"
        echo "---"
    done
    
    echo "Public Sale Accounts (2-min intervals, 6 sessions):"
    echo "--------------------------------------------"
    for account in "${PUBLIC_ACCOUNTS[@]}"; do
        echo "Account: $account"
        near view session_vault.intellex_contract_owner.testnet get_account "{\"account_id\": \"$account\"}" | grep -E "claimed_amount|unclaimed_amount|deposited_amount"
        echo "---"
    done
    
    sleep 30
done 