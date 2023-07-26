<script lang="ts" context="module">
	export interface User {
		ty: 'user';
		forename: string;
		surname: string;
		account: string;
		role: string;
	}
</script>

<script lang="ts">
	export let user: User | null;
	export let editable: boolean = false;
	export let isNew: boolean = false;
	export let onHighlighted: boolean;
	export let searchRole: string | null;
	export var back: () => Promise<void>;
	export var request: (
		url: string,
		type: string,
		json: BodyInit | null | undefined
	) => Promise<any>;
	export var reload: () => void;

	let forename = '';
	let surname = '';
	let account = '';
	let role = '';

	$: if (editable || isNew || !editable || !isNew) setUser(user);
	$: if (searchRole) role = searchRole;

	function setUser(user: User | null) {
		if (user && !isNew) {
			forename = user.forename;
			surname = user.surname;
			account = user.account;
			role = user.role;
		} else {
			forename = '';
			surname = '';
			account = '';
			if (role) {
				role = searchRole as string;
			} else {
				role = '';
			}
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		await request('/api/user', 'POST', JSON.stringify({ forename, surname, account, role }));
		await back();
		reload();
	}

	let editResponse: Promise<any>;
	async function edit() {
		await request(
			`/api/user/${user?.account}`,
			'PUT',
			JSON.stringify({ forename, surname, account, role })
		);
		await back();
		reload();
	}
	export async function del() {
		await request(`/api/user/${user?.account}`, 'DELETE', null);
		await back();
		reload();
	}
</script>

<div id="user-container">
	<div class="card-title row">
		<div class="col">
			<label for="forename" class="form-label">Vorname</label>
			<input
				id="forename"
				type="text"
				class="form-control"
				placeholder="Vorname"
				aria-label="Vorname"
				readonly={!editable}
				bind:value={forename}
			/>
		</div>
		<div class="col">
			<label for="surname" class="form-label">Nachname</label>
			<input
				id="surname"
				type="text"
				class="form-control"
				placeholder="Nachname"
				aria-label="Nachname"
				readonly={!editable}
				bind:value={surname}
			/>
		</div>
	</div>
	<div class="row">
		<div class="col">
			<label for="account" class="form-label">Account</label>
			<input
				id="account"
				type="text"
				class="form-control"
				placeholder="Account"
				aria-label="Account"
				readonly={!editable}
				bind:value={account}
			/>
		</div>
		<div class="col">
			<label for="role" class="form-label">Gruppe</label>
			<input
				id="role"
				type="text"
				class="form-control"
				placeholder="Gruppe"
				aria-label="Gruppe"
				readonly={!editable}
				bind:value={role}
			/>
		</div>
	</div>

	<button
		id="user-add-button"
		class="btn btn-outline-danger m-3"
		type="button"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		{#await addResponse}
			<span
				id="user-add-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Hinzufügen
	</button>

	<button
		id="user-confirm-button"
		type="button"
		class="btn btn-outline-danger m-3"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		{#await editResponse}
			<span
				id="user-confirm-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Bestätigen
	</button>

	<button
		id="user-abort-button"
		type="button"
		class="btn btn-outline-danger m-3"
		hidden={!editable}
		on:click={async () => {
			if (!onHighlighted) {
				await back();
			} else {
				setUser(user);
				editable = false;
				isNew = false;
			}
		}}
	>
		Abbrechen
	</button>
</div>
