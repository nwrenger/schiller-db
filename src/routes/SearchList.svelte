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

	function isObject(obj: any): obj is { account: string } {
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
	function sortby(a: T, b: T) {
		let accountA = a;
		let accountB = b;
		if (isObject(a) && isObject(b)) {
			accountA = a.account.toLowerCase() as unknown as T;
			accountB = b.account.toLowerCase() as unknown as T;
		}
		if (accountA < accountB) {
			return -1;
		} else if (accountA > accountB) {
			return 1;
		} else {
			return 0;
		}
	}

	export async function onUpdate(newItem: T | null, isNew: boolean) {
		// console.log('On Update', newItem);
		if (role || date) {
			await back();
			reload();
			return;
		}
		if (newItem) {
			if (isNew) {
				if (isObject(newItem) && (newItem.account.includes(params) || newItem.forename.includes(params) || newItem.surname.includes(params))) {
					list.push(newItem);
					list.sort(sortby);
					activeIndex = list.findIndex((entry) => entry === newItem);
				} else {
					await back();
					return;
				}
			} else {
				list[activeIndex] = newItem;
			}
			active = newItem;
		} else {
			if (newItem === null) {
				if (activeIndex > -1) {
					list.splice(activeIndex, 1);
				}
			}
		}
		items = list as unknown as Promise<T[]>;
	}

	$: if (items instanceof Promise) items.then((val) => (list = val));

	let active: T | null;
	let activeIndex: number;
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
	{#each data as entry, i}
		<button
			class="list-group-item list-group-item-action"
			class:active={active === entry}
			on:click={() => {
				list = data;
				active = entry;
				activeIndex = i;
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
