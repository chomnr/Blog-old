export default function Navbar() {
    return (
    <div className="flex lg:flex-row flex-col justify-center items-center w-screen h-40 space-x-5 bg-[#3B1320]">
        <h1 id="brand" className="text-7xl lg:pl-11 pb-3 lg:pt-3 pt-6"><a href="/">BLOG</a></h1>
        <nav className="flex flex-row lg:flex-shrink-0 flex-grow space-x-3 text-sm">
          <a href="post/entries" className="hover:text-[#A29E9D] cursor-pointer">ENTRIES</a>
        </nav>
    </div>
    )
  }