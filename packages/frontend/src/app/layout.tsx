import { Providers } from './providers'

export const metadata = {
  title: 'wagmi',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <h1>Email Wallet</h1>
          {children}
        </Providers>
      </body>
    </html>
  )
}
