import { defineConfig } from '@wagmi/cli'
import { foundry, react } from '@wagmi/cli/plugins'
import * as chains from 'wagmi/chains'

export default defineConfig({
    out: 'src/abis.ts',
    plugins: [
        foundry({
            project: "../contracts",
            include: [
                "*Handler.sol/**",
                "EmailWalletCore.sol/**",
                "TokenRegistry.sol/**",
                "ERC20.sol/**",
                "WETH9.sol/**",
            ],
            deployments: {
                EmailWalletCore: {
                    [chains.sepolia.id]: "0xDBEf046EA0f17aC352212bA620F5B6Abc7F5017b"
                }
            }
        }),
        react()
    ],
})
