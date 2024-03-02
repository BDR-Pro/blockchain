// Ensure the following dependencies are in your Cargo.toml:
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
use std::path::{Path, PathBuf};

fn get_block_hash_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    if count_files_in_folder(path.as_ref().parent().unwrap())? == 0 {
        return Ok("0".to_string());
    }
    let data = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&data)?;
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

fn sign(message: &str, reward:u64) -> Vec<u8> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let ec_key = EcKey::generate(&group).unwrap();
    let pkey = PKey::from_ec_key(ec_key.clone()).unwrap();

    let mut signer = Signer::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
    signer.update(message.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();

    let mut verifier = Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
    verifier.update(message.as_bytes()).unwrap();
    assert!(verifier.verify(&signature).unwrap());

    let private_key_pem = ec_key.private_key_to_pem().unwrap();
    let dir_path = Path::new("my_keys");
    let file_path = dir_path.join(format!("private_key_{}_{}.pem", Utc::now().timestamp(),reward));

    if !dir_path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(dir_path)
            .expect("Unable to create directory");
    }

    fs::write(&file_path, &private_key_pem).expect("Unable to save private key");
    println!("Signature and private key have been successfully generated and saved.");
    signature
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
    fn new(data: String, previous_hash: String, block_number: u64, reward: u64) -> Result<Block, &'static str> {
        let timestamp = Utc::now().timestamp();
        let contents = format!("{}:{}:{}:{}:{}", timestamp, data, previous_hash, block_number, reward);
        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());
        let signature = sign(&content_hash, reward);

        let mut hasher_with_signature = Sha256::new();
        hasher_with_signature.update(format!("{}:{}", contents, &content_hash).as_bytes());
        let block_hash = format!("{:x}", hasher_with_signature.finalize());

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
        let genesis_block = Block::new(
            "Genesis Block".to_string(), 
            "0".to_string(), 
            0, 
            Self::calculate_reward(0)
        ).expect("Failed to create the genesis block");

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn calculate_reward(block_number: u64) -> u64 {
        // Reward starts at 784 (28 * 28).
        // Shift the reward right by one (halve it) every 65536 blocks.
        784 >> (block_number / 65536)
    }

    fn add_block(&mut self, data: String) -> Result<(), &'static str> {
        let mut block_number = count_files_in_folder("my_blocks").map_err(|_| "Failed to count files in folder")? as u64;
        block_number += 1;
        println!("Block number: {}", block_number);
        let previous_hash = get_block_hash_from_file(Path::new("my_blocks").join(format!("{}.json", block_number - 1))).map_err(|_| "Failed to read previous block hash from file")?;
        println!("Previous hash: {}", previous_hash);
        let reward = Self::calculate_reward(block_number);
        println!("Reward: {}", reward);
        let new_block = Block::new(data, previous_hash, block_number, reward)?;
    
        // Here, directly handle the Result returned by serde_json::to_string
        let json_str = serde_json::to_string(&new_block).map_err(|_| "Failed to serialize block")?;
    
        let mut file_path = PathBuf::from("my_blocks");
        file_path.push(format!("{}.json", block_number));
    
        // Ensure the directory exists before writing the file
        if let Some(parent) = file_path.parent() {
            DirBuilder::new()
                .recursive(true)
                .create(parent)
                .map_err(|_| "Failed to create directory")?;
        }
    
        // Now you can pass json_str directly since it's already a String
        fs::write(&file_path, json_str.as_bytes()).map_err(|_| "Failed to write block to file")?;
    
        // Add the new block to the chain
        self.chain.push(new_block);
        Ok(())
    }

    fn validate_chain(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            if block.previous_hash != self.chain[i - 1].block_hash {
                return false;
            }
        }
        true
    }
    fn load_chain_from_disk() -> Result<Blockchain, &'static str> {
        let mut chain = vec![];
        let mut i = 0;
        loop {
            let file_path = Path::new("my_blocks").join(format!("{}.json", i));
            if !file_path.exists() {
                break;
            }
            let block_hash = get_block_hash_from_file(&file_path).map_err(|_| "Failed to read block hash from file")?;
            let block = Block {
                block_hash,
                ..serde_json::from_str(&fs::read_to_string(&file_path).map_err(|_| "Failed to read block from file")?).map_err(|_| "Failed to deserialize block")?
            };
            chain.push(block);
            i += 1;
        }
        Ok(Blockchain { chain })
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    Blockchain::load_chain_from_disk().expect("Failed to load chain from disk");
    blockchain.validate_chain();
    blockchain.add_block("Block 1 Transactions Data".to_string()).expect("Failed to add block");
    blockchain.add_block("Block 2 Transactions Data".to_string()).expect("Failed to add block");

    println!("Blockchain: {:?}", blockchain);
}
