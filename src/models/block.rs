use chrono::prelude::*;
use super::blockchain::Blockchain;
use sha2::{Sha256,Digest};
use serde::{Deserialize,Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block{
      pub index: u64,
      pub timestamp: u64,
      pub proof_of_work: u64,
      pub previous_hash: String,
      pub hash: String 
}

impl Block{
    pub fn generate_block_hash(&self) -> String{
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serializedBlockData = serde_json::to_string(&block_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serializedBlockData);
        let result = hasher.finalize();
        format!("{:x}",result)
    }

    pub fn new(_index:u64,previous_hash:String) -> Self{
        let mut block = Block{
            index:_index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default()
        };
        block   
    }
    
    pub fn mine(&mut self, blockchain:Blockchain){
        loop{
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work +=1;
                self.hash = self.generate_block_hash();
            }
            else{
                break
            }
        }
    }
}   