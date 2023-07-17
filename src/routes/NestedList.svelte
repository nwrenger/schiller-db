<script lang="ts">
	type T = $$Generic<ToString>;
	interface ToString {
		toString(): string;
	}

	export var fetchItems: (parents: T[]) => Promise<T[]>;
	export var onSelect: (parents: T[]) => boolean;
	export var stats: () => void;

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

	let active: T | null;
	let items: Promise<T[]> = fetchItems([]);
	let parents: T[] = [];
</script>

<ul class="sidebar-list list-group list-group-flush" id="sidebar-list">
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
			<button
				class="list-group-item list-group-item-action list-group-item-danger"
				on:click={() => {
					reset();
					stats();
				}}>Zurück - {parents.join(' - ')}</button
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
				}}>{entry.toString()}</button
			>
		{:else}
			<li class="list-group-item">Keine Einträge!</li>
		{/each}
	{/await}
</ul>

<style>
	.sidebar-list {
		flex: 1;
		overflow-y: scroll;
	}

	.list-group-item-action {
		cursor: pointer;
	}
</style>
