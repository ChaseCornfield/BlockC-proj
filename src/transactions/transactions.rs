use crate::helpers::{self};
use crate::entity::entity::Entity;


/// Represents a transaction between two entities in the blockchain.
/// 
/// Transactions store only addresses (not full Entity objects) to:
/// - Avoid stale data issues
/// - Match real blockchain design
/// - Reduce memory usage
/// - Improve data consistency
/// 
/// # Fields
/// 
/// * `sender_address` - Address of the entity sending funds
/// * `receiver_address` - Address of the entity receiving funds
/// * `amount` - Amount being transferred
/// * `timestamp` - Unix timestamp when transaction was created
/// * `signature` - Digital signature created by the sender
/// 
/// # Example
/// 
/// ```
/// use blockc::transactions::transactions::Transaction;
/// use blockc::entity::entity::Entity;
/// 
/// // Create entities
/// let mut alice = Entity::new(...);
/// let mut bob = Entity::new(...);
/// 
/// // Create and execute transaction
/// let transaction = Transaction::create_and_execute(&mut alice, &mut bob, 50.0)?;
/// ```
#[derive(Debug, Clone)]
pub struct Transaction 
{
    pub sender_address: String,
    pub receiver_address: String,
    pub amount: f64,
    pub timestamp: u32,
    pub signature: String,

}

impl Transaction 
{
    fn new(sender_address: String, receiver_address: String, amount_tx: f64, signature: String, time_stamp: u32) -> Self
    {
        Transaction
        {
            sender_address: sender_address,
            receiver_address: receiver_address,
            amount: amount_tx,
            timestamp: time_stamp,
            signature: signature
        }

    }

    fn create_and_sign(sender: &Entity, receiver: &Entity, amount_tx: f64) -> Self{
        // create transaction
        let time_stamp = helpers::helper_functions::get_time();
        let transaction_data = format!("{}{}{}{}", 
            sender.address,      
            receiver.address,   
            amount_tx,          
            time_stamp          
        );

        // calling for signature
        let signature = sender.sign(&transaction_data);

        // create new transaction using new (store addresses only)
        Transaction::new(
            sender.address.clone(),
            receiver.address.clone(),
            amount_tx, 
            signature, 
            time_stamp
        )
    }

    
    /// Creates, signs, validates, and executes a transaction between two entities.
    /// 
    /// This method handles the complete transaction flow:
    /// 1. Validates sender has sufficient balance
    /// 2. Creates transaction data
    /// 3. Signs the transaction with sender's private key
    /// 4. Updates both entities' balances
    /// 5. Records transaction in both entities' histories
    /// 
    /// # Arguments
    /// 
    /// * `sender` - Mutable reference to the sending entity
    /// * `receiver` - Mutable reference to the receiving entity
    /// * `amount` - Amount to transfer
    /// 
    /// # Returns
    /// 
    /// * `Ok(Transaction)` - The created and executed transaction
    /// * `Err(String)` - Error message if validation fails (e.g., insufficient balance)
    /// 
    /// # Example
    /// 
    /// ```
    /// # use blockc::transactions::transactions::Transaction;
    /// # use blockc::entity::entity::Entity;
    /// # let mut alice = Entity::new("Alice".to_string(), 100.0, Vec::new(), "pub".to_string(), "priv".to_string());
    /// # let mut bob = Entity::new("Bob".to_string(), 50.0, Vec::new(), "pub2".to_string(), "priv2".to_string());
    /// match Transaction::create_and_execute(&mut alice, &mut bob, 25.0) {
    ///     Ok(tx) => println!("Transaction successful: {} -> {}", tx.sender_address, tx.receiver_address),
    ///     Err(e) => println!("Transaction failed: {}", e),
    /// }
    /// ```
    pub fn create_and_execute(sender: &mut Entity, receiver: &mut Entity, amount: f64) -> Result<Self, String> {
        // Validate sender has enough
        if !sender.can_send(amount) {
            return Err("Insufficient balance".to_string());
        }
        
        // Create and sign transaction (pass references, not clones)
        let transaction = Transaction::create_and_sign(
            sender,    // &Entity reference
            receiver,  // &Entity reference
            amount
        );
        
        // Update balances
        sender.send_amount(amount)?;
        receiver.receive_amount(amount);
        
        // Add to histories
        sender.add_to_history(transaction.clone());
        receiver.add_to_history(transaction.clone());
        
        Ok(transaction)
    }

}
