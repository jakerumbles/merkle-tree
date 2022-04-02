use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: i32,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: i32) -> Self {
        Transaction { from, to, amount }
    }
}

#[derive(Debug)]
pub struct Transactions(Vec<String>);

// Assume even number of transactions to start
impl Transactions {
    /// Accepts a vector of type `Transaction` and serializes each transaction, returning a `Transactions` instance holding the vector of `Transaction`s.
    /// # Future Upgrade
    /// Make `transactions` argument only accept a vector holding types that implement `Serialize` and `Deserialize`.
    pub fn new(transactions: Vec<Transaction>) -> Self {
        // Serialize each transaction
        let serialized_transactions: Vec<String> = transactions
            .iter()
            .map(|t| match serde_json::to_string(&t) {
                Ok(val) => val,
                Err(e) => panic!("Error serializing: {}", e),
            })
            .collect();

        Transactions(serialized_transactions)
    }
}

pub struct MerkleTree {
    merkle_root: Option<Rc<MerkleNode>>,
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    /// Hash all transactions provided and create `MerkleNode`s for each corresponding hash. Finally store `MerkleNode`s in `leaves` vector.
    pub fn new(leaves: Transactions) -> Self {
        let mut hashed_transactions: Vec<MerkleNode> = vec![];

        for (_, val) in leaves.0.iter().enumerate() {
            println!("{}", val);

            // Need to create a new hasher each loop because they save state. Need to start fresh each hash.
            let mut hasher = DefaultHasher::new();
            // Feed value into the hasher
            val.hash(&mut hasher);
            // Actually hash the value
            let hash = hasher.finish();

            let new_node = MerkleNode::new(hash, None);
            hashed_transactions.push(new_node);
        }

        MerkleTree {
            merkle_root: None,
            leaves: hashed_transactions,
        }
    }

    pub fn merkle_root(&self) -> &Option<Rc<MerkleNode>> {
        &self.merkle_root
    }

    pub fn leaves(&self) -> &Vec<MerkleNode> {
        &self.leaves
    }

    pub fn build_tree() {
        // much wow 🐕
    }
}

#[derive(Debug)]
pub struct MerkleNode {
    hash: u64,
    pointer: Option<Rc<MerkleNode>>,
}

impl MerkleNode {
    fn new(hash: u64, pointer: Option<Rc<MerkleNode>>) -> Self {
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
