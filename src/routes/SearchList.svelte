<script lang="ts">
	type T = $$Generic<toString>;

	export var fetchItems: (params: string, role: string | null, date: string | null) => Promise<T[]>;
	export var onSelect: (entry: T | null) => void;
	export var back: () => Promise<void>;
	export var params: string;
	export var role: string | null;
	export var date: string | null;
	export var nested: boolean = false;

	export function reload() {
		items = fetchItems(params, role, date);
	}

	export function deselectAll() {
		active = null;
	}

	export function isSelected() {
		if (active) {
			return true;
		} else {
			return false;
		}
	}

	let active: T | null;
	let items: Promise<T[]> | never[] = [];
	$: items = fetchItems(params, role, date);
</script>

{#await items}
	<li class="list-group-item">
		<div class="d-flex justify-content-center">
			<div class="spinner-grow" role="status">
				<span class="visually-hidden">Loading...</span>
			</div>
		</div>
	</li>
{:then data}
	<button
		class="list-group-item list-group-item-action list-group-item-danger"
		on:click={async () => {
			await back();
			nested = true;
			params = '';
			role = null;
		}}>Zurück {params || !role ? `- "${params}"` : ''}{role ? ` - ${role}` : ''}</button
	>
	{#each data as entry}
		<button
			class="list-group-item list-group-item-action"
			class:active={active === entry}
			on:click={() => {
				active = entry;
				onSelect(active);
			}}>{entry.account}</button
		>
	{:else}
		<li class="list-group-item">Keine Einträge!</li>
	{/each}
{/await}

<style>
	.list-group-item-action {
		cursor: pointer;
	}
</style>
