// Ensure you have these dependencies in your Cargo.toml:
// chrono = "0.4"
// serde = { version = "1.0", features = ["derive"] }
// sha2 = "0.10"
// openssl = "0.10"
// serde_json = "1.0"
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::sign::{Signer, Verifier};

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
    let public_key_pem = ec_key.public_key_to_pem().unwrap();
    std::fs::write("private_key.pem", &private_key_pem).expect("Unable to save private key");
    std::fs::write("public_key.pem", &public_key_pem).expect("Unable to save public key");

    println!("Signature and keys have been successfully generated and saved.");

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
    fn new(data: String, previous_hash: String, block_number:u64 , reward:u64) -> Result<Block, &'static str> {
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
        let genesis_block = Block::new("Genesis Block".to_string(),
         "".to_string(),0,28*28).unwrap();
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    fn calculate_reward(block_number: u64) -> u64 {
        let reward: u64 = 28*28;
        reward >> (block_number / 2^16)
    }
    // This function must handle the Result type returned by Block::new
    fn add_block(&mut self, data: String) -> Result<(), &'static str> {
        let previous_hash: String = self.chain.last().ok_or("Chain should have at least one block")?.block_hash.clone();
        let block_number: u64 = self.chain.len() as u64;
        let reward =Self::calculate_reward(block_number);
        let new_block: Block = Block::new(data, previous_hash, reward , block_number)?;
        self.chain.push(new_block);
        Ok(())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block 1 Data".to_string()).unwrap();
    blockchain.add_block("Block 2 Data".to_string()).unwrap();

    println!("Blockchain: {:?}", blockchain);
}
