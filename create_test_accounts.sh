#!/bin/bash

# Array of account names
declare -a accounts=(
    "team1"
    "team2"
    "team3"
    "private1"
    "private2"
    "private3"
    "public1"
    "public2"
    "public3"
    "reserve"
)

# Function to create an account
create_account() {
    local account="$1"
    local full_account="${account}.testnet"
    echo "Creating account: $full_account"
    
    # Create the account using NEAR CLI with faucet service
    near account create-account sponsor-by-faucet-service "$full_account" autogenerate-new-keypair save-to-keychain network-config testnet create
    
    if [ $? -eq 0 ]; then
        echo "Account $full_account created successfully"
    else
        echo "Failed to create account $full_account"
    fi
    echo "----------------------------------------"
}

# Main execution
echo "Starting account creation process..."
echo "----------------------------------------"

# Create each account
for account in "${accounts[@]}"; do
    create_account "$account"
done

echo "All accounts created successfully!"
echo "Please check ~/.near-credentials/testnet/ for the credentials" 