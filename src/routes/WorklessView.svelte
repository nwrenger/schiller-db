<script lang="ts" context="module">
    export interface Workless {
		ty: "workless"
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
	export var getUser: () => void;
	export var search: (
		params: string,
		kind: string,
		role: string | null,
		limit: number | null
	) => Promise<any[]>;

	let account = '';
	let old_company = '';
	let date_of_dismiss = '';
	let currently = false;
	let new_company = '';
	let total_time = '';

    $: setWorkless(workless);

	function setWorkless(workless: Workless | null) {
		if (workless) {
			account = workless.account;
			old_company = workless.old_company;
			date_of_dismiss = workless.date_of_dismiss;
			currently = workless.currently;
			new_company = workless.new_company;
			total_time = workless.total_time;
		} else {
			account = '';
			old_company = '';
			date_of_dismiss = '';
			currently = false;
			new_company = '';
			total_time = '';
		}
	}

	function onChange() {
		workless = { ty: "workless", account, old_company, date_of_dismiss, currently, new_company, total_time };
		console.log(`Change ${workless}`);
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
					<li><button id="yes-currently" class="dropdown-item" type="button">Ja</button></li>
					<li><button id="no-currently" class="dropdown-item" type="button">Nein</button></li>
				</ul>
				<input
					id="currently-workless"
					type="text"
					class="form-control"
					placeholder="Auswahl"
					aria-label="Auswahl"
					readonly={!editable}
					bind:value={currently}
				/>
			</div>
		</div>
	</div>
	<div id="only-on-currently-no" class="row" hidden>
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
		on:click={onChange}
		><span
			id="workless-add-button-spinner"
			class="spinner-border spinner-border-sm"
			role="status"
			aria-hidden="true"
			hidden
		/>Hinzufügen</button
	>
	<button
		id="workless-confirm-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!(editable && !isNew)}
		on:click={onChange}
		><span
			id="workless-confirm-button-spinner"
			class="spinner-border spinner-border-sm"
			role="status"
			aria-hidden="true"
			hidden
		/>Bestätigen</button
	>
	<button
		id="workless-abort-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!editable}
		on:click={() => {
			editable = false;
			setWorkless(workless);
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
