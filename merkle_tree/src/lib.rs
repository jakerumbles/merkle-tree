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
            // println!("{}", val);

            // Hash the serialized transaction `val`
            let hash = MerkleTree::hash_single(&val);

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

    /// Construct the Merkle Tree from the `leaves`. Once complete, returns the merkle root hash.
    pub fn build_tree(&mut self) -> u64 {
        // much wow 🐕
        for i in (0..self.leaves.len()).step_by(2) {
            println!("i: {}, Node: {:?}", i, self.leaves[i]);

            let hash1 = self.leaves[i].hash;
            let hash2 = self.leaves[i + 1].hash;

            // Compute new hash from 2 child hashes
            let combined_hash = MerkleTree::hash_double(hash1, hash2);

            // Create new MerkleNode from `combined_hash`
            let new_node = Rc::new(MerkleNode::new(combined_hash, None));

            println!("Strong Count: {}", Rc::strong_count(&new_node));

            // Link child nodes to `new_node`
            let node1 = &mut self.leaves[i];
            node1.set_pointer(Rc::clone(&new_node));
            println!("Strong Count: {}", Rc::strong_count(&new_node));

            let mut node2 = &mut self.leaves[i + 1];
        }

        25
    }

    fn hash_single(val: &String) -> u64 {
        // Need to create a new hasher each loop because they save state. Need to start fresh each hash.
        let mut hasher = DefaultHasher::new();

        // Feed value into the hasher
        val.hash(&mut hasher);

        // Actually hash the value
        hasher.finish()
    }

    fn hash_double(val1: u64, val2: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        val1.hash(&mut hasher);
        val2.hash(&mut hasher);
        hasher.finish()
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

    fn set_pointer(&mut self, node: Rc<MerkleNode>) {
        self.pointer = Some(node);
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
