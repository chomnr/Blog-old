import './globals.css'

export const metadata = {
  title: 'Home',
  description: 'Simple blog app written in react + rust.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className=''>
          {children}
      </body>
    </html>
  )
}
