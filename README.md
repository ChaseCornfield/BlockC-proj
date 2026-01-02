# BlockC - Simple Blockchain Implementation in Rust

A learning project implementing a basic blockchain with transactions, digital signatures, and block hashing. This project demonstrates core blockchain concepts while learning Rust ownership, borrowing, and data structures.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Modules](#modules)
- [Usage Examples](#usage-examples)
- [Architecture Decisions](#architecture-decisions)
- [Getting Started](#getting-started)

## Overview

This is an educational blockchain implementation built to learn:
- **Rust ownership & borrowing** - Managing data ownership in a blockchain context
- **Data structures** - Using `Vec` to chain blocks together
- **Cryptographic hashing** - SHA-256 for block and transaction hashing
- **Digital signatures** - Simple signing mechanism for transactions
- **Blockchain fundamentals** - How blocks link together and maintain integrity

**Disclaimer**: This is a learning project and should not be used in production environments.

## Features

### Implemented

- **Entity Management** - Create entities with addresses, balances, and key pairs
- **Transaction System** - Create, sign, and execute transactions between entities
- **Digital Signatures** - Transaction signing using SHA-256 with private keys
- **Block Structure** - Blocks with hash, previous hash, transactions, timestamp, and nonce
- **Block Hashing** - SHA-256 hashing of block data
- **Balance Management** - Send/receive amounts with validation

### Planned

- **Blockchain** - Chain blocks together with validation
- **Proof of Work** - Mining blocks with difficulty adjustment
- **Chain Validation** - Verify blockchain integrity
- **Persistence** - Save/load blockchain from file

## Project Structure

```
blockc/
├── src/
│   ├── main.rs                 # Entry point
│   ├── entity/
│   │   ├── mod.rs              # Module declaration
│   │   └── entity.rs           # Entity (wallet/user) implementation
│   ├── transactions/
│   │   ├── mod.rs              # Module declaration
│   │   └── transactions.rs      # Transaction implementation
│   ├── block/
│   │   ├── mod.rs               # Module declaration
│   │   └── block.rs             # Block implementation
│   └── helpers/
│       ├── mod.rs               # Module declaration
│       └── helper_functions.rs # Utility functions
├── Cargo.toml                   # Dependencies
└── README.md                    # This file
```

## Modules

### Entity Module (`src/entity/`)

Represents a participant in the blockchain (like a wallet or user).

**Struct:**
```rust
pub struct Entity {
    pub address: String,        // Unique identifier
    pub balance: f64,          // Current balance
    pub history: Vec<Transaction>, // Transaction history
    pub public_key: String,     // Public key for verification
    pub private_key: String,    // Private key for signing
}
```

**Key Methods:**
- `new()` - Create a new entity
- `get_balance()` - Get current balance
- `can_send(amount)` - Check if entity has sufficient balance
- `send_amount(amount)` - Deduct amount from balance
- `receive_amount(amount)` - Add amount to balance
- `add_to_history(transaction)` - Record transaction in history
- `sign(transaction_data)` - Sign transaction data with private key

**Design Decision**: Entities store their balance and history for simplicity. In real blockchains, balances are calculated from transaction history.

### Transaction Module (`src/transactions/`)

Represents a transaction between two entities.

**Struct:**
```rust
pub struct Transaction {
    pub sender_address: String,    // Sender's address
    pub receiver_address: String,  // Receiver's address
    pub amount: f64,               // Transaction amount
    pub timestamp: u32,            // When transaction occurred
    pub signature: String,         // Digital signature
}
```

**Key Methods:**
- `new()` - Create a new transaction
- `create_and_sign()` - Create and sign a transaction
- `create_and_execute()` - Create, sign, validate, and execute transaction

**Design Decision**: Transactions store only addresses (not full Entity objects) to:
- Avoid stale data (entity balance might change)
- Match real blockchain design
- Reduce memory usage
- Improve data consistency

### Block Module (`src/block/`)

Represents a block in the blockchain.

**Struct:**
```rust
pub struct Block {
    block_hash: String,              // This block's hash
    previous_block_hash: String,     // Previous block's hash
    transaction: Vec<Transaction>,    // Transactions in this block
    time_stamp: u32,                 // Block creation time
    nonce: u32,                      // Proof of work value
}
```

**Key Methods:**
- `new()` - Create a new block
- `hash()` - Static function to calculate block hash
- `calculate_hash()` - Instance method to calculate this block's hash

**Hash Calculation**: The block hash includes:
- Previous block hash
- Timestamp
- Nonce
- All transaction data (sender, receiver, amount, timestamp)

### Helpers Module (`src/helpers/`)

Utility functions used across the project.

**Functions:**
- `get_time()` - Get current Unix timestamp as `u32`

## Usage Examples

### Creating Entities

```rust
use blockc::entity::entity::Entity;

let alice = Entity::new(
    "Alice".to_string(),
    100.0,  // Initial balance
    Vec::new(),  // Empty history
    "alice_public_key".to_string(),
    "alice_private_key".to_string(),
);
```

### Creating and Executing Transactions

```rust
use blockc::transactions::transactions::Transaction;
use blockc::entity::entity::Entity;

// Create entities
let mut alice = Entity::new(...);
let mut bob = Entity::new(...);

// Create and execute a transaction
match Transaction::create_and_execute(&mut alice, &mut bob, 50.0) {
    Ok(transaction) => {
        println!("Transaction successful!");
        println!("Alice balance: {}", alice.get_balance());
        println!("Bob balance: {}", bob.get_balance());
    }
    Err(e) => {
        println!("Transaction failed: {}", e);
    }
}
```

### Creating Blocks

```rust
use blockc::block::block::Block;
use blockc::transactions::transactions::Transaction;

// Create some transactions
let transactions = vec![transaction1, transaction2];

// Create a block
let block = Block::new(
    transactions,
    "0".to_string()  // Genesis block has "0" as previous hash
);

// Calculate block hash
let hash = block.calculate_hash();
println!("Block hash: {}", hash);
```

## Architecture Decisions

### Why Addresses Instead of Full Entities in Transactions?

**Problem**: If transactions stored full `Entity` objects, they would contain stale balance data when entities' balances change.

**Solution**: Store only addresses (strings) in transactions. This:
- Matches real blockchain design
- Prevents stale data issues
- Reduces memory usage
- Improves data consistency

### Why Separate Transaction Creation from Execution?

**Design**: `create_and_execute()` handles the full flow:
1. Validates sender has sufficient balance
2. Creates transaction data
3. Signs transaction
4. Updates balances
5. Records in history

This separation allows for:
- Testing transaction creation separately
- Future validation before execution
- Clear separation of concerns

### Hash Function Design

**Static vs Instance Methods**: 
- `hash()` - Static function that can be called without an instance (useful in `new()`)
- `calculate_hash()` - Instance method for convenience

This pattern allows hashing during block creation (when `self` doesn't exist yet) and later validation.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building

```bash
# Clone or navigate to project directory
cd blockc

# Build the project
cargo build

# Run (when main.rs is implemented)
cargo run

# Check for errors
cargo check
```

### Dependencies

- `sha2 = "0.10.9"` - SHA-256 hashing

## Learning Concepts Demonstrated

### Ownership & Borrowing
- `&self` vs `self` - When to borrow vs move
- References in function parameters
- Avoiding unnecessary clones

### Data Structures
- `Vec<T>` - Storing blocks and transactions
- Struct composition
- Module organization

### Cryptography
- SHA-256 hashing
- Digital signatures (simplified)
- Hash-based integrity

### Error Handling
- `Result<T, E>` for transaction validation
- Error propagation with `?`

## Next Steps

1. **Blockchain Struct** - Create the chain that holds blocks
2. **Genesis Block** - Special first block creation
3. **Proof of Work** - Mining algorithm with difficulty
4. **Chain Validation** - Verify blockchain integrity
5. **Persistence** - Save/load from file using serde

## Notes

- This is a learning project focused on understanding blockchain concepts and Rust
- The signing mechanism is simplified (SHA-256 hash) and not cryptographically secure for production
- Balance management is simplified - real blockchains calculate balances from transaction history
- Timestamps use `u32` which is sufficient until year 2106

## Contributing

This is a personal learning project, but suggestions and improvements are welcome!

## License

This project is for educational purposes only.

