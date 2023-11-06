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
        }),
        react()
    ],
})
