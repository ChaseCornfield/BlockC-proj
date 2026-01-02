use crate::transactions::transactions::Transaction;
use sha2::{Sha256, Digest};



/// Minimum allowed balance for an entity
pub const MIN_BALANCE:f64 = 0.0 ;

/// Represents a participant in the blockchain (like a wallet or user).
/// 
/// An Entity has an address, balance, transaction history, and cryptographic keys
/// for signing transactions.
/// 
/// # Fields
/// 
/// * `address` - Unique identifier for this entity
/// * `balance` - Current balance (in a simplified model, stored directly)
/// * `history` - Vector of all transactions this entity has participated in
/// * `public_key` - Public key for transaction verification
/// * `private_key` - Private key for signing transactions
/// 
/// # Example
/// 
/// ```
/// use blockc::entity::entity::Entity;
/// 
/// let entity = Entity::new(
///     "Alice".to_string(),
///     100.0,
///     Vec::new(),
///     "public_key".to_string(),
///     "private_key".to_string(),
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Entity{
    pub address: String,
    pub balance: f64,
    pub history: Vec<Transaction>,
    pub public_key: String,
    pub private_key: String,
}



impl Entity{
    fn new(address: String, balance: f64, history: Vec<Transaction>, public_key: String, private_key: String) -> Self{
        Entity{
            address: address,
            balance: balance,
            history: history,
            public_key: public_key,
            private_key: private_key,
        }
    }


    /// Returns the current balance of this entity.
    /// 
    /// # Returns
    /// 
    /// The current balance as `f64`.
    pub fn get_balance(&self) -> f64
    {
        self.balance
    }

    /// Checks if the entity has sufficient balance to send the specified amount.
    /// 
    /// # Arguments
    /// 
    /// * `amount` - The amount to check
    /// 
    /// # Returns
    /// 
    /// `true` if balance is sufficient, `false` otherwise.
    pub fn can_send(&self, amount: f64) -> bool {
        self.balance >= amount
    }

    /// Deducts the specified amount from the entity's balance.
    /// 
    /// This method validates that the entity has sufficient balance before deducting.
    /// 
    /// # Arguments
    /// 
    /// * `amount` - The amount to deduct
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If deduction was successful
    /// * `Err(String)` - If insufficient balance, contains error message
    /// 
    /// # Example
    /// 
    /// ```
    /// # use blockc::entity::entity::Entity;
    /// # let mut entity = Entity::new("Alice".to_string(), 100.0, Vec::new(), "pub".to_string(), "priv".to_string());
    /// match entity.send_amount(50.0) {
    ///     Ok(()) => println!("Balance deducted successfully"),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn send_amount(&mut self, amount: f64) -> Result<(), String> {
        if !self.can_send(amount) {
            return Err(format!("Insufficient balance. Have: {}, Need: {}", self.balance, amount));
        }
        self.balance -= amount;
        Ok(())
    }

    /// Adds the specified amount to the entity's balance.
    /// 
    /// Used when the entity receives funds from a transaction.
    /// 
    /// # Arguments
    /// 
    /// * `amount` - The amount to add
    pub fn receive_amount(&mut self, amount: f64) {
        self.balance += amount;
    }

    /// Adds a transaction to the entity's transaction history.
    /// 
    /// # Arguments
    /// 
    /// * `transaction` - The transaction to add to history
    pub fn add_to_history(&mut self, transaction: Transaction) {
        self.history.push(transaction);
    }

    /// Signs transaction data using the entity's private key.
    /// 
    /// Creates a digital signature by hashing the transaction data combined with
    /// the private key using SHA-256.
    /// 
    /// # Arguments
    /// 
    /// * `transaction_data` - The transaction data to sign (as a string)
    /// 
    /// # Returns
    /// 
    /// A hexadecimal string representing the signature.
    /// 
    /// # Note
    /// 
    /// This is a simplified signing mechanism for learning purposes.
    /// Real blockchains use more sophisticated cryptographic signatures (ECDSA, RSA, etc.).
    pub fn sign(&self, transaction_data: &str) -> String{
        let data_to_sign = format!("{}{}", transaction_data, self.private_key);
        
        // Hash it
        let mut hasher = Sha256::new();
        hasher.update(data_to_sign.as_bytes());
        let result = hasher.finalize();
        
        // Convert to hex string (signature)
        result.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }
    
}