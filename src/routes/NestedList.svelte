<script lang="ts">
	type T = $$Generic<ToString>;
	interface ToString {
		toString(): string;
	}

	export let state: string | null;

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

	function isUser(obj: any): obj is { ty: 'user'; role: any } {
		return obj && typeof obj.role === 'string';
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
		if (parents.length <= 0) {
			await back();
			reload();
			return;
		}
		if (!isNew) {
			if (
				(isUser(newItem) && isUser(active) && newItem.role != active.role) ||
				(isWorkless(newItem) &&
					isWorkless(active) &&
					newItem.date_of_dismiss != active.date_of_dismiss) ||
				(isCriminal(newItem) && isCriminal(active) && newItem.account != active.account)
			) {
				await back();
				if (activeIndex > -1) {
					list.splice(activeIndex, 1);
				}
				items = list as unknown as Promise<T[]>;
				return;
			}
		}
		if (newItem) {
			if (isNew) {
				if (
					(isUser(newItem) && isUser(active) && newItem.role != active.role) ||
					(isWorkless(newItem) &&
						isWorkless(active) &&
						newItem.date_of_dismiss != active.date_of_dismiss) ||
					(isCriminal(newItem) && isCriminal(active) && newItem.account != active.account)
				) {
					await back();
					return;
				} else {
					list.push(newItem);
					list.sort(sortby);
					activeIndex = list.findIndex((entry) => entry === newItem);
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
	{#each data as entry, i}
		<button
			class="list-group-item list-group-item-action"
			class:active={active === entry}
			on:click={() => {
				list = data;
				active = entry;
				activeIndex = i;
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
