<script lang="ts" context="module">
	export interface Workless {
		ty: 'workless';
		account: string;
		old_company: string;
		date_of_dismiss: string;
		currently: boolean;
		new_company: string;
		total_time: string;
	}
</script>

<script lang="ts">
	import Select from './Select.svelte';

	export let workless: Workless | null;
	export let editable: boolean = false;
	export let isNew: boolean = false;
	export let onHighlighted: boolean;
	export let searchDate: string | null;
	export var getUser: () => void;
	export var search: (
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) => Promise<any[]>;
	export var back: () => Promise<void>;
	export var reload: () => void;
	export var request: (
		url: string,
		type: string,
		json: BodyInit | null | undefined
	) => Promise<any>;

	let account = '';
	let old_company = '';
	let date_of_dismiss = '';
	let currently = false;
	let new_company = '';
	let total_time = '';

	$: if (editable || isNew || !editable || !isNew) setWorkless(workless);
	$: if (searchDate) date_of_dismiss = searchDate;

	function setWorkless(workless: Workless | null) {
		if (!isNew) {
			if (workless) {
				account = workless.account;
				old_company = workless.old_company;
				date_of_dismiss = workless.date_of_dismiss;
				currently = workless.currently;
				new_company = workless.new_company;
				total_time = workless.total_time;
			}
		} else {
			account = '';
			old_company = '';
			if (date_of_dismiss) {
				date_of_dismiss = searchDate as string;
			} else {
				date_of_dismiss = '';
			}
			currently = false;
			new_company = '';
			total_time = '';
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		await request(
			'/api/workless',
			'POST',
			JSON.stringify({ account, old_company, date_of_dismiss, currently, new_company, total_time })
		);
		await reset();
	}

	let editResponse: Promise<any>;
	async function edit() {
		await request(
			`/api/workless/${workless?.account}/${workless?.old_company}/${workless?.date_of_dismiss}`,
			'PUT',
			JSON.stringify({ account, old_company, date_of_dismiss, currently, new_company, total_time })
		);
		onChange();
	}
	export async function del() {
		await request(
			`/api/workless/${workless?.account}/${workless?.old_company}/${workless?.date_of_dismiss}`,
			'DELETE',
			null
		);
		await reset();
	}

	function onChange() {
		workless = {
			ty: 'workless',
			account,
			old_company,
			date_of_dismiss,
			currently,
			new_company,
			total_time
		};
		console.log('Changed: ', workless);
		editable = false;
		isNew = false;
		reload();
	}

	async function reset() {
		workless = null;
		await back();
	}
</script>

<div id="workless-container">
	<div class="card-title row">
		<Select bind:value={account} {editable} {search} label={'Account'} />
		<div class="col">
			<label for="old-company" class="form-label">Vorheriger Betrieb</label>
			<input
				id="old-company"
				type="text"
				class="form-control"
				placeholder="Vorgeriger Betrieb"
				aria-label="Vorgeriger Betrieb"
				readonly={!editable}
				bind:value={old_company}
			/>
		</div>
	</div>
	<div class="row">
		<div class="col form-group">
			<label for="date-of-dismiss" class="form-label">Datum der Entlassung</label>
			<input
				type="date"
				class="form-control"
				id="date-of-dismiss"
				readonly={!editable}
				bind:value={date_of_dismiss}
			/>
		</div>
		<div class="col">
			<label for="currently-workless" class="form-label">Aktuell Arbeitslos</label>
			<div class="input-group mb-3 currently-select">
				<button
					id="currently-select-button"
					class="btn btn-outline-danger dropdown-toggle"
					type="button"
					data-bs-toggle="dropdown"
					aria-expanded="false"
					title="Auswahl"
					disabled={!editable}>Auswahl</button
				>
				<ul id="currently-select-dropdown" class="dropdown-menu">
					<li>
						<button
							id="yes-currently"
							class="dropdown-item"
							type="button"
							on:click={() => (currently = true)}>Ja</button
						>
					</li>
					<li>
						<button
							id="no-currently"
							class="dropdown-item"
							type="button"
							on:click={() => (currently = false)}>Nein</button
						>
					</li>
				</ul>
				<input
					id="currently-workless"
					type="text"
					class="form-control"
					placeholder="Auswahl"
					aria-label="Auswahl"
					readonly
					value={currently ? 'Ja' : 'Nein'}
				/>
			</div>
		</div>
	</div>
	<div id="only-on-currently-no" class="row" hidden={currently}>
		<div class="col">
			<label for="new-company" class="form-label">Neuer Betrieb</label>
			<input
				id="new-company"
				type="text"
				class="form-control"
				placeholder="Neuer Betrieb"
				aria-label="Neuer Betrieb"
				readonly={!editable}
				bind:value={new_company}
			/>
		</div>
		<div class="col">
			<label for="total-time" class="form-label">Insgeammte arbeitslose Zeit</label>
			<input
				id="total-time"
				type="text"
				class="form-control"
				placeholder="Insgeammte arbeitslose Zeit"
				aria-label="Insgeammte arbeitslose Zeit"
				readonly={!editable}
				bind:value={total_time}
			/>
		</div>
	</div>
	<button
		id="workless-add-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		{#await addResponse}
			<span
				id="workless-add-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Hinzufügen</button
	>
	<button
		id="workless-confirm-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		{#await editResponse}
			<span
				id="workless-confirm-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Bestätigen</button
	>
	<button
		id="workless-abort-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!editable}
		on:click={async () => {
			if (!onHighlighted) {
				await back();
			} else {
				setWorkless(workless);
				editable = false;
				isNew = false;
			}
		}}>Abbrechen</button
	>
	<button
		type="button"
		class="btn btn-outline-danger m-3 justify-content-center get-workless"
		on:click={() => getUser()}
		style="max-width: 160px;"
		hidden={editable}>Bürger abrufen</button
	>
</div>
