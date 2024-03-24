extern crate dotenv;
extern crate ethers;
extern crate reqwest;
extern crate serde;
extern crate tokio;

pub mod abis;

mod token_registry;

#[tokio::main]
async fn main() {
    // Parse args
    let args: Vec<String> = std::env::args().collect();

    println!("args: {:?}", args);

    match args[1].as_str() {
        "populate-token-registry" => {
            token_registry::run().await;
        }
        _ => {
            println!("Unknown command");
        }
    }
}
