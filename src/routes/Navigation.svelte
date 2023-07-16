<script lang="ts">
	import type { Writable } from 'svelte/store';
    import { page } from '$app/stores';
	import NavigationProfile from './NavigationProfile.svelte';

	export let currentUser: string | null = null;
	export let onSelect: (val: string) => void;
</script>

<nav class="nav navbar bg-secondary-subtle">
	<div class="container-fluid">
		<a href={$page.url.pathname} class="navbar-brand">SNDI</a>
		{#if currentUser != null}
            <NavigationProfile onSelect={onSelect} currentUser={currentUser} />
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
