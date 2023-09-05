<script lang="ts">
	type T = $$Generic<toString>;

	export let params: string;
	export let isNew: boolean;
	export let role: string | null;
	export let date: string | null;
	export let nested: boolean = false;
	export let currentEntry: T | null;
	export let onHighlighted: boolean;

	export var fetchItems: (params: string, role: string | null, date: string | null) => Promise<T[]>;
	export var onSelect: (entry: T | null) => void;
	export var back: () => Promise<void>;

	export function reload() {
		items = fetchItems(params, role, date);
	}

	function isObject(obj: any): obj is { ty: any; account: string } {
		return obj && typeof obj.account === "string";
	}

	function isUser(obj: any): obj is { ty: "user"; account: string; forename: string } {
		return obj && typeof obj.account === "string" && typeof obj.forename === "string";
	}

	function isWorkless(obj: any): obj is {
		ty: "workless";
		account: any;
		currently: any;
		date_of_dismiss: any;
		old_company: any;
	} {
		return obj && typeof obj.currently === "boolean";
	}

	function isCriminal(obj: any): obj is { ty: "criminal"; account: any; kind: any } {
		return obj && typeof obj.kind === "string";
	}

	export function deselectAll() {
		active = null;
	}

	async function selectItem(list: T[] | null) {
		if (list && currentEntry && currentEntry !== active) {
			active =
				list.find(
					(entry) =>
						(isUser(entry) && isUser(currentEntry) && entry.account === currentEntry.account) ||
						(isWorkless(entry) &&
							isWorkless(currentEntry) &&
							entry.account === currentEntry.account &&
							entry.date_of_dismiss === currentEntry.date_of_dismiss &&
							entry.old_company === currentEntry.old_company) ||
						(isCriminal(entry) &&
							isCriminal(currentEntry) &&
							entry.account === currentEntry.account &&
							entry.kind === currentEntry.kind)
				) || null;
			const id = isObject(active) ? active.account : active?.toString();
			if (id) {
				const element = document.getElementById(id);
				if (element) {
					element.scrollIntoView({ behavior: "smooth", block: "nearest" });
				}
			}
			if (
				active == null &&
				!(
					isObject(
						list.find(
							(entry) =>
								isObject(entry) && isObject(currentEntry) && entry.account === currentEntry.account
						)
					) && isUser(currentEntry)
				)
			) {
				// console.log('Cannot find entry: ', active, 'at: ', currentEntry);
				if (!isNew) await back();
			}
		}
	}

	$: if (!nested) currentEntry = null;
	$: selectItem(list);
	$: if (active) {
		onHighlighted = true;
	} else {
		onHighlighted = false;
	}
	$: if (items instanceof Promise) items.then((val) => (list = val));

	let active: T | null;
	let list: T[] | null;
	let items: Promise<T[]>;
	$: if (params || role) {
		items = fetchItems(params, role, date);
	} else {
		nested = true;
	}
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
		class="list-group-item list-group-item-action list-group-item-danger d-flex align-items-center"
		on:click={() => {
			nested = true;
			params = "";
			role = null;
		}}
	>
		<div class="me-auto d-flex align-items-center">
			<h5 class="mb-0">Zurück</h5>
		</div>
		<div class="d-flex align-items-center">
			<small class="me-2"
				>{params || !role ? `"${params}"` : ""}{role && params ? " - " : ""}{role
					? role
					: ""}</small
			>
			<span class="tag tag-primary">{data.length}</span>
		</div>
	</button>
	{#each data as entry}
		<button
			class="list-group-item list-group-item-action"
			class:active={active === entry}
			id={isObject(entry) ? entry.account : entry.toString()}
			on:click={() => {
				active = entry;
				onSelect(active);
			}}>{entry.account}{isWorkless(entry) && entry.currently ? " - Arbeitslos" : ""}</button
		>
	{:else}
		<li class="list-group-item">Keine Einträge!</li>
	{/each}
{:catch error}
	<li class="list-group-item">Error: {error}</li>
{/await}

<style>
	.list-group-item-action {
		cursor: pointer;
	}
</style>
