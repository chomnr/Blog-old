'use client';

import { usePathname, useRouter, useSearchParams } from 'next/navigation';
import Navbar from '@/components/navbar';
import { useEffect, useState } from 'react';

export default function Page() {
  const router = useRouter();
  let blog_id = usePathname().replace(/^\D+/g, '');
  const [data, setData] = useState(null)
  const [isLoading, setLoading] = useState(false)

  useEffect(() => {
    setLoading(true)
    fetch('http://localhost:8000/api/post/entries/' + blog_id)
      .then((res) => res.json())
      .then((data) => {
        setData(data)
        setLoading(false)
      })
  }, [])

    if (isLoading) return <p className='text-blue-500'>Loading...</p>
    if (!data) return <p className='text-red-500'></p>
  
  return (
    <>
    <Navbar/>
    <div className='flex flex-col justify-center items-center bg-white h-fit pb-8 text-black pt-5 max-w-[700px]'>
        <span className='font-bold text-md'>{data["author"]}</span>
        <h1 className='font-bold text-4xl'>{data["title"]}</h1>
        <div dangerouslySetInnerHTML={{ __html: atob(data["content"])} } />
    </div>
    </>
  );
}

/*
<div className='text-red-500'>{atob(data["content"])}</div>
*/