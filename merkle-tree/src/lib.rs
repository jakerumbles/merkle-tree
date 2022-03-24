use serde::{Deserialize, Serialize};
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: i32,
}

impl Transaction {
    fn new(from: String, to: String, amount: i32) -> Self {
        Transaction { from, to, amount }
    }
}

struct Transactions(Vec<Transaction>);

// Assume even number of transactions to start
impl Transactions {
    fn new() -> Self {
        let transactions: Vec<Transaction> = vec![
            Transaction::new(String::from("Bob"), String::from("Alice"), 12),
            Transaction::new(String::from("Alice"), String::from("Jake"), 25),
            Transaction::new(String::from("Jake"), String::from("Bob"), 7),
            Transaction::new(String::from("Eric"), String::from("Bob"), 82),
        ];

        Transactions(transactions)
    }
}

struct MerkleTree {
    merkle_root: Option<Rc<MerkleNode>>,
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    /// Hash all transactions provided and create `MerkleNode`s for each corresponding hash. Finally store `MerkleNode`s in `leaves` vector.
    fn new(&mut self, leaves: Transactions) -> Self {
        self.merkle_root = None;
        self.leaves = vec![];

        for (i, val) in leaves.0.iter().enumerate() {
            let serialized = serde_json::to_string(&val).unwrap();
            let mut hasher = Sha256::new();
            hasher.update(serialized.as_bytes());
            let hash = hasher.finalize();
            let x: u32 = hash;
            // Create `MerkleNode`
            MerkleNode::new()
        }

        MerkleTree {
            merkle_root: None,
            leaves: vec![],
        }
    }
}

struct MerkleNode {
    hash: GenericArray,
    pointer: Option<Rc<MerkleNode>>,
}

impl MerkleNode {
    fn new(hash: String, pointer: Option<Rc<MerkleNode>>) -> Self {
        MerkleNode { hash, pointer }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
