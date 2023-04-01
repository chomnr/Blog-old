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

  
  
  return (
    <>
    <div className='text-7xl mb-2 text-center'>Register</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded" onSubmit={handleSubmit}>
        <input className='p-1 bg-[#421525]' type='username' value={formData.username} onChange={(e) => setFormData({ ...formData, username: e.target.value })}></input>
        <input className='p-1 bg-[#421525]' type='email' value={formData.email} onChange={(e) => setFormData({ ...formData, email: e.target.value })}></input>
        <input className='p-1 bg-[#421525]' type='password' value={formData.password} onChange={(e) => setFormData({ ...formData, password: e.target.value })}></input>
        <button>Register</button>
    </form>
    </>
  )
}

/*
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