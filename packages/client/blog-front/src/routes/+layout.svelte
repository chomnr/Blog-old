<script>
	import Header from './Header.svelte';
	import "../style.css";
</script>

<div class="app">
	<Header />

	<main>
		<slot />
	</main>

</div>

