import { configureChains, createConfig } from 'wagmi'
import { mainnet, arbitrum, sepolia, foundry } from 'wagmi/chains'
import { CoinbaseWalletConnector } from 'wagmi/connectors/coinbaseWallet'
import { InjectedConnector } from 'wagmi/connectors/injected'
import { MetaMaskConnector } from 'wagmi/connectors/metaMask'
import { publicProvider, } from 'wagmi/providers/public'
import { alchemyProvider, } from 'wagmi/providers/alchemy'
import { jsonRpcProvider, } from 'wagmi/providers/jsonRpc'

const { chains, publicClient, webSocketPublicClient } = configureChains(
  [mainnet, arbitrum, ...(process.env.NODE_ENV === 'development' ? [sepolia, foundry] : [])],
  [
    alchemyProvider({ apiKey: process.env.NEXT_PUBLIC_ALCHEMY_API_KEY || "" }), publicProvider(),
  ],
)

export const config = createConfig({
  autoConnect: true,
  connectors: [
    new MetaMaskConnector({ chains }),
    new CoinbaseWalletConnector({
      chains,
      options: {
        appName: 'wagmi',
      },
    }),
    new InjectedConnector({
      chains,
      options: {
        name: 'Injected',
        shimDisconnect: true,
      },
    }),
  ],
  publicClient,
  webSocketPublicClient,
})
