import Image from 'next/image'
import { Metadata } from 'next'

export const metadata: Metadata = {
    title: 'Register',
    description: 'Register your account',
  }
  
export default function Page() {
  return (
    <>
    <div className='text-7xl mb-2 text-center'>Register</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded">
        <input className='p-1 bg-[#421525]' type='username' placeholder='Username'></input>
        <input className='p-1 bg-[#421525]' type='email' placeholder='Email'></input>
        <input className='p-1 bg-[#421525]' type='password' placeholder='Password'></input>
        <button>Register</button>
    </form>
    </>
  )
}