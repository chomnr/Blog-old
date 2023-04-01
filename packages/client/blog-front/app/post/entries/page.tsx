'use client';

import { Metadata } from 'next'
import { useState } from 'react';
import { useRouter } from 'next/router';
import Navbar  from "@/components/navbar"


export default function Page() {
  return (
    <>
    <Navbar/>
    <div className='flex flex-col items-center bg-white h-fit pb-8 '>
      sadsddassd
      <div id="blog-post-1" className='flex flex-col min-w-[500px] mt-3 pl-3 border-l-2 border-t-2 border-[#4E192B] text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Cracking the theory of mind. p1
            </div>
            April 1st, 2023
        </div>
      </div>

      <div id="blog-post-1" className='flex flex-col min-w-[500px] mt-3 pl-3 text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Cracking the theory of mind. p1
            </div>
            April 1st, 2023
        </div>
      </div>
      <div id="blog-post-1" className='flex flex-col min-w-[500px] mt-3 pl-3 text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Cracking the theory of mind. p1
            </div>
            April 1st, 2023
        </div>
      </div>
      <div id="blog-post-1" className='flex flex-col min-w-[500px] mt-3 pl-3 text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Cracking the theory of mind. p1
            </div>
            April 1st, 2023
        </div>
      </div>
      <div id="blog-post-1" className='flex flex-col min-w-[500px] mt-3 pl-3 text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Cracking the theory of mind. p1
            </div>
            April 1st, 2023
        </div>
      </div>
      

      <div className='blog-post-2" flex flex-col min-w-[500px] mt-3 pl-3 border-r-2 border-b-2 border-[#4E192B] text-black hover:bg-[#F2F2F2] cursor-pointer'>
        <div className='flex flex-col p-1'>
            Zeljko
            <div className='flex flex-row font-bold'>
              Solutions of the universe...
            </div>
            April 1st, 2023
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