fn main() {
    println!("     --- Git Information ---");
    println!(
        "Current Branch: {}",
        alexandria::git::current_branch().unwrap()
    );
    println!(
        "Current Hash: {}",
        alexandria::git::current_commit_hash().unwrap()
    );
}
