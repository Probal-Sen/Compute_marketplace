# Compute Marketplace (Soroban Smart Contract)
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/ab046251-20f6-43ee-b9ee-89216c1ef457" />

## Project Description

Compute Marketplace is a decentralized platform built on Stellar's Soroban smart contract framework. It allows users to post computational tasks with rewards, and other users (workers) can accept and complete those tasks.

This system enables distributed computing using blockchain-based trust, transparency, and decentralization.

## What it does

* Users create compute tasks with rewards
* Workers browse and accept tasks
* Task creators mark tasks as completed
* All task data is stored on-chain

## Features

* Task creation with reward
* Task acceptance by workers
* Task completion tracking
* On-chain decentralized storage
* Simple and scalable marketplace logic

## Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CBX6U74XVNBVXDLWU7APYQNQQ6CVTTTWDJAY5QNU4FKWTGUMEFJUS34A
## Future Improvements

* Escrow-based payment system
* Automatic reward distribution
* Worker reputation system
* Task deadlines and verification
* Token integration (XLM / USDC)

## Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

## How to Use

1. Build the contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. Deploy using Soroban CLI:

   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/contract.wasm \
     --source-account YOUR_ACCOUNT \
     --network testnet
   ```

3. Interact with the contract:

   * `create_task` → create a new task
   * `accept_task` → accept a task
   * `complete_task` → mark task as completed
   * `get_tasks` → fetch all tasks

## Notes

* This contract currently does not include payment handling
* Use a valid Stellar account (G... address) for deployment
* Replace the contract link with actual contract ID after deployment
