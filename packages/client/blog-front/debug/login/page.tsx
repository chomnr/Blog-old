'use client';

import Image from 'next/image'
import { Metadata } from 'next'

export const metadata: Metadata = {
    title: 'Login',
    description: 'Login to your account',
  }
  
export default function Page() {
  let login = "Doggy"
  let password = "Doggy123!"
    let handleSubmit = async (e: { preventDefault: () => void }) => {
    e.preventDefault();
    try {
      let res = await fetch("http://localhost:8000/api/user/login", {
        method: "POST",
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
          "Access-Control-Allow-Headers": "Origin, X-Requested-With, Content-Type, Accept",
          'Origin': 'http://localhost:3000/',
        },
        body: JSON.stringify({
          login: login,
          password: password
        }),
      });
      let resJson = await res.json();
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <>
    <div className='text-7xl mb-2 text-center'>Login</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded" onSubmit={handleSubmit}>
        <input className='p-1 bg-[#421525]' placeholder='Email or Username'></input>
        <input className='p-1 bg-[#421525]' type='password' placeholder='Password'></input>
        <button>Login</button>
    </form>
    </>
  )
}