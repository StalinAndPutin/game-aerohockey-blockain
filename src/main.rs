use blockchain::Blockchain;
use std::io;

#[path = "./blockchain/block.rs"]
pub mod block;
#[path = "./blockchain/blockchain.rs"]
pub mod blockchain;

//Start work blockchain
fn main() {
    let mut blockchain = Blockchain::new(4);

    loop {
        println!("\nChoose an option:");
        println!("1. Add Block");
        println!("2. View Blockchain");
        println!("3. Validate Blockchain");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter data for the block:");
                let mut data = String::new();
                io::stdin().read_line(&mut data).unwrap();
                blockchain.add_block(data.trim().to_string());
                print!("Block added!")
            }
            "2" => {
                for block in &blockchain.chain {
                    println!("{:#?}", block);
                }
            }
            "3" => {
                if blockchain.is_valid() {
                    println!("Blockchain is valid.");
                } else {
                    println!("Blockchain is invalid.");
                }
            }
            "4" => {
                println!("Exiting... Bye, blockchain buddy!");
                break;
            }
            _ => {
                println!(" fuck you ")
            }
        }
    }
}
