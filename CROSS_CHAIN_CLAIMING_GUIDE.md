# Cross-Chain Token Claiming Guide

## Table of Contents
1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Contract Addresses](#contract-addresses)
4. [User Journey](#user-journey)
5. [Admin Setup](#admin-setup)
6. [Monitoring Scripts](#monitoring-scripts)
7. [Troubleshooting](#troubleshooting)
8. [Error Handling Scenarios](#error-handling-scenarios)
9. [Enhanced Monitoring Metrics](#enhanced-monitoring-metrics)
10. [Security Best Practices](#security-best-practices)
11. [Vesting Schedule Scripts](#vesting-schedule-scripts)

## Overview
This guide details the process for claiming ITLX tokens on NEAR after burning them on Avalanche. It covers both user and admin workflows, including all necessary scripts and commands.

## Prerequisites
- Metamask wallet with Avalanche C-Chain configured
- NEAR wallet (can be created at wallet.testnet.near.org)
- NEAR CLI installed (`npm install -g near-cli`)
- Node.js and npm installed
- Basic understanding of command line operations

## Contract Addresses
```
Avalanche ITLX: 0x99817F89A707620591c176a1e389da70E5b9399D
NEAR ITLX: itlx.intellex_contract_owner.testnet
NEAR Vault: session_vault.intellex_contract_owner.testnet
```

## User Journey

### 1. NEAR Wallet Setup
```bash
# Create NEAR wallet at wallet.testnet.near.org
# Once created, register with the token contract:
near call itlx.intellex_contract_owner.testnet storage_deposit '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}' --accountId YOUR_NEAR_WALLET.testnet --amount 0.00125
```

### 2. Burn Process on Avalanche
```javascript
// Approve tokens (using web3.js)
const avaxContract = "0x99817F89A707620591c176a1e389da70E5b9399D";
const abi = [
    "function approve(address spender, uint256 amount) public returns (bool)",
    "function burn(uint256 amount) public"
];
const contract = new ethers.Contract(avaxContract, abi, signer);

// Approve
await contract.approve(avaxContract, burnAmount);

// Burn
await contract.burn(burnAmount);

// Save transaction hash for verification
const txHash = receipt.transactionHash;
```

### 3. Claim Process on NEAR
```bash
# Check claim status
near view session_vault.intellex_contract_owner.testnet get_account '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}'

# Claim available tokens
near call session_vault.intellex_contract_owner.testnet claim '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}' --accountId YOUR_NEAR_WALLET.testnet --gas 100000000000000
```

## Admin Setup

### 1. Vault Configuration
```bash
# Set up vault for cross-chain claims
near call session_vault.intellex_contract_owner.testnet add_account '{
    "account_id": "crosschain_public.intellex_contract_owner.testnet",
    "start_timestamp": LAUNCH_TIME,
    "session_interval": 120,
    "session_num": 6,
    "release_per_session": "TOTAL_PUBLIC_ALLOCATION"
}' --accountId intellex_contract_owner.testnet --amount 0.1
```

### 2. Verification Script
```javascript
// verification.js
async function verifyBurnAndSetupClaim(burnTxHash, nearAccountId) {
    // 1. Verify Avalanche burn
    const burnTx = await avalancheProvider.getTransaction(burnTxHash);
    const burnEvent = parseBurnEvent(burnTx);
    
    // 2. Setup vesting on NEAR
    const setupCommand = `near call session_vault.intellex_contract_owner.testnet add_account '{
        "account_id": "${nearAccountId}",
        "start_timestamp": ${LAUNCH_TIME},
        "session_interval": 120,
        "session_num": 6,
        "release_per_session": "${convertDecimals(burnEvent.amount)}"
    }' --accountId intellex_contract_owner.testnet --amount 0.1`;
    
    // 3. Transfer tokens to vault
    const transferCommand = `near call itlx.intellex_contract_owner.testnet ft_transfer_call '{
        "receiver_id": "session_vault.intellex_contract_owner.testnet",
        "amount": "${convertDecimals(burnEvent.amount)}",
        "msg": "${nearAccountId}"
    }' --accountId intellex_contract_owner.testnet --amount 0.000000000000000000000001 --gas 100000000000000`;
    
    return { setupCommand, transferCommand };
}
```

## Monitoring Scripts

### 1. User Monitoring Script
```bash
#!/bin/bash
# monitor_user_claims.sh

NEAR_ACCOUNT="YOUR_NEAR_WALLET.testnet"

while true; do
    echo "=== $(date) ==="
    
    # Check NEAR wallet registration
    near view itlx.intellex_contract_owner.testnet storage_balance_of '{
        "account_id": "'$NEAR_ACCOUNT'"
    }'
    
    # Check vesting status
    near view session_vault.intellex_contract_owner.testnet get_account '{
        "account_id": "'$NEAR_ACCOUNT'"
    }' | grep -E "claimed_amount|unclaimed_amount|deposited_amount"
    
    # Check ITLX balance
    near view itlx.intellex_contract_owner.testnet ft_balance_of '{
        "account_id": "'$NEAR_ACCOUNT'"
    }'
    
    echo "---"
    sleep 30
done
```

### 2. Admin Monitoring Script
```bash
#!/bin/bash
# monitor_vault.sh

echo "Monitoring Vault Contract Status"

while true; do
    echo "=== $(date) ==="
    
    # Check vault total balance
    near view session_vault.intellex_contract_owner.testnet contract_metadata
    
    # Check specific account status
    near view session_vault.intellex_contract_owner.testnet get_account '{
        "account_id": "crosschain_public.intellex_contract_owner.testnet"
    }'
    
    echo "---"
    sleep 30
done
```

## Troubleshooting

### Common Issues and Solutions

1. **Token Registration Failed**
```bash
# Retry registration with higher deposit
near call itlx.intellex_contract_owner.testnet storage_deposit '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}' --accountId YOUR_NEAR_WALLET.testnet --amount 0.01
```

2. **Claim Transaction Failed**
```bash
# Retry with higher gas
near call session_vault.intellex_contract_owner.testnet claim '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}' --accountId YOUR_NEAR_WALLET.testnet --gas 200000000000000
```

3. **Verification Check**
```bash
# Check if account is properly set up
near view session_vault.intellex_contract_owner.testnet get_account '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}'
```

### Making Scripts Executable
```bash
chmod +x monitor_user_claims.sh
chmod +x monitor_vault.sh
```

### Running Monitoring Scripts
```bash
# For users
./monitor_user_claims.sh

# For admins
./monitor_vault.sh
```

## Error Handling Scenarios

### User-Side Errors
1. **Insufficient Balance for Burn**
```javascript
try {
    const balance = await contract.balanceOf(userAddress);
    if (balance.lt(burnAmount)) {
        throw new Error("Insufficient balance for burn");
    }
    await contract.burn(burnAmount);
} catch (error) {
    console.error("Burn failed:", error.message);
    // Provide user feedback
}
```

2. **Failed NEAR Registration**
```bash
# Check if registration failed due to insufficient deposit
near view itlx.intellex_contract_owner.testnet storage_balance_of '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}'

# If null, retry with higher deposit
near call itlx.intellex_contract_owner.testnet storage_deposit '{
    "account_id": "YOUR_NEAR_WALLET.testnet"
}' --accountId YOUR_NEAR_WALLET.testnet --amount 0.01
```

3. **Transaction Timeout**
```javascript
// Implement timeout for Avalanche transactions
const timeout = 60000; // 60 seconds
const timeoutPromise = new Promise((_, reject) => {
    setTimeout(() => reject(new Error("Transaction timeout")), timeout);
});

try {
    await Promise.race([contract.burn(burnAmount), timeoutPromise]);
} catch (error) {
    if (error.message === "Transaction timeout") {
        // Handle timeout
    }
}
```

### Admin-Side Errors
1. **Invalid Burn Verification**
```javascript
async function verifyBurnWithRetry(txHash, maxRetries = 3) {
    for (let i = 0; i < maxRetries; i++) {
        try {
            const burnTx = await avalancheProvider.getTransaction(txHash);
            if (!burnTx) {
                throw new Error("Transaction not found");
            }
            // Verify burn event
            return parseBurnEvent(burnTx);
        } catch (error) {
            if (i === maxRetries - 1) throw error;
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    }
}
```

2. **Vault Setup Failures**
```bash
# Check vault state before setup
near view session_vault.intellex_contract_owner.testnet get_account '{
    "account_id": "crosschain_public.intellex_contract_owner.testnet"
}'

# If setup fails, check contract storage
near view session_vault.intellex_contract_owner.testnet storage_balance_of '{
    "account_id": "crosschain_public.intellex_contract_owner.testnet"
}'
```

## Enhanced Monitoring Metrics

### 1. System Health Monitoring
```bash
#!/bin/bash
# monitor_system_health.sh

while true; do
    echo "=== System Health Check $(date) ==="
    
    # Check contract state
    near view session_vault.intellex_contract_owner.testnet contract_metadata
    
    # Check total tokens in vault
    near view itlx.intellex_contract_owner.testnet ft_balance_of '{
        "account_id": "session_vault.intellex_contract_owner.testnet"
    }'
    
    # Check pending claims
    near view session_vault.intellex_contract_owner.testnet get_total_unclaimed
    
    # Check gas usage statistics
    near state session_vault.intellex_contract_owner.testnet | grep storage_usage
    
    echo "---"
    sleep 300
