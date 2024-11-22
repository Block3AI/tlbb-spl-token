# TLBB SPL Token

A Solana-based SPL token built using the Anchor framework. This project implements custom tokenomics and functionality for managing token supply, fees, and distribution.

---

## Features

- **Tokenomics**:
  - Total Supply: 1,000,000,000 TLBB
  - Categories:
    - Liquidity Pool: 30%
    - Presale Allocation: 25%
    - Marketing & Development: 20%
    - Team Reserve: 10%
    - Community Rewards: 10%
    - Charity Reserve: 5%

- **Transaction Fee**:
  - 2% fee per transaction:
    - 1% to a Charity Wallet
    - 1% to a Development Wallet

---

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html)

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/Block3AI/tlbb-spl-token.git
   cd tlbb-spl-token

	2.	Install dependencies:

anchor build


	3.	Configure your environment:
	•	Set up your wallet:

export ANCHOR_WALLET=/path/to/your/wallet.json


	•	Specify the cluster:

export CLUSTER_URL=https://api.devnet.solana.com

Usage

Build the Program

anchor build

Deploy the Program

anchor deploy

Run Tests

anchor test

Project Structure

tlbb-spl-token/
├── programs/               # Solana program logic
│   └── tlbb-spl-token/     
│       ├── src/
│       │   ├── lib.rs      # Main program logic
│       │   └── instruction.rs  # Instructions for program
├── tests/                  # Integration tests
│   └── tlbb_spl_token_test.rs
├── migrations/             # Deployment scripts
├── Anchor.toml             # Anchor configuration
├── Cargo.toml              # Rust package configuration
└── README.md               # Project documentation

License

This project is licensed under the MIT License.

Acknowledgements

	•	Solana Blockchain
	•	Anchor Framework

---

