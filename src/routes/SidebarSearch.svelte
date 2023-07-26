<script lang="ts">
	import type { Writable } from 'svelte/store';

	export let params: string;
	export let role: string | null;
	export let date: string | null;
	export let nested: boolean;
	export let sidebarState: Writable<string | null>;
	export let accessUser: string | null = null;
	export let accessWorkless: string | null = null;
	export let accessCriminal: string | null = null;

	export var back: () => Promise<void>;
	export var fetchRoleSelectItems: (params: string, date: string | null) => Promise<[]>;

	var roleSelect: Promise<any[]> | never[] = [];
</script>

<div class="sidebar-search input-group pb-1 px-1">
	<button
		id="advanced"
		class="btn btn-outline-secondary dropdown-toggle hide-arrow"
		type="button"
		aria-expanded="false"
		data-bs-toggle="dropdown"
		data-bs-auto-close="outside"
		title="Nach Parametern Suchen"
		on:click={() => (roleSelect = fetchRoleSelectItems(params, date))}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="16"
			height="16"
			fill="currentColor"
			class="bi bi-gear"
			viewBox="0 0 16 16"
		>
			<path
				d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492zM5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0z"
			/>
			<path
				d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52l-.094-.319zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115l.094-.319z"
			/>
		</svg>
	</button>
	<ul class="dropdown-menu" id="group-select-dropdown">
		<li>
			<h6 class="dropdown-header">Gruppe</h6>
		</li>
		<form class="px-3 py-1" action="javascript:handleAdvanced()">
			<div class="mb-2">
				<select
					id="group-select"
					class="form-select"
					aria-label="Group Select"
					bind:value={role}
					on:click={async () => {
						await back();
						nested = false;
					}}
				>
					{#await roleSelect}
						<li class="list-group-item">
							<div class="d-flex justify-content-center">
								<div class="spinner-grow" role="status">
									<span class="visually-hidden">Loading...</span>
								</div>
							</div>
						</li>
					{:then data}
						<option value={null}>Alle</option>
						{#each data as entry}
							<option value={entry} selected={entry === role ? true : false}>{entry}</option>
						{/each}
					{/await}
				</select>
			</div>
		</form>
	</ul>
	<input
		type="text"
		class="form-control"
		placeholder="Suche"
		id="search"
		bind:value={params}
		on:click={async () => {
			await back();
			nested = false;
		}}
	/>
	<button
		id="select-button"
		class="btn btn-outline-secondary dropdown-toggle"
		type="button"
		title="Kategorie"
		data-bs-toggle="dropdown"
		aria-expanded="false"
		>{$sidebarState === 'user' ? 'Bürger' : ''}{$sidebarState === 'workless'
			? 'Arbeitslosenreg.'
			: ''}{$sidebarState === 'criminal' ? 'Kriminalregister' : ''}</button
	>
	<ul class="dropdown-menu dropdown-menu-end">
		<li>
			<h6 class="dropdown-header">Kategorie</h6>
		</li>
		<li>
			<button
				id="user"
				class={$sidebarState === 'user' ? 'dropdown-item active' : 'dropdown-item'}
				type="button"
				disabled={accessUser === 'None' ? true : false}
				on:click={() => {
					sidebarState.set('user');
				}}>Bürger</button
			>
		</li>
		<li>
			<button
				id="workless"
				class={$sidebarState === 'workless' ? 'dropdown-item active' : 'dropdown-item'}
				type="button"
				disabled={accessWorkless === 'None' ? true : false}
				on:click={() => {
					sidebarState.set('workless');
				}}>Arbeitslosenreg.</button
			>
		</li>
		<li>
			<button
				id="criminal"
				class={$sidebarState === 'criminal' ? 'dropdown-item active' : 'dropdown-item'}
				type="button"
				disabled={accessCriminal === 'None' ? true : false}
				on:click={() => {
					sidebarState.set('criminal');
				}}>Kriminalregister</button
			>
		</li>
	</ul>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}

	.sidebar-search {
		flex: 0;
	}
</style>
