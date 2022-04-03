use merkle_tree::{MerkleTree, Transaction, Transactions};

fn main() {
    let test = Transaction::new(String::from("Bob"), String::from("Alice"), 12);

    let vec_transactions: Vec<Transaction> = vec![
        Transaction::new(String::from("Bob"), String::from("Alice"), 12),
        Transaction::new(String::from("Alice"), String::from("Jake"), 25),
        Transaction::new(String::from("Jake"), String::from("Bob"), 7),
        Transaction::new(String::from("Eric"), String::from("Bob"), 82),
    ];

    let transactions = Transactions::new(vec_transactions);

    let mut merkle_tree = MerkleTree::new(transactions);

    for node in merkle_tree.leaves() {
        println!("{:?}", node);
    }

    merkle_tree.build_tree();
}
