import Image from 'next/image'
import { Metadata } from 'next'
import { useState } from 'react';

export const metadata: Metadata = {
    title: 'Create Post',
    description: 'Create a post on the blog',
  }
  
export default function Page() {
  
  return (
    <>
    <div className='text-7xl mb-2 text-center'>Create a thread</div>
    <form className="flex flex-col bg-[#572536] space-y-3 p-3 rounded">
        <input className='p-1 bg-[#421525]' placeholder='Title'></input>
        <textarea className='p-1 bg-[#421525]' placeholder='INPUT TEXT'></textarea>
        <button>Post</button>
    </form>
    </>
  )
}