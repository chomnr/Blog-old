<script>
	import { page } from '$app/stores';
</script>
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

