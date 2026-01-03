use crate::helpers::{self};
use sha2::{Sha256, Digest}; // hashing
use crate::transactions::transactions::Transaction;

/// Represents a block in the blockchain.
/// 
/// Each block contains:
/// - A hash of its own data
/// - A hash of the previous block (linking blocks together)
/// - A list of transactions
/// - A timestamp
/// - A nonce (used for proof-of-work mining)
/// 
/// # Fields
/// 
/// * `block_hash` - SHA-256 hash of this block's data
/// * `previous_block_hash` - Hash of the previous block in the chain
/// * `transaction` - Vector of transactions included in this block
/// * `time_stamp` - Unix timestamp when block was created (u32, valid until 2106)
/// * `nonce` - Proof-of-work value (will be used for mining)
/// 
/// # Hash Calculation
/// 
/// The block hash is calculated from:
/// - Previous block hash
/// - Timestamp
/// - Nonce
/// - All transaction data (sender, receiver, amount, timestamp)
pub struct Block{

    pub block_hash: String,
    pub previous_block_hash: String,
    pub transaction: Vec<Transaction>,
    pub time_stamp: u32, // small because of project size, good until 2106
    pub nonce: u32,

}


impl Block{
    pub fn new(transaction: Vec<Transaction>, previous_block_hash: String) -> Self
    {
        let time_stamp = helpers::helper_functions::get_time();
        let nonce = 0;

        // create hash using helper fn
        let block_hash = Block::hash(
        &previous_block_hash,
        time_stamp,
        nonce,
        &transaction
        );

        // create block
        Block {
            block_hash: block_hash,
            previous_block_hash: previous_block_hash,
            transaction: transaction,
            time_stamp: time_stamp,
            nonce: nonce,
        }
    }


    /// Calculates the SHA-256 hash of a block's data.
    /// 
    /// This is a static function that can be called without a Block instance,
    /// which is useful when creating a new block (before `self` exists).
    /// 
    /// # Arguments
    /// 
    /// * `previous_hash` - Hash of the previous block
    /// * `time_stamp` - Block creation timestamp
    /// * `nonce` - Proof-of-work nonce value
    /// * `transaction` - Vector of transactions in the block
    /// 
    /// # Returns
    /// 
    /// A hexadecimal string representing the SHA-256 hash.
    pub fn hash(previous_hash: &str, time_stamp: u32, nonce: u32, transaction: &Vec<Transaction>) -> String
    {
        let mut hasher = Sha256::new();
        
        // Convert transaction vector to a string representation
        let transaction_str: String = transaction.iter()
            .map(|t| format!("{}{}{}{}", 
                t.sender_address, 
                t.receiver_address, 
                t.amount, 
                t.timestamp
            ))
            .collect::<Vec<String>>()
            .join(",");
        
        let data_to_hash = format!("{}{}{}{}",
                                previous_hash,
                                time_stamp,
                                nonce,
                                transaction_str);
        hasher.update(data_to_hash.as_bytes());
        let result = hasher.finalize();
        result.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    /// Calculates the hash of this block instance.
    /// 
    /// Convenience method that calls the static `hash()` function with
    /// this block's data.
    /// 
    /// # Returns
    /// 
    /// A hexadecimal string representing the SHA-256 hash of this block.
    pub fn calculate_hash(&self) -> String {
        Block::hash(
            &self.previous_block_hash,
            self.time_stamp,
            self.nonce,
            &self.transaction
        )
    }
}