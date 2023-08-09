<script lang="ts">
	import type { User } from '../views/UserView.svelte';
	export var value = '';
	export var label = '';
	export let editable: boolean = true;
	export var search: (
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) => Promise<User[]>;

	let items: Promise<User[]> | never[] = [];
</script>

<div class="col">
	<label for="select" class="form-label">{label}</label>
	<div class="input-group mb-3 select">
		<button
			id="select-button"
			class="btn btn-outline-secondary dropdown-toggle hide-arrow"
			type="button"
			data-bs-toggle="dropdown"
			aria-expanded="false"
			title="Auswählen"
			disabled={!editable}
			on:click={() => {
				items = search(value, 'user', null, null, 10);
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentColor"
				class="bi bi-search"
				viewBox="0 0 16 16"
			>
				<path
					d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"
				/>
			</svg>
		</button>
		<ul id="select-dropdown" class="dropdown-menu" hidden={!editable}>
			{#await items}
				<li class="dropdown-item">
					<div class="d-flex justify-content-center">
						<div class="spinner-grow" role="status">
							<span class="visually-hidden">Loading...</span>
						</div>
					</div>
				</li>
			{:then data}
				{#each data as entry}
					<button
						class="dropdown-item"
						on:click={() => {
							value = entry.account;
						}}>{entry.account}</button
					>
				{:else}
					<button class="dropdown-item" disabled>Keine Einträge!</button>
				{/each}
			{/await}
		</ul>
		<input
			id="account"
			type="text"
			class="form-control"
			placeholder={label}
			aria-label={label}
			readonly={!editable}
			bind:value
		/>
	</div>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}
</style>
