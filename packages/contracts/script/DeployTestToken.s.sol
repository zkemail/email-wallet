// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "forge-std/Script.sol";

contract TestERC20 is ERC20 {
    uint maxPerMint;

    constructor(string memory name, string memory tick, uint _maxPerMint) ERC20(name, tick) {
        maxPerMint = _maxPerMint;
    }

    function freeMint(uint256 amount) public {
        freeMint(msg.sender, amount);
    }

    function freeMint(address to, uint256 amount) public {
        require(amount <= maxPerMint, "amount exceeds maxPerMint");
        _mint(to, amount);
    }
}

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }
        uint256 maxAmount = vm.envUint("MAX_AMOUNT");
        if (maxAmount == 0) {
            console.log("MAX_AMOUNT env var not set");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);
        TestERC20 testToken = new TestERC20("EmailWalletV1Test", "EWV1TEST", maxAmount);
        vm.stopBroadcast();

        console.log("TestERC20 deployed at: %s", address(testToken));
    }
}
