'use client';

import { Metadata } from 'next'
import { useState } from 'react';
import { useRouter } from 'next/router';

export default function Page() {

  let url = "http://localhost:8000/api/user/login"
  const [formData, setFormData] = useState({
    login: '',
    password: '',
  })

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    let res = await fetch(url, {
      method: "POST",
      credentials: 'include',
      redirect: 'follow',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    });
    let data = await res.json();
    if (res.status === 200) {
      console.log("Success: " + data["message"])
      //window.location.href = "/";
    } else {
      console.log("Failed: " + data["message"])
    }
  };

  return (
    <>
    <div className='flex h-screen'>
      <div className='lg:flex md:hidden hidden bg-[#4E192B] w-3/6'>
          <div className='flex flex-row font-semibold m-3 text-2xl space-x-2'>
            <span>BLOG</span>
            <span className='text-sm'>"Write something great."</span>
          </div>
      </div>
      <div className='flex justify-center items-center bg-[#fff] w-screen text-black'>
          <div className='flex flex-col space-y-5 min-w-[400px] min-h-[400px] '>
            <h2 className='text-2xl'>Login</h2>
            <form className="flex flex-col space-y-5 rounded w-[300px]" onSubmit={handleSubmit}>
              <div className='flex flex-col space-y-2'>
                  <label className='font-bold text-sm'>Username or Email</label>
                  <input className='border-2 p-1.5 text-sm' placeholder='Username or Email' value={formData.login} onChange={(e) => setFormData({ ...formData, login: e.target.value })}></input>
              </div>
              <div className='flex flex-col space-y-2'>
                  <label className='font-bold text-sm'>Password</label>
                  <input className='border-2 p-1.5 text-sm' type='password' placeholder='Password' value={formData.password} onChange={(e) => setFormData({ ...formData, password: e.target.value })}></input>
              </div>
              <div className='flex flex-col space-y-2'>
                  <button className='bg-[#4E192B] rounded text-white p-2 hover:bg-[#603040]' placeholder='Password'>Login</button>
                  <span className='text-sm'>Don't have an account? <a href="/auth/register" className='text-[#234E52] hover:text-[#A29E9D]'>Register</a></span>
              </div>
            </form>
          </div>
      </div>
    </div>
    </>
  )
}
