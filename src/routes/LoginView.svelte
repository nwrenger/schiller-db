<script lang="ts" context="module">
	export interface Login {
		ty: 'login';
	}
</script>

<script lang="ts">
	import Select from "./Select.svelte";
	export var stats: () => void;
	export var search: (
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) => Promise<any[]>;
	var addAccount: string = '';
	var deleteAccount: string = '';
</script>

<div id="login-container">
	<div>
		<label for="add-login" class="form-label">Einen Login hinzufügen: </label>
		<div class="card-title row add-login">
			<Select bind:value={addAccount} {search} label={'Benutzer'} />
			<div class="col">
				<label for="login-add-password" class="form-label">Passwort</label>
				<input
					id="login-add-password"
					type="password"
					class="form-control"
					placeholder="Passwort"
					aria-label="Passwort"
				/>
			</div>
		</div>
		<div class="row" style="padding-top: 5px;">
			<div class="col">
				<label for="login-add-user-permissions" class="form-label">Rechte für Bürger</label>
				<select id="login-add-user-permissions" class="form-select" aria-label="Permissions">
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
			<div class="col">
				<label for="login-add-workless-permissions" class="form-label">Rechte für Arbeitslose</label
				>
				<select id="login-add-workless-permissions" class="form-select" aria-label="Permissions">
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
			<div class="col">
				<label for="login-add-criminal-permissions" class="form-label"
					>Rechte für das Kriminalregister</label
				>
				<select id="login-add-criminal-permissions" class="form-select" aria-label="Permissions">
					<option value="None">None</option>
					<option value="ReadOnly">ReadOnly</option>
					<option value="Write">Write</option>
				</select>
			</div>
		</div>
		<button id="add-login-button" type="button" class="btn btn-outline-danger m-3">
			<span
				id="add-login-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
				hidden
			/>
			Hinzufügen
		</button>
	</div>
	<div>
		<label for="delete-login" class="form-label">Einen Login entfernen:</label>
		<div class="card-title row delete-login">
			<Select bind:value={deleteAccount} {search} label={'Benutzer'} />
		</div>
		<button id="delete-login-button" type="button" class="btn btn-outline-danger m-3">
			<span
				id="delete-login-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
				hidden
			/>
			Entfernen
		</button>
	</div>
	<div>
		<p style="margin: 0;">Alle Logins entfernen:</p>
		<button
			id="delete-all-logins-button"
			type="button"
			class="btn btn-outline-danger m-3 delete-all-logins"
		>
			<span
				id="delete-all-logins-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
				hidden
			/>
			Alle Logins löschen
		</button>
	</div>
	<button class="btn btn-outline-danger m-2" type="button" on:click={() => stats()}
		>Schließen</button
	>
</div>
