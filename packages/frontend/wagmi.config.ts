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
                    [chains.sepolia.id]: "0x8B452C4a15BF90B0491793A0B278ABD589b23c57"
                }
            }
        }),
        react()
    ],
})
