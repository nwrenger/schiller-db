<script lang="ts">
	type T = $$Generic<toString>;

	export var fetchItems: (params: string, role: string | null, date: string | null) => Promise<T[]>;
	export var onSelect: (entry: T | null) => void;
	export var back: () => Promise<void>;
	export let params: string;
	export let role: string | null;
	export let date: string | null;
	export let nested: boolean = false;
	export let currentEntry: T | null;

	export function reload() {
		items = fetchItems(params, role, date);
	}

	function isObject(obj: any): obj is { ty: any; account: string } {
		return obj && typeof obj.account === 'string';
	}

	function isWorkless(obj: any): obj is { ty: 'workless'; currently: any } {
		return obj && typeof obj.currently === 'boolean';
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

	export function select(item: T | null) {
		if (item) {
			active = item;
		}
	}

	async function selectItem(list: T[] | undefined, ident: string | null) {
		if (list && ident) {
			active = list.find(entry => isObject(entry) && entry.account === ident) || null;
			if (active == null) {
				console.log('Cannot find entry: ', active, 'at: ', ident);
				await back();
			}
		}
	}

	$: selectItem(list, currentEntry && isObject(currentEntry) ? currentEntry.account : null);
	$: if (items instanceof Promise) items.then((val) => (list = val));

	let active: T | null;
	let list: T[];
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
			}}>{entry.account}{isWorkless(entry) && entry.currently ? ' - Arbeitslos' : ''}</button
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
