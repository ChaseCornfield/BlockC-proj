# 1. Block
 I will start now by creatting a block, so lets understand first what is the block for us

 1. Pure Data
 2. Self conteined
 3. Owened by the blockchain
 4. Not aware of anything else

 - disclaimer: Source (gtp)

 A critical part is puttign the block as a separate module. THe block should never knwo that the block chaing exists

 Why??

 Lets break it down
 What does knowing that the blockchains exits means?
 -When a block stores a reference point to the blockchain
 - when a block stores a ref to another block
 -calls method on the block chain
 - assumes it is part of the chain

 # You should be able to say:

“Here is a block. Verify it using only its data.”

 # IN RUST

 Is even more dangerous because of the ownership factor
 Rust’s rule:

Ownership must flow in one direction.

Blockchain → Block
Never Block → Blockchain.