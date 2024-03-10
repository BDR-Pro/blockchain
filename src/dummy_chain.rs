use blockchain_maker::Blockchain;

fn main() {
    println!("Hello, world!");
    let mut blockchain: Blockchain = match Blockchain::load_chain_from_disk("my_blocks".to_string(), 2^16, 50) {
        Ok(chain) => chain,
        Err(e) => {
            // Handle the error e.g., by logging or creating a new, empty blockchain
            println!("Failed to load chain from disk, error: {}", e);
            // Potentially initialize a new, empty blockchain here if desired
            Blockchain::new(50,2^16) // This assumes you have a `new` method to create an empty blockchain
        },
    };
    if let Err(e) = blockchain.add_block("Block 1 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }

    if let Err(e) = blockchain.add_block("Block 2 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }
    if let Err(e) = blockchain.add_block("Block 3 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }

    if let Err(e) = blockchain.add_block("Block 4 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }
}


