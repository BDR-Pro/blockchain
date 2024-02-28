use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::error::Error;
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::signature;
use ring::signature::KeyPair;
use ring::signature::UnparsedPublicKey;
use ring::signature::EcdsaKeyPair;
use ring::signature::Ed25519KeyPair;
use ring::signature::ECDSA_P256_SHA256_ASN1;
use ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING;
use ring::signature::ED25519;


#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    signature: Vec<u8>,
    block_hash : String,
}

impl Block {
    fn new(data: String, previous_hash: String, keypair: &Keypair) -> Result<Self, Box<dyn Error>> {
        let timestamp: i64 = Utc::now().timestamp();
        let contents = format!("{}:{}:{}", timestamp, &data, &previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let hash_result = hasher.finalize();
        let hash_str = format!("{:x}", hash_result);
        let signature = keypair.sign(&hash_result);
        contents.push_str(&hash_str);
        contents.push_str(&signature.to_bytes().to_vec());
        hasher.update(contents.as_bytes());
        let hash_after_signature = hasher.finalize();
        let block_hash: String = format!("{:x}", hash_after_signature);
        Ok(Block {
            timestamp,
            data,
            previous_hash,
            hash: hash_str,
            signature: signature.to_bytes().to_vec(),
            block_hash: block_hash ,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        
        let genesis_block = Block::new("Genesis Block".to_string(), "".to_string(), &key_pair).unwrap(); // Handle this unwrap better in real code
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String, keypair: &Keypair) -> Result<(), Box<dyn Error>> {
        let previous_hash = self.chain.last().ok_or("Chain should have at least one block")?.hash.clone();
        let new_block = Block::new(data, previous_hash, keypair)?;
        self.chain.push(new_block);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let rand = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rand)?; // pkcs8 format used for persistent storage
    let keypair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, pkcs8_bytes.as_ref(), &rand).map_err(|_| Unspecified)?;

    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block 1 Data".to_string(), &keypair)?;
    blockchain.add_block("Block 2 Data".to_string(), &keypair)?;

    println!("Blockchain: {:?}", blockchain);
    Ok(())
}
