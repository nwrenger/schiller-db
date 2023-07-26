<script lang="ts">
	import { page } from '$app/stores';
	import NavigationProfile from './NavigationProfile.svelte';

	export let currentUser: string | null = null;
	export let accessUser: string | null = null;
	export let onSelect: ((val: string) => void) | null;
</script>

<nav class="nav navbar bg-secondary-subtle">
	<div class="container-fluid">
		<a href={$page.url.pathname} data-sveltekit-reload class="navbar-brand">SchillerDB</a>
		{#if currentUser != null}
			<NavigationProfile {onSelect} {currentUser} {accessUser} />
		{/if}
	</div>
</nav>

<style>
	.nav {
		grid-area: nav;
	}
	.navbar-brand {
		font-size: x-large;
	}
</style>
