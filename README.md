ERC-20 Token Smart Contract (Rust + ink! 🦀🚀)

A secure, efficient, and permissioned ERC-20 token implementation on a Substrate-based blockchain using Rust & ink!
🌟 Project Overview

This project is an ERC-20 Token Smart Contract built using Rust and ink!, a Rust-based smart contract framework for the Substrate blockchain. It implements all core ERC-20 functionalities such as minting, transferring, and burning tokens securely on a WASM-compatible blockchain.

Why Rust & ink! instead of Solidity?
✔ Memory Safety – Rust prevents buffer overflows and memory corruption.
✔ Concurrency – Rust’s ownership model makes it thread-safe, avoiding race conditions.
✔ WebAssembly (WASM) Support – WASM-based execution optimizes gas usage and improves security.
✔ No Reentrancy Attacks – Unlike Solidity, Rust prevents reentrant vulnerabilities at the compiler level.

📌 Features

Feature	Description
✅ ERC-20 Standard Compliance	Implements all required ERC-20 functions like transfer, approve, and balanceOf.
🔥 Mint & Burn Tokens	Admin-controlled token supply management.
🔒 Secure Transfers	Prevents re-entrancy and overflows using Rust’s type safety.
🔑 Permissioned Minting	Only the contract owner can mint new tokens.
⚡ Optimized Gas Usage	WASM-based execution ensures lower fees compared to EVM contracts.
🚀 Technology Stack

Technology	Purpose
Rust	Safe and efficient smart contract programming language
ink!	Rust-based smart contract framework for Substrate
cargo-contract	CLI tool for compiling and deploying ink! contracts
Substrate	Blockchain framework for deploying the contract
Polkadot.js	Web interface for interacting with the contract
📥 Installation Guide

🔧 Step 1: Install Rust & Set Nightly Toolchain
Run the following commands in your terminal:

# Install Rust (Nightly)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup update nightly
Verify the installation:

rustc --version
cargo --version
🔧 Step 2: Install Required Dependencies
Install cargo-contract, which is necessary for building and deploying ink! contracts:

cargo install cargo-contract --force
Ensure you have the WASM target for compiling smart contracts:

rustup target add wasm32-unknown-unknown
🔧 Step 3: Clone This Repository
git clone https://github.com/YOUR_GITHUB_USERNAME/erc20-ink-token.git
cd erc20-ink-token
🛠 Project Structure

/erc20-ink-token
│── src/
│   ├── lib.rs      # Main contract implementation
│   ├── tests.rs    # Unit tests for contract functionality
│── Cargo.toml      # Project dependencies
│── README.md       # Documentation
│── ink.toml        # ink! configuration
Each module follows modular architecture ensuring clean, reusable, and scalable smart contract code.

🏗 How It Works (Flowchart)

graph TD;
  Admin-->|Mint 500 Tokens|UserA;
  UserA-->|Transfer 100 Tokens|UserB;
  UserB-->|Burn 50 Tokens|Contract;
1️⃣ Admin mints tokens → Users receive tokens
2️⃣ Users transfer tokens → Secure ledger updates balances
3️⃣ Users burn tokens → Reducing total supply

🚀 Building & Deploying the Contract

🔹 Step 1: Build the Contract
cargo contract build
This will compile the contract and generate a .wasm file for deployment.

🔹 Step 2: Deploy the Contract
cargo contract instantiate --constructor new --salt $(date +%s)
This deploys the contract to the Substrate blockchain.

🔹 Step 3: Interacting with the Contract
You can use Polkadot.js Apps to interact with the contract or use cargo-contract CLI:

cargo contract call transfer --args <recipient> <amount>
Alternatively, you can interact via Polkadot.js Apps:

Go to https://polkadot.js.org/apps.
Connect to your local Substrate node.
Upload the .contract file and execute transactions.

🛑 Common Issues & Fixes

Issue	Fix
error[E0773]: attempted to define built-in macro more than once	Remove duplicate macro_rules! cfg definitions
duplicate lang item in crate core	Ensure Rust nightly is up to date (rustup update nightly)
distributed_slice is not implemented for this platform	Switch to linkme compatible versions
error[E0425]: cannot find value LINKME_START	Ensure your Rust toolchain is properly configured

🔗 Polkadot Integration for Local Testing

To test transactions locally:

1️⃣ Start a local Substrate node

substrate --dev
This runs a local blockchain for testing.

2️⃣ Deploy the contract on this local chain

cargo contract instantiate --constructor new
3️⃣ Interact via Polkadot.js Apps

Connect to ws://127.0.0.1:9944
Upload .contract file
Execute transactions (mint, transfer, burn)
📜 License

This project is licensed under the MIT License.

⭐ Contributing

Want to improve this project? PRs are welcome!

Fork this repo
Create a new branch (feature-new-functionality)
Commit changes
Submit a PR 🚀
📞 Support & Contact

📧 Email: arunmadhavan2821@gmail.com
💼 LinkedIn: Arunmadhavan Evr

💡 Final Thoughts
This ERC-20 token contract leverages Rust and ink! to provide secure, efficient, and scalable token management on a Substrate blockchain. By using WASM execution and memory safety features, it ensures better performance and security compared to Solidity-based ERC-20 implementations.

🚀 Deploy your own token today with Rust & ink! 🚀

💬 Feedback? Drop a Star ⭐ on GitHub if this helped!