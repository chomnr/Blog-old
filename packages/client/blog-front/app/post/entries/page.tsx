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