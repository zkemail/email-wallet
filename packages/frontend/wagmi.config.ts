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
                    [chains.sepolia.id]: "0x3259282dD19Cf44639e8b974B301805fB3342Fac"
                }
            }
        }),
        react()
    ],
})
