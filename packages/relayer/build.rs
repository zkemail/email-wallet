use ethers::contract::Abigen;
// use std::path::Path;

fn main() {
    // if !Path::new("../contracts/artifacts").exists() {
    //     panic!("Please run `forge build` in `../contracts` first");
    // }
    // let _ = std::process::Command::new("forge")
    //     .arg("build")
    //     .arg("--root")
    //     .arg("../contracts")
    //     .status();

    Abigen::new(
        "EmailWalletCore",
        "../contracts/artifacts/EmailWalletCore.sol/EmailWalletCore.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/email_wallet_core.rs")
    .unwrap();
    Abigen::new("ERC20", "../contracts/artifacts/ERC20.sol/ERC20.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/erc20.rs")
        .unwrap();
    Abigen::new(
        "TokenRegistry",
        "../contracts/artifacts/TokenRegistry.sol/TokenRegistry.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/token_registry.rs")
    .unwrap();
    Abigen::new(
        "AccountHandler",
        "../contracts/artifacts/AccountHandler.sol/AccountHandler.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/account_handler.rs")
    .unwrap();
    Abigen::new(
        "ExtensionHandler",
        "../contracts/artifacts/ExtensionHandler.sol/ExtensionHandler.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/extension_handler.rs")
    .unwrap();
    Abigen::new(
        "RelayerHandler",
        "../contracts/artifacts/RelayerHandler.sol/RelayerHandler.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/relayer_handler.rs")
    .unwrap();
    Abigen::new(
        "UnclaimsHandler",
        "../contracts/artifacts/UnclaimsHandler.sol/UnclaimsHandler.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/unclaims_handler.rs")
    .unwrap();
    Abigen::new(
        "EmailWalletEvents",
        "../contracts/artifacts/Events.sol/EmailWalletEvents.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/events.rs")
    .unwrap();
    Abigen::new(
        "ECDSAOwnedDKIMRegistry",
        "../contracts/artifacts/ECDSAOwnedDKIMRegistry.sol/ECDSAOwnedDKIMRegistry.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/ecdsa_owned_dkim_registry.rs")
    .unwrap();
    Abigen::new(
        "NFTExtension",
        "../contracts/artifacts/NFTExtension.sol/NFTExtension.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/nft_extension.rs")
    .unwrap();

    Abigen::new(
        "TestERC20",
        "../contracts/artifacts/TestERC20.sol/TestERC20.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("./src/abis/test_erc20.rs")
    .unwrap();

    Abigen::new("ERC721", "../contracts/artifacts/ERC721.sol/ERC721.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/abis/erc_721.rs")
        .unwrap();
}
