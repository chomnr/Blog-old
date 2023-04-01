'use client';

import { Metadata } from 'next'
import { useState } from 'react';
import { useRouter } from 'next/router';

export default function Page() {

  let url = "http://localhost:8000/api/user/create"
  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: ''
  })
  
  const [error, setError] = useState({
    message: ''
  })

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    let res = await fetch(url, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    });
    let data = await res.json();
    if (res.status === 200) {
      console.log("Success: " + data["message"])
      window.location.href = "/auth/login";
    } else {
      console.log("Failed: " + data["message"])
      error.message = data["message"]
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
            <h2 className='text-2xl'>Register</h2>
            <form className="flex flex-col space-y-5 rounded w-[300px]" onSubmit={handleSubmit}>
              <div className='flex flex-col space-y-2'>
                  <label className='font-bold text-sm'>Username</label>
                  <input className='border-2 p-1.5 text-sm' type='username' placeholder='Username' value={formData.username} onChange={(e) => setFormData({ ...formData, username: e.target.value })}></input>
              </div>
              <div className='flex flex-col space-y-2'>
                  <label className='font-bold text-sm'>Email</label>
                  <input className='border-2 p-1.5 text-sm' type='email' placeholder='Email' value={formData.email} onChange={(e) => setFormData({ ...formData, email: e.target.value })}></input>
              </div>
              <div className='flex flex-col space-y-2'>
                  <label className='font-bold text-sm'>Password</label>
                  <input className='border-2 p-1.5 text-sm' type='password' placeholder='Password' value={formData.password} onChange={(e) => setFormData({ ...formData, password: e.target.value })}></input>
              </div>
              <div className='flex flex-col space-y-2'>
                  <button className='bg-[#4E192B] rounded text-white p-2 hover:bg-[#603040]' placeholder='Password'>Register</button>
                  {error && (
                    <div className="text-red-500 mt-2 text-sm">
                        {error.message}
                    </div>
                  )}
                  <span className='text-sm'>Have an account already?<a href="/auth/login" className='text-[#234E52] hover:text-[#A29E9D]'> Login</a></span>
              </div>
            </form>
          </div>
      </div>
    </div>
    </>
  )
}

/*

let url = "http://localhost:8000/api/user/create"
  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: ''
  })

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    let res = await fetch(url, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    });
    let data = await res.json();
    if (res.status === 200) {
      console.log("Success: " + data["message"])
      window.location.href = "/";
    } else {
      console.log("Failed: " + data["message"])
    }
  };

<form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded w-[300px]" onSubmit={handleSubmit}>
        <div className='font-bold text-3xl'>User Register</div>
        <input className='p-3 bg-[#421525]' type='username' placeholder='Username' value={formData.username} onChange={(e) => setFormData({ ...formData, username: e.target.value })}></input>
        <input className='p-3 bg-[#421525]' type='email' placeholder='Email' value={formData.email} onChange={(e) => setFormData({ ...formData, email: e.target.value })}></input>
        <input className='p-3 bg-[#421525]' type='password' placeholder='Password' value={formData.password} onChange={(e) => setFormData({ ...formData, password: e.target.value })}></input>
        <button>Register</button>
      </form>


export const metadata: Metadata = {
    title: 'Register',
    description: 'Register your account',
}
  
export default function Page() {

  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: ''
  })
 
  };
    
  return (
    <>
    <div className='text-7xl mb-2 text-center'>Register</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded" onSubmit={handleSubmit}>
        <input className='p-1 bg-[#421525]' type='username' value={username} onChange={(e) => setUsername(e.target.value)}></input>
        <input className='p-1 bg-[#421525]' type='email' value={email} onChange={(e) => setEmail(e.target.value)}></input>
        <input className='p-1 bg-[#421525]' type='password' value={password} onChange={(e) => setPassword(e.target.value)}></input>
        <button>Register</button>
        <div className="message">{message ? <p>{message}</p> : null}</div>
    </form>
    </>
  )
}
*/

/*()
 const [username, setUsername] = useState("");
 const [email, setEmail] = useState("");
 const [password, setPassword] = useState("");
 const [message, setMessage] = useState("");
 const router = useRouter();

 let handleSubmit = async (e: { preventDefault: () => void }) => {
   e.preventDefault();
   try {
     let res = await fetch("http://localhost:8000/api/user/create", {
       method: "POST",
       headers: {
         'Content-Type': 'application/json',
       },
       body: JSON.stringify({
         username: username,
         email: email,
         password: password
       }),
     });
     let resJson = await res.json();
     router.push('/login');

     if (res.status === 200) {
       setUsername("");
       setEmail("");
       setPassword("");
       setMessage("User created successfully");
     } else {
       setMessage(resJson["message"]);
     }
   } catch (err) {
     console.log(err);
   }
   */