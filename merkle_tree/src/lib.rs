use serde::{Deserialize, Serialize};
use std::cell::RefCell;
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
    merkle_root: Option<Rc<RefCell<MerkleNode>>>,
    nodes: Vec<Vec<Rc<RefCell<MerkleNode>>>>,
    tree_height: usize,
}

impl MerkleTree {
    /// Hash all transactions provided and create `MerkleNode`s for each corresponding hash. Finally store `MerkleNode`s in `leaves` vector.
    pub fn new(leaves: Transactions) -> Self {
        let mut hashed_transactions: Vec<Rc<RefCell<MerkleNode>>> = vec![];

        for (_, val) in leaves.0.iter().enumerate() {
            // println!("{}", val);

            // Hash the serialized transaction `val`
            let hash = MerkleTree::hash_single(&val);

            let new_node = Rc::new(RefCell::new(MerkleNode::new(hash, None)));
            hashed_transactions.push(Rc::clone(&new_node));
        }

        // Hardcoded for 4 transactions
        let tree_height = MerkleTree::compute_tree_height(hashed_transactions.len());
        println!("\nTree Height: {}\n", tree_height);

        // Create vector of vectors. Top level is each level of tree. Inner lever is for holding all `MerkleNodes` in each level.
        let mut hashed_transactions = vec![hashed_transactions];
        for _ in 1..tree_height {
            hashed_transactions.push(vec![]);
        }
        MerkleTree {
            merkle_root: None,
            nodes: hashed_transactions,
            tree_height,
        }
    }

    pub fn merkle_root(&self) -> &Option<Rc<RefCell<MerkleNode>>> {
        &self.merkle_root
    }

    pub fn nodes(&self) -> &Vec<Vec<Rc<RefCell<MerkleNode>>>> {
        &self.nodes
    }

    /// Construct the Merkle Tree from the `leaves`. Once complete, returns the merkle root hash.
    pub fn build_tree(&mut self) -> u64 {
        // much wow 🐕

        let mut new_node: Rc<RefCell<MerkleNode>> =
            Rc::new(RefCell::new(MerkleNode::new(420, None)));
        let mut combined_hash: u64 = 0;

        for layer in 0..self.tree_height - 1 {
            for i in (0..self.nodes[layer].len()).step_by(2) {
                println!(
                    "Creating new node. Layer: {}, i: {}, i+1: {}",
                    layer,
                    i,
                    i + 1
                );

                let hash1 = self.nodes[layer][i].borrow().hash;
                let hash2 = self.nodes[layer][i + 1].borrow().hash;

                // Compute new hash from 2 child hashes
                combined_hash = MerkleTree::hash_double(hash1, hash2);

                // Create new MerkleNode from `combined_hash`
                new_node = Rc::new(RefCell::new(MerkleNode::new(combined_hash, None)));

                // Push new node into `layer` + 1 vec
                self.nodes[layer + 1].push(Rc::clone(&new_node));

                // Link child nodes to `new_node`
                let node1 = &mut self.nodes[layer][i].borrow_mut();
                node1.set_pointer(Rc::clone(&new_node));

                let node2 = &mut self.nodes[layer][i + 1].borrow_mut();
                node2.set_pointer(Rc::clone(&new_node));
            }
        }

        // This means we have finished computing the merkle root and can return the final hash
        self.merkle_root = Some(Rc::clone(&new_node));
        combined_hash
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

    pub fn compute_tree_height(num_transactions: usize) -> usize {
        let height = ((num_transactions as f64).log2() as usize) + 1;
        height
    }
}

#[derive(Debug)]
pub struct MerkleNode {
    hash: u64,
    pointer: Option<Rc<RefCell<MerkleNode>>>,
}

impl MerkleNode {
    fn new(hash: u64, pointer: Option<Rc<RefCell<MerkleNode>>>) -> Self {
        MerkleNode { hash, pointer }
    }

    fn set_pointer(&mut self, node: Rc<RefCell<MerkleNode>>) {
        self.pointer = Some(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_merkle_root() {
        let vec_transactions: Vec<Transaction> = vec![
            Transaction::new(String::from("Bob"), String::from("Alice"), 12),
            Transaction::new(String::from("Alice"), String::from("Jake"), 25),
            Transaction::new(String::from("Jake"), String::from("Bob"), 7),
            Transaction::new(String::from("Eric"), String::from("Bob"), 82),
        ];

        let transactions = Transactions::new(vec_transactions);

        let mut merkle_tree = MerkleTree::new(transactions);

        let merkle_root = merkle_tree.build_tree();

        assert_eq!(merkle_root, 7978775544804379803);
    }
}
