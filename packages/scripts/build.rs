use ethers::contract::Abigen;

fn main() {
    Abigen::new(
        "TokenRegistry",
        "../contracts/artifacts/TokenRegistry.sol/TokenRegistry.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/token_registry.rs")
    .unwrap();
}