done
```

### 2. Transaction Monitoring
```bash
#!/bin/bash
# monitor_transactions.sh

while true; do
    echo "=== Transaction Monitor $(date) ==="
    
    # Monitor recent claims
    near view session_vault.intellex_contract_owner.testnet get_recent_claims '{
        "from_index": 0,
        "limit": 10
    }'
    
    # Monitor failed transactions
    near view session_vault.intellex_contract_owner.testnet get_failed_transactions '{
        "from_index": 0,
        "limit": 10
    }'
    
    echo "---"
    sleep 60
done
```

### 3. User Activity Dashboard
```javascript
// dashboard.js
async function generateActivityDashboard() {
    const metrics = {
        totalBurned: await getTotalBurned(),
        totalClaimed: await getTotalClaimed(),
        activeUsers: await getActiveUsers(),
        averageClaimTime: await getAverageClaimTime(),
        failedClaims: await getFailedClaims()
    };
    
    return metrics;
}
```

## Security Best Practices

### 1. Transaction Verification
```javascript
// Implement multiple verification steps
async function verifyBurnTransaction(txHash, nearAccountId) {
    // 1. Verify transaction exists and is confirmed
    const tx = await avalancheProvider.getTransaction(txHash);
    if (!tx || tx.confirmations < 12) {
        throw new Error("Transaction not confirmed");
    }
    
    // 2. Verify burn event signature
    const burnTopic = ethers.utils.keccak256(
        ethers.utils.toUtf8Bytes("Transfer(address,address,uint256)")
    );
    const burnEvent = tx.logs.find(log => 
        log.topics[0] === burnTopic && 
        log.topics[2] === ethers.constants.HashZero
    );
    
    // 3. Verify amount matches expected format
    const amount = ethers.BigNumber.from(burnEvent.data);
    if (amount.lt(ethers.constants.Zero)) {
        throw new Error("Invalid burn amount");
    }
    
    return { verified: true, amount };
}
```

### 2. Access Control
```javascript
// Implement role-based access control
const ADMIN_ROLE = ethers.utils.keccak256(
    ethers.utils.toUtf8Bytes("ADMIN_ROLE")
);

