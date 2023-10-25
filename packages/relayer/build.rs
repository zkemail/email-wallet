use ethers::contract::Abigen;

fn main() {
    Abigen::new("EmailWalletCore", "./src/abis/EmailWalletCore.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/email_wallet_core.rs")
        .unwrap();
    Abigen::new("ERC20", "./src/abis/ERC20.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/erc20.rs")
        .unwrap();
    Abigen::new("TokenRegistry", "./src/abis/TokenRegistry.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/token_registry.rs")
        .unwrap();
    Abigen::new("AccountHandler", "./src/abis/AccountHandler.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/account_handler.rs")
        .unwrap();
    Abigen::new("ExtensionHandler", "./src/abis/ExtensionHandler.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/extension_handler.rs")
        .unwrap();
    Abigen::new("RelayerHandler", "./src/abis/RelayerHandler.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/relayer_handler.rs")
        .unwrap();
    Abigen::new("UnclaimsHandler", "./src/abis/UnclaimsHandler.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/unclaims_handler.rs")
        .unwrap();
    Abigen::new("EmailWalletEvents", "./src/abis/EmailWalletEvents.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/events.rs")
        .unwrap();
}
