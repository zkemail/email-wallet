import { defineConfig } from '@wagmi/cli'
import { foundry } from '@wagmi/cli/plugins'

export default defineConfig({
    out: 'src/generated.ts', contracts: [], plugins: [
        foundry({
            project: '../contracts',
            include: [
                'EmailWalletCore.sol/**',
                `Wallet.sol/**`,
                `IOauth.sol/**`,
            ],
            exclude: [
                "*.json"
            ]
        }),
    ]
})
