import { Deployer } from '@matterlabs/hardhat-zksync-deploy';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';
import { Wallet } from 'zksync-ethers';

export default async function (
    hre: HardhatRuntimeEnvironment,
) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const privateKey = process.env.PRIVATE_KEY!;
    const wallet = new Wallet(privateKey);
    const deployer = new Deployer(hre, wallet);

    const addArtifact = await deployer.loadArtifact('EcAdd');
    const add = await deployer.deploy(addArtifact, [], undefined, []);
    const addAddress = await add.getAddress();
    console.log(`ec add address: ${addAddress}`);

    const mulArtifact = await deployer.loadArtifact('EcMul');
    const mul = await deployer.deploy(mulArtifact, [], undefined, []);
    const mulAddress = await mul.getAddress();
    console.log(`ec mul address: ${mulAddress}`);

    const pairingArtifact = await deployer.loadArtifact('EcPairing');
    const pairing = await deployer.deploy(pairingArtifact, [], undefined, []);
    const pairingAddress = await pairing.getAddress();
    console.log(`ec pairing address: ${pairingAddress}`);
}