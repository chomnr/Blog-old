import Image from 'next/image'
import { Metadata } from 'next'

export const metadata: Metadata = {
    title: 'Login',
    description: 'Login to your account',
  }
  
export default function Page() {
  return (
    <>
    <div className='text-7xl mb-2 text-center'>Login</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded">
        <input className='p-1 bg-[#421525]' placeholder='Email or Username'></input>
        <input className='p-1 bg-[#421525]' type='password' placeholder='Password'></input>
        <button>Login</button>
    </form>
    </>
  )
}