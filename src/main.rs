use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    hash: String,
    data: String,
    prev_hash: String,
    nonce: u64,
}
#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
}
#[derive(Debug, Clone)]
struct ProofOfWork {
    block: Block,
    target: String,
}

fn new_proof(b: &Block) -> ProofOfWork {
    ProofOfWork {
        block: b.clone(),
        target: String::from("000"),
    }
}

impl ProofOfWork {
    fn prepare_data(&self, nonce: u64) -> String {
        let mut data = String::new();
        data.push_str(&self.block.prev_hash);
        data.push_str(&self.block.data);
        data.push_str(&nonce.to_string());
        data
    }

    fn run(&self) -> (u64, String) {
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            let mut hasher = Sha256::new();
            let data = self.prepare_data(nonce);
            hasher.update(data);

            hash = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect();
            // print!("\r{} {}", hash, nonce);

            if hash.starts_with(&self.target) {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }
    fn validate(&self) -> bool {
        let mut hasher = Sha256::new();
        let data = self.prepare_data(self.block.nonce);
        hasher.update(data);

        let hash: String = hasher
            .finalize()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect();
        hash.starts_with(&self.target)
    }
}

fn create_block(data: String, prev_hash: String) -> Block {
    let mut block = Block {
        hash: String::new(),
        data: data,
        prev_hash: prev_hash,
        nonce: 0,
    };

    let pow = new_proof(&block);
    let (nonce, hash) = pow.run();
    block.nonce = nonce as u64;
    block.hash = hash;
    return block;
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }

    fn add_block(&mut self, data: String) {
        let prevHash = if self.blocks.len() == 0 {
            String::from("0")
        } else {
            self.blocks[self.blocks.len() - 1].hash.clone()
        };

        let block = create_block(data, prevHash);

        self.blocks.push(block);
    }
}

fn genesis() -> Block {
    let mut block = Block {
        hash: String::from(""),
        data: String::from("Genesis Block"),
        prev_hash: String::from(""),
        nonce: 0,
    };
    let pow = new_proof(&block);
    let (nonce, hash) = pow.run();
    block.nonce = nonce as u64;
    block.hash = hash;
    block
}

fn init_blockchain() -> Blockchain {
    Blockchain {
        blocks: vec![genesis()],
    }
}

fn main() {
    let mut bc = init_blockchain();
    bc.add_block(String::from("Hello"));
    bc.add_block(String::from("Nguyen"));

    let mut i = 0;
    for block in bc.blocks {
        println!("\nBlock {}", i);
        println!("Prev. hash: {}", block.prev_hash);
        println!("Data: {}", block.data);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        // validate
        let pow = new_proof(&block);
        println!("PoW: {}", pow.validate());
        i += 1;
    }
}
