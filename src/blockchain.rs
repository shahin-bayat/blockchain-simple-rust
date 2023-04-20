use crate::block::check_difficulty;
use std::collections::HashSet;

use super::*;
#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamps,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationError> {
        let i = self.blocks.len();
        let prev_block = &self.blocks[i - 1];

        // 1. Actual index = stored index value (note that Bitcoin blocks don't store their index)
        if block.index != i as u32 {
            return Err(BlockValidationError::MismatchedIndex);
            // 2. Block's hash fits stored difficulty value (we will trust the difficulty value for now ⚠️INSECURE)
        } else if !check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationError::InvalidHash);
        } else if i != 0 {
            // Not Genesis block:
            // 3. Time is always increasing
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationError::AchronologicalTimestamps);
            // 4. Actual prev_block_hash = stored prev_block_hash value (except for the genesis block)
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        } else {
            // Genesis block:
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationError::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                let output_hashes = transaction.output_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                // check for uncommon ones
                {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value > input_value {
                    return Err(BlockValidationError::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(output_hashes);
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));

            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);
        Ok(())
    }
}
