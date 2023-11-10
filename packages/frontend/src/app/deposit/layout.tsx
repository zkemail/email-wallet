export default function RootLayout({
    children,
}: {
    children: React.ReactNode
}) {
    return (
        <div>
            <h2>Deposit your tokens to an email address</h2>
            <section>{children}</section>
        </div>
    )
}