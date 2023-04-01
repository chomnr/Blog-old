'use client';

import { Metadata } from 'next'
import { useState } from 'react';
import { useRouter } from 'next/router';
import Navbar from "@/components/navbar"
import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import cookies from 'next-cookies'
import React from 'react';

function Page({ }) {
  let url = "http://localhost:8000/api/post/create"
  const [formData, setFormData] = useState({
    title: '',
    content: '',
  })

  const [error, setErrorMessage] = useState({
    message: ''
  })

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    let res = await fetch(url, {
      method: "POST",
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    });
    let data = await res.json();
    if (res.status === 200) {
      console.log("Success: " + data["message"])
      setErrorMessage({ message: data["message"] });
    } else {
      console.log("Failed: " + data["message"])
      setErrorMessage({ message: data["message"] });
    }
  };

  return (
    <>
      <Navbar />
      <div className='flex flex-col items-center bg-white h-fit pb-8 pt-8 text-black'>
        <form className='flex flex-col space-y-3 items-center' onSubmit={handleSubmit}>
          <input className='p-3 w-[700px] text-[#000]' type='username' placeholder='Title' value={formData.title} onChange={(e) => setFormData({ ...formData, title: e.target.value })} required></input>
          <ReactQuill className='w-[700px] max-w-[700px] word-break' theme="snow" value={formData.content} onChange={(e) => setFormData({ ...formData, content: e.toString() })} />
          <div className="text-red-500 mt-2 text-sm">
              {error.message}
          </div>
          <button className='bg-[#4E192B] text-white p-1'>Create Post</button>
        </form>
      </div>
    </>
  )
}




export default Page
/*

class Page extends React.Component {
  constructor(props: any) {
    super(props);
    this.state = {
      title: '',
      content: ''
    };
  }

  setTitle(value: string) {
    this.setState({ title: value })
  }

  setContent(value: any) {
    this.setState({ content: value })
  }

  render() {
    return (
      <>
        <Navbar />
        <div className='flex flex-col items-center bg-white h-fit pb-8 pt-8 text-black'>
          <form className='flex flex-col space-y-3 items-center'>
            <input className='p-3 w-[700px] text-[#000]' type='username' placeholder='Title' onChange={(e) => this.setTitle(e.target.value)} />
            <ReactQuill
            className='w-[700px] max-w-[700px] word-break'
            theme="snow"
            onChange={(e) => this.setTitle(e.valueOf.to)}
          />
            <button className='bg-[#4E192B] text-white p-1'>Create Post</button>
          </form>
        </div>
      </>
    )
  }
};
*/

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