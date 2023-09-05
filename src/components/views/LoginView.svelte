<script lang="ts" context="module">
	export interface Login {
		ty: "login";
	}
</script>

<script lang="ts">
	import Dialog from "../basic/Dialog.svelte";
	import Select from "../buttons/Select.svelte";

	export var back: () => Promise<void>;
	export var search: (
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) => Promise<any[]>;
	export var request: (
		url: string,
		type: string,
		json: BodyInit | null | undefined
	) => Promise<any>;

	let addUser: string = "";
	let password: string = "";
	let userPermissions: string = "None";
	let worklessPermissions: string = "None";
	let criminalPermissions: string = "None";
	let addResponse: Promise<any>;
	let dialog: Dialog;

	async function add() {
		await request(
			"/api/login",
			"POST",
			JSON.stringify({
				user: addUser,
				password: password,
				access_user: userPermissions,
				access_workless: worklessPermissions,
				access_criminal: criminalPermissions
			})
		);
	}

	let deleteUser: string = "";
	let delResponse: Promise<any>;

	async function del() {
		await request(`/api/login/${deleteUser}`, "DELETE", null);
	}

	async function delAll() {
		await request("/api/all_logins", "DELETE", null);
	}
</script>

<div id="login-container">
	<Dialog bind:this={dialog} fun={delAll} />
	<div>
		<h4>Logins hinzufügen</h4>
		<div class="card-title row add-login">
			<Select bind:value={addUser} {search} label={"Benutzer"} />
			<div class="col">
				<label for="login-add-password" class="form-label">Passwort</label>
				<input
					id="login-add-password"
					type="password"
					class="form-control"
					placeholder="Passwort"
					aria-label="Passwort"
					bind:value={password}
				/>
			</div>
		</div>
		<div class="row" style="padding-top: 5px;">
			<div class="col">
				<label for="login-add-user-permissions" class="form-label">Rechte für Bürger</label>
				<select
					id="login-add-user-permissions"
					class="form-select"
					aria-label="Permissions"
					bind:value={userPermissions}
				>
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
			<div class="col">
				<label for="login-add-workless-permissions" class="form-label"
					>Rechte für Arbeitslosenreg.</label
				>
				<select
					id="login-add-workless-permissions"
					class="form-select"
					aria-label="Permissions"
					bind:value={worklessPermissions}
				>
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
			<div class="col">
				<label for="login-add-criminal-permissions" class="form-label"
					>Rechte für das Kriminalregister</label
				>
				<select
					id="login-add-criminal-permissions"
					class="form-select"
					aria-label="Permissions"
					bind:value={criminalPermissions}
				>
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
		</div>
		<button
			id="add-login-button"
			type="button"
			class="btn btn-outline-primary m-3"
			on:click={() => (addResponse = add())}
		>
			{#await addResponse}
				<span
					id="add-login-button-spinner"
					class="spinner-border spinner-border-sm"
					role="status"
					aria-hidden="true"
				/>
			{/await}
			Hinzufügen
		</button>
	</div>
	<div>
		<h4>Logins entfernen</h4>
		<div class="card-title row delete-login">
			<Select bind:value={deleteUser} {search} label={"Benutzer"} />
		</div>
		<button
			id="delete-login-button"
			type="button"
			class="btn btn-outline-danger m-3"
			on:click={() => (delResponse = del())}
		>
			{#await delResponse}
				<span
					id="add-login-button-spinner"
					class="spinner-border spinner-border-sm"
					role="status"
					aria-hidden="true"
				/>
			{/await}
			Entfernen
		</button>
	</div>
	<div>
		<h4>Alle Logins entfernen</h4>
		<button
			id="delete-all-logins-button"
			type="button"
			class="btn btn-outline-danger m-3 delete-all-logins"
			on:click={() => {
				if (dialog) dialog.open("Warnung", "Alle Logins entfernen?");
			}}
		>
			Alle Logins entfernen
		</button>
	</div>
	<button class="btn btn-outline-secondary m-2" type="button" on:click={async () => await back()}
		>Schließen</button
	>
</div>
