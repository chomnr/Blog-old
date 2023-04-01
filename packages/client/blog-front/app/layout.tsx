import './globals.css'
import Navbar from './components/navbar'

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
        <Navbar/>
        <div className="flex flex-col justify-center items-center space-y-3">
          {children}
        </div>
      </body>
    </html>
  )
}
