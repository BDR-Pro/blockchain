// Ensure you have these dependencies in your Cargo.toml:
// chrono = "0.4"
// serde = { version = "1.0", features = ["derive"] }
// sha2 = "0.10"
// openssl = "0.10"
// serde_json = "1.0"
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Sha256, Digest};
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::sign::{Signer, Verifier};
use std::fs::{self, DirBuilder};
use std::path::Path;
use std::path::PathBuf;

fn get_block_hash_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    // Read the file to a string
    let data = fs::read_to_string(path)?;

    // Parse the string as JSON
    let json: Value = serde_json::from_str(&data)?;

    // Access the "block_hash" field
    let block_hash = json
        .get("block_hash")
        .ok_or("The key 'block_hash' does not exist")?
        .as_str()
        .ok_or("The value for 'block_hash' is not a string")?
        .to_string();

    Ok(block_hash)
}

fn count_files_in_folder<P: AsRef<Path>>(path: P) -> std::io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            count += 1;
        }
    }
    Ok(count)
}


fn sign(message: &String) -> Vec<u8> {
    // Generate an EC key pair
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let ec_key = EcKey::generate(&group).unwrap();
    let pkey = PKey::from_ec_key(ec_key.clone()).unwrap(); // Clone ec_key for later use

    // Sign a message
    let message = message.as_bytes();
    let mut signer = Signer::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
    signer.update(message).unwrap();
    let signature = signer.sign_to_vec().unwrap();

    // Verify the signature (just for checking, usually done by the receiver)
    let mut verifier = Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
    verifier.update(message).unwrap();
    assert!(verifier.verify(&signature).unwrap());

    // Convert the private key and public key to PEM format and save them
    let private_key_pem = ec_key.private_key_to_pem().unwrap();

    // Define the directory and file paths
    let dir_path = Path::new("my_keys");
    let file_path = dir_path.join(format!("private_key_{}.pem", Utc::now().timestamp()));

    // Create the directory if it does not exist
    if !dir_path.exists() {
        DirBuilder::new()
            .recursive(true) // This option will create all required parent directories as well
            .create(dir_path)
            .expect("Unable to create directory");
    }

    // Write the private key to the file
    fs::write(&file_path, &private_key_pem).expect("Unable to save private key");

    println!("Signature and key have been successfully generated and saved.");

    signature // Return the signature
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: i64,
    data: String,
    previous_hash: String,
    reward: u64,
    block_number: u64,
    content_hash: String,
    signature: Vec<u8>,
    block_hash: String,
}

impl Block {
    // This function must return a Result containing the Block, adjust its signature and fix implementation
    fn new(data: String, reward:u64) -> Result<Block, &'static str> {
        let block_number: u64 = count_files_in_folder("my_blocks").unwrap() as u64; // Assuming block numbers start from 0
        let previous_hash: String = get_block_hash_from_file(format!("my_blocks/{}.json", block_number - 1)).unwrap_or("0".to_string());
        let timestamp: i64 = Utc::now().timestamp();
        let mut contents: String = format!("{}:{}:{}:{}:{}", timestamp, &data, &previous_hash, &block_number, &reward);
        let mut hasher = Sha256::new();
        let mut new_hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let hash_result = hasher.finalize();
        let content_hash = format!("{:x}", hash_result);
        let signature = sign(&content_hash); // Ensure this is a String or change the return type of sign function
        contents.push_str(&content_hash);
        new_hasher.update(contents.as_bytes());
        let hash_after_signature = new_hasher.finalize();
        let block_hash: String = format!("{:x}", hash_after_signature);
        Ok(Block {
            timestamp,
            data,
            previous_hash,
            reward,
            block_number,
            content_hash,
            signature,
            block_hash,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        // Handle the Result returned by Block::new using unwrap or expect
        let genesis_block = Block::new("Genesis Block".to_string(),28*28).unwrap();    
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    fn calculate_reward(block_number: u64) -> u64 {
        let reward: u64 = 28*28;
        reward >> (block_number / 2^16)
        
    }   
    fn add_block(&mut self, data: String) -> Result<(), &'static str> {

        
        let block_number: u64 = self.chain.len() as u64; // Assuming block numbers start from 0
        let reward = Self::calculate_reward(block_number); // Assuming this is implemented elsewhere
        let new_block: Block = Block::new(data, reward)
            .map_err(|_| "Failed to create new block")?;
        
        self.chain.push(new_block);

        // Serialize the new block to a JSON string
        let json_str = serde_json::to_string(&self.chain.last().unwrap()) // It's safe to unwrap here since we just added a block
            .map_err(|_| "Failed to serialize block")?;

        // Construct the file path using PathBuf
        let mut file_path = PathBuf::from("my_blocks");
        file_path.push(format!("{}.json", block_number)); // Using format! macro for filename

        // Ensure the directory exists
        DirBuilder::new()
            .recursive(true)
            .create(file_path.parent().ok_or("Failed to get parent directory")?)
            .map_err(|_| "Failed to create directory")?;

        // Write the JSON string to the file
        fs::write(&file_path, json_str).map_err(|_| "Failed to write block to file")?;

        Ok(())
    }

}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block Transactions Data".to_string()).unwrap();
    blockchain.add_block("Block Transactions Data".to_string()).unwrap();

    println!("Blockchain: {:?}", blockchain);
}