async function verifyAdminAccess(accountId) {
    const result = await contract.hasRole(ADMIN_ROLE, accountId);
    if (!result) {
        throw new Error("Unauthorized access");
    }
}
```

### 3. Rate Limiting
```javascript
// Implement rate limiting for claim attempts
const rateLimiter = new Map();

function checkRateLimit(accountId) {
    const now = Date.now();
    const lastAttempt = rateLimiter.get(accountId) || 0;
    
    if (now - lastAttempt < 60000) { // 1 minute cooldown
        throw new Error("Rate limit exceeded");
    }
    
    rateLimiter.set(accountId, now);
}
```

## Vesting Schedule Scripts

### 1. Strategic Sale Schedule
```bash
#!/bin/bash
# setup_strategic_vesting.sh

ACCOUNT_ID=$1
START_TIMESTAMP=$2
TOTAL_AMOUNT=$3

near call session_vault.intellex_contract_owner.testnet add_account '{
    "account_id": "'$ACCOUNT_ID'",
    "start_timestamp": '$START_TIMESTAMP',
    "session_interval": 2592000,
    "session_num": 12,
    "release_per_session": "'$TOTAL_AMOUNT'"
}' --accountId intellex_contract_owner.testnet --amount 0.1
```

### 2. Public Sale Schedule
```bash
#!/bin/bash
# setup_public_vesting.sh

ACCOUNT_ID=$1
START_TIMESTAMP=$2
TOTAL_AMOUNT=$3

near call session_vault.intellex_contract_owner.testnet add_account '{
    "account_id": "'$ACCOUNT_ID'",
    "start_timestamp": '$START_TIMESTAMP',
    "session_interval": 120,
    "session_num": 6,
    "release_per_session": "'$TOTAL_AMOUNT'"
}' --accountId intellex_contract_owner.testnet --amount 0.1
```

### 3. Private Sale Schedule
```bash
#!/bin/bash
# setup_private_vesting.sh

ACCOUNT_ID=$1
START_TIMESTAMP=$2
TOTAL_AMOUNT=$3

near call session_vault.intellex_contract_owner.testnet add_account '{
    "account_id": "'$ACCOUNT_ID'",
    "start_timestamp": '$START_TIMESTAMP',
    "session_interval": 300,
    "session_num": 12,
    "release_per_session": "'$TOTAL_AMOUNT'"
}' --accountId intellex_contract_owner.testnet --amount 0.1
```

### 4. Batch Setup Script
```bash
#!/bin/bash
# batch_setup_vesting.sh

# Read accounts from CSV file
# Format: account_id,sale_type,amount
while IFS=, read -r account_id sale_type amount; do
    case $sale_type in
        "strategic")
            ./setup_strategic_vesting.sh "$account_id" "$START_TIME" "$amount"
            ;;
        "public")
            ./setup_public_vesting.sh "$account_id" "$START_TIME" "$amount"
            ;;
        "private")
            ./setup_private_vesting.sh "$account_id" "$START_TIME" "$amount"
            ;;
    esac
    
    echo "Setup complete for $account_id"
    sleep 2
done < accounts.csv 