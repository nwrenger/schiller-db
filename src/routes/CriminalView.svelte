<script lang="ts" context="module">
	export interface Criminal {
		ty: 'criminal';
		account: string;
		kind: string;
		accuser: string;
		police_consultant: string;
		lawyer_culprit: string;
		lawyer_accuser: string;
		facts: string;
		time_of_crime: string;
		location_of_crime: string;
		note: string;
		verdict: string;
	}
</script>

<script lang="ts">
	import Select from './Select.svelte';

	export let criminal: Criminal | null;
	export let editable: boolean = false;
	export let isNew: boolean = false;
	export let onHighlighted: boolean;
	export let searchAccount: string | null;
	export var getUser: () => void;
	export var reload: () => void;
	export var search: (
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) => Promise<any[]>;
	export var back: () => Promise<void>;
	export var request: (
		url: string,
		type: string,
		json: BodyInit | null | undefined
	) => Promise<any>;

	let account = '';
	let kind = '';
	let accuser = '';
	let police_consultant = '';
	let lawyer_culprit = '';
	let lawyer_accuser = '';
	let facts = '';
	let time_of_crime = '';
	let location_of_crime = '';
	let note = '';
	let verdict = '';

	$: if (editable || isNew || !editable || !isNew) setCriminal(criminal);
	$: if (searchAccount) account = searchAccount;

	function setCriminal(criminal: Criminal | null) {
		if (!isNew) {
			if (criminal) {
				account = criminal.account;
				kind = criminal.kind;
				accuser = criminal.accuser;
				police_consultant = criminal.police_consultant;
				lawyer_culprit = criminal.lawyer_culprit;
				lawyer_accuser = criminal.lawyer_accuser;
				facts = criminal.facts;
				time_of_crime = criminal.time_of_crime;
				location_of_crime = criminal.location_of_crime;
				note = criminal.note;
				verdict = criminal.verdict;
			}
		} else {
			if (searchAccount) {
				account = searchAccount as string;
			} else {
				account = '';
			}
			kind = '';
			accuser = '';
			police_consultant = '';
			lawyer_culprit = '';
			lawyer_accuser = '';
			facts = '';
			time_of_crime = '';
			location_of_crime = '';
			note = '';
			verdict = '';
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		await request(
			'/api/criminal',
			'POST',
			JSON.stringify({
				account,
				kind,
				accuser,
				police_consultant,
				lawyer_culprit,
				lawyer_accuser,
				facts,
				time_of_crime,
				location_of_crime,
				note,
				verdict
			})
		);
		onChange();
	}

	let editResponse: Promise<any>;
	async function edit() {
		await request(
			`/api/criminal/${criminal?.account}/${criminal?.kind}`,
			'PUT',
			JSON.stringify({
				account,
				kind,
				accuser,
				police_consultant,
				lawyer_culprit,
				lawyer_accuser,
				facts,
				time_of_crime,
				location_of_crime,
				note,
				verdict
			})
		);
		onChange();
	}
	export async function del() {
		await request(`/api/criminal/${criminal?.account}/${criminal?.kind}`, 'DELETE', null);
		await onDel();
	}

	function onChange() {
		criminal = {
			ty: 'criminal',
			account,
			kind,
			accuser,
			police_consultant,
			lawyer_culprit,
			lawyer_accuser,
			facts,
			time_of_crime,
			location_of_crime,
			note,
			verdict
		};
		// console.log('Changed: ', criminal);
		editable = false;
		isNew = false;
		reload();
	}

	async function onDel() {
		criminal = null;
		await back();
		reload();
	}
</script>

<div id="criminal-container">
	<div class="card-title row">
		<Select bind:value={account} {editable} {search} label={'Beschuldigter'} />
		<div class="col">
			<label for="kind" class="form-label">Art</label>
			<input
				id="kind"
				type="text"
				class="form-control"
				placeholder="Art"
				aria-label="Art"
				readonly={!editable}
				bind:value={kind}
			/>
		</div>
	</div>
	<div class="row">
		<Select bind:value={accuser} {editable} {search} label={'Anzeiger'} />
		<Select bind:value={police_consultant} {editable} {search} label={'Sachberater Polizei'} />
	</div>
	<div class="row">
		<Select bind:value={lawyer_culprit} {editable} {search} label={'Anwalt des Beschuldigten'} />
		<Select bind:value={lawyer_accuser} {editable} {search} label={'Anwalt des Anzeigers'} />
	</div>
	<div class="row">
		<div class="col">
			<label for="facts" class="form-label">Tatbestand</label>
			<input
				id="facts"
				type="text"
				class="form-control"
				placeholder="Tatbestand"
				aria-label="Tatbestand"
				readonly={!editable}
				bind:value={facts}
			/>
		</div>
	</div>
	<div class="row">
		<div class="col">
			<label for="time-of-crime" class="form-label">Zeitpunkt der Tat</label>
			<input
				id="time-of-crime"
				type="text"
				class="form-control"
				placeholder="Zeitpunkt der Tat"
				aria-label="Zeitpunkt der Tat"
				readonly={!editable}
				bind:value={time_of_crime}
			/>
		</div>
		<div class="col">
			<label for="location-of-crime" class="form-label">Ort der Tat</label>
			<input
				id="location-of-crime"
				type="text"
				class="form-control"
				placeholder="Ort der Tat"
				aria-label="Ort der Tat"
				readonly={!editable}
				bind:value={location_of_crime}
			/>
		</div>
	</div>
	<div class="row">
		<div class="col">
			<label for="note" class="form-label">Kommentar</label>
			<input
				id="note"
				type="text"
				class="form-control"
				placeholder="Kommentar"
				aria-label="Kommentar"
				readonly={!editable}
				bind:value={note}
			/>
		</div>
	</div>
	<div class="row">
		<div class="col">
			<label for="verdict" class="form-label">Urteil</label>
			<div class="input-group mb-3 verdict-select">
				<button
					id="verdict-select-button"
					class="btn btn-outline-secondary dropdown-toggle"
					type="button"
					data-bs-toggle="dropdown"
					aria-expanded="false"
					title="Auswählen"
					disabled={!editable}>Urteil</button
				>
				<ul id="verdict-select-dropdown" class="dropdown-menu">
					<li>
						<button
							id="no-yet"
							class="dropdown-item"
							type="button"
							on:click={() => (verdict = 'a.) Noch kein Verfahren')}>a.) Noch kein Verfahren</button
						>
					</li>
					<li>
						<button
							id="guilty"
							class="dropdown-item"
							type="button"
							on:click={() => (verdict = 'b.) Schuldig')}>b.) Schuldig</button
						>
					</li>
					<li>
						<button
							id="innocent"
							class="dropdown-item"
							type="button"
							on:click={() => (verdict = 'c.) Unschuldig')}>c.) Unschuldig</button
						>
					</li>
				</ul>
				<input
					id="verdict"
					type="text"
					class="form-control"
					placeholder="Urteil"
					aria-label="Urteil"
					readonly={!editable}
					bind:value={verdict}
				/>
			</div>
		</div>
	</div>
	<button
		id="criminal-add-button"
		type="button"
		class="btn btn-outline-primary m-3"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		{#await addResponse}
			<span
				id="criminal-add-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Hinzufügen</button
	>
	<button
		id="criminal-confirm-button"
		type="button"
		class="btn btn-outline-primary m-3"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		{#await editResponse}
			<span
				id="criminal-confirm-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Bestätigen</button
	>
	<button
		id="criminal-abort-button"
		type="button"
		class="btn btn-outline-secondary m-3"
		hidden={!editable}
		on:click={async () => {
			if (!onHighlighted) {
				await back();
			} else {
				setCriminal(criminal);
				editable = false;
				isNew = false;
			}
		}}>Abbrechen</button
	>
	<button
		type="button"
		class="btn btn-outline-secondary m-3 get-user"
		style="max-width: 160px;"
		hidden={editable}
		on:click={() => getUser()}>Bürger abrufen</button
	>
</div>
