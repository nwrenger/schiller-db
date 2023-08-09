<script lang="ts">
	import { page } from '$app/stores';
	import NavigationProfile from './NavigationProfile.svelte';
	import type { Permission } from '../../routes/+page.svelte';

	export let currentUser: string | null = null;
	export let permission: Permission | null;
	export let onSelect: ((val: string) => void) | null;
</script>

<nav class="nav navbar bg-secondary-subtle">
	<div class="container-fluid">
		<a href={$page.url.pathname} data-sveltekit-reload class="navbar-brand">SchillerDB</a>
		{#if currentUser != null}
			<NavigationProfile {onSelect} {currentUser} {permission} />
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
