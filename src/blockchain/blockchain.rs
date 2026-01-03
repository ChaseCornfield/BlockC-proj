use crate::block::block::Block;
use crate::transactions::transactions::Transaction;
pub struct Blockchain {
    chain: Vec<Block>,        // The chain of blocks
    difficulty: u32,          // Proof-of-work difficulty (number of leading zeros)
}


impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    /// 
    /// The genesis block is the first block in the chain and has:
    /// - No transactions (empty vector)
    /// - Previous hash of "0" (indicating it's the first block)
    /// 
    /// # Returns
    /// 
    /// A new `Blockchain` instance with a genesis block and default difficulty of 3.
    pub fn new() -> Self {
        // Create genesis block (first block in the chain)
        let genesis = Block::new(
            Vec::new(),              // No transactions in genesis block
            "0".to_string()          // Previous hash is "0" for genesis
        );
        
        Blockchain {
            chain: vec![genesis],    // Initialize chain with genesis block
            difficulty: 3,            // Default difficulty (3 leading zeros)
        }
    }
    
    /// Returns a reference to the latest block in the chain.
    /// 
    /// # Returns
    /// 
    /// A reference to the last block in the chain.
    /// 
    /// # Panics
    /// 
    /// Panics if the chain is empty (should never happen after genesis block is created).
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    /// Gets the hash of the latest block as a string.
    /// 
    /// This is a convenience method to get just the hash without needing to access the block.
    /// 
    /// # Returns
    /// 
    /// The hash of the latest block as a `String`.
    pub fn get_latest_hash(&self) -> String {
        self.chain.last().unwrap().block_hash.clone()
    }

    /// Adds a new block to the blockchain.
    /// 
    /// This method:
    /// 1. Gets the previous block's hash
    /// 2. Creates a new block with the given transactions
    /// 3. Adds it to the chain
    /// 
    /// Note: Proof-of-work mining is not yet implemented, so blocks are added immediately.
    /// 
    /// # Arguments
    /// 
    /// * `transactions` - Vector of transactions to include in the new block
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        // Get previous block's hash
        let previous_hash = self.get_latest_hash();
        
        // Create new block
        let new_block = Block::new(transactions, previous_hash);
        
        // Add to chain
        self.chain.push(new_block);
    }
    
    /// Validates the integrity of the entire blockchain.
    /// 
    /// This method checks:
    /// 1. Each block's stored hash matches its calculated hash
    /// 2. Each block's `previous_hash` matches the previous block's hash
    /// 3. Genesis block has the correct previous hash ("0")
    /// 
    /// # Returns
    /// 
    /// * `true` - If the chain is valid
    /// * `false` - If any validation check fails
    pub fn is_valid(&self) -> bool {
        // Check if chain is empty
        if self.chain.is_empty() {
            return false;
        }
        
        // Validate genesis block
        let genesis = &self.chain[0];
        if genesis.previous_block_hash != "0" {
            return false;
        }
        
        // Check if genesis block's hash matches its calculated hash
        if genesis.block_hash != genesis.calculate_hash() {
            return false;
        }
        
        // Validate the rest of the chain
        // Use windows(2) to get pairs of consecutive blocks
        for window in self.chain.windows(2) {
            let previous = &window[0];
            let current = &window[1];
            
            // Check if current block's hash matches its calculated hash
            if current.block_hash != current.calculate_hash() {
                return false;
            }
            
            // Check if current block's previous_hash matches previous block's hash
            if current.previous_block_hash != previous.block_hash {
                return false;
            }
        }
        
        true
    }
}