use chrono::prelude::*;
use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain{
    pub genesisBlock: Block,
    pub chain: Blocks,
    pub difficulty: usize    
}

impl Blockchain{
    pub fn new(difficulty: usize)->Self {
        let mut genesisBlock = Block{
            index:0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default()
        };

        let mut chain = Vec::new();
        chain.push(genesisBlock.clone());
        let blockchain = Blockchain{
            genesisBlock,
            chain,
            difficulty
        };
        blockchain
    }

    pub fn addBlock(&mut self){
        let mut newBlock = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].previous_hash.clone()
        );
        newBlock.mine(self.clone());
        self.chain.push(newBlock.clone());
        println!("New block -> {:?}",newBlock);
    }
}

