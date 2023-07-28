<script lang="ts">
	type T = $$Generic<ToString>;
	interface ToString {
		toString(): string;
	}

	export let state: string | null;
	export let currentEntry: T | null;

	export var fetchItems: (parents: T[]) => Promise<T[]>;
	export var onSelect: (parents: T[]) => boolean;
	export var back: () => Promise<void>;

	export function reload() {
		items = fetchItems(parents);
	}

	export function reset() {
		parents.pop();
		items = fetchItems(parents);
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

	function isObject(obj: any): obj is { ty: any; account: string } {
		return obj && typeof obj.account === 'string';
	}

	function isCriminal(obj: any): obj is { ty: 'criminal'; account: any; kind: any } {
		return obj && typeof obj.kind === 'string';
	}

	function isWorkless(obj: any): obj is { ty: 'workless'; currently: any; date_of_dismiss: any } {
		return obj && typeof obj.currently === 'boolean';
	}

	function formatDate(date: string) {
		const [year, month, day] = date.split('-');
		return `${day}.${month}.${year}`;
	}

	async function selectItem(list: T[] | undefined, ident: string | null) {
		if (list && ident) {
			active = list.find((entry) => isObject(entry) && entry.account === ident) || null;
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
	let items: Promise<T[]> = fetchItems([]);
	let parents: T[] = [];
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
	{#if parents.length > 0}
		{data.length === 0 ? reset() : ''}
		<button
			class="list-group-item list-group-item-action list-group-item-danger"
			on:click={async () => {
				await back();
				reset();
			}}
			>Zurück - {state === 'workless'
				? formatDate(parents.join(' - '))
				: parents.join(' - ')}</button
		>
	{/if}
	{#each data as entry}
		<button
			class="list-group-item list-group-item-action"
			class:active={active === entry}
			id={isObject(entry) ? entry.account : entry.toString()}
			on:click={() => {
				active = entry;
				if (onSelect([...parents, active])) {
					parents.push(active);
					items = fetchItems(parents);
					active = null;
				}
			}}
			>{isObject(entry) && parents.length > 0
				? isCriminal(entry)
					? entry.kind
					: entry.account
				: state === 'workless'
				? formatDate(entry.toString())
				: entry.toString()}{isWorkless(entry) && entry.currently ? ' - Arbeitslos' : ''}</button
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
