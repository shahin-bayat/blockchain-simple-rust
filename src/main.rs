use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", genesis_block);

    // let mut prev_block_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");
}
