<script>
	import { page } from '$app/stores';
</script>

<header class="flex flex-col items-center bg-[#001332] border-b-2 border-[#001B48] text-[#dde8f0] space-y-2 pt-2 pb-2">
	<div class="text-3xl bg-[#1A325A] text-center rounded p-1">
		<span class="text-[#97cbdc] font-[300] tracking-widest">BLOG</span>
		<div class="float"></div>
	</div>
	<nav class="space-x-3 font-sans">
		<a href="#" class="transition ease-in-out delay-100 hover:-translate-y-1 hover:scale-110 hover:text-[#808DA4] duration-300 ">Home</a>
		<a href="#" class="transition ease-in-out delay-100 hover:-translate-y-1 hover:scale-110 hover:text-[#808DA4] duration-300 ">History</a>
		<a href="#" class="transition ease-in-out delay-100 hover:-translate-y-1 hover:scale-110 hover:text-[#808DA4] duration-300 ">Placeholder</a>
	</nav>
</header>
<!--
<header class="flex flex-col justify-center bg-[#F5F5F5] border-b-2 border-[#E6E6E6]">
	<nav class="flex justify-center space-x-1 p-1.5">
		<ul class="flex flex-row space-x-1">
			<li aria-current={$page.url.pathname === '/login' ? 'page' : undefined}>
				<a href="/login" class="bg-[#EBEBEB] p-1 rounded pl-2 pr-2">Login</a>
			</li>
			<li aria-current={$page.url.pathname === '/home' ? 'page' : undefined}>
				<a href="/logout" class="bg-[#EBEBEB] p-1 rounded pl-2 pr-2">Logout</a>
			</li>
			<li aria-current={$page.url.pathname === '/register' ? 'page' : undefined}>
				<a href="/register" class="bg-[#EBEBEB] p-1 rounded pl-2 pr-2">Register</a>
			</li>
		</ul>
	</nav>
	<nav class="flex justify-center space-x-1 p-1.5">
		<ul class="flex flex-row space-x-1">
			<li aria-current={$page.url.pathname === '/create_post' ? 'page' : undefined}>
				<a href="create_post" class="bg-[#EBEBEB] p-1 rounded pl-2 pr-2">Create Post</a>
			</li>
			<li aria-current={$page.url.pathname === '/login' ? 'page' : undefined}>
				<a href="#" class="bg-[#EBEBEB] p-1 rounded pl-2 pr-2">View Posts</a>
			</li>
		</ul>
	</nav>
</header>
-->
<!--
<header>
	<nav>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
		</svg>
		<ul>
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
			<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
				<a href="/about">About</a>
			</li>
			<li aria-current={$page.url.pathname.startsWith('/sverdle') ? 'page' : undefined}>
				<a href="/sverdle">Sverdle</a>
			</li>
		</ul>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
		</svg>
	</nav>
</header>
-->

