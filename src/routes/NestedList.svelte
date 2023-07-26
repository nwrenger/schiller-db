<script lang="ts">
	type T = $$Generic<ToString>;
	interface ToString {
		toString(): string;
	}

	export var state: string | null;

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

	function isObject(obj: any): obj is { account: string } {
		return obj && typeof obj.account === 'string';
	}

	function isCriminal(obj: any): obj is { ty: 'criminal'; kind: any } {
		return obj && typeof obj.kind === 'string';
	}

	function isWorkless(obj: any): obj is { ty: 'workless'; currently: any } {
		return obj && typeof obj.currently === 'boolean';
	}

	function formatDate(date: string) {
		const [year, month, day] = date.split('-');
		return `${day}.${month}.${year}`;
	}

	let active: T | null;
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
			on:click={() => {
				active = entry;

				if (onSelect([...parents, active])) {
					parents.push(active);
					items = fetchItems(parents);
					active = null;
				}
			}}
			>{isObject(entry)
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
