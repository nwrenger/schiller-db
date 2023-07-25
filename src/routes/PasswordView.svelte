<script lang="ts" context="module">
	export interface Password {
		ty: 'password';
	}
</script>

<script lang="ts">
	export var auth: string | null;
	export let current_user: string | null;
	export var stats: () => void;
	export var request: (
		url: string,
		type: string,
		json: BodyInit | null | undefined
	) => Promise<any>;
	export var error: (error: string) => void;
	export var info: (info: string) => void;

	var newPassword: string;
	var wdhPassword: string;
	var valid: string;
	var response: Promise<any>;

	async function changePassword() {
		if (newPassword == wdhPassword) {
			await request(
				'/api/login',
				'PUT',
				JSON.stringify({
					user: current_user,
					password: newPassword,
					access_user: 'None',
					access_workless: 'None',
					access_criminal: 'None'
				})
			);
			auth = btoa(current_user + ':' + newPassword);
			window.localStorage.setItem('auth', auth);
			valid = '';
			info('Passwort Änderung war Erfolgreich!');
		} else {
			valid = 'is-invalid';
			error('Falsche Passwort Wiederholung!');
		}
	}
</script>

<div id="password-changer-container">
	<form on:submit={() => (response = changePassword())}>
		<label for="password-changer" class="form-label">Passwort ändern: </label>
		<div class="card-title row password-changer">
			<div class="col">
				<label for="new-password" class="form-label">Neues Passwort</label>
				<input
					id="new-password"
					type="password"
					class="form-control {valid}"
					placeholder="Neues Passwort"
					aria-label="Neues Passwort"
					bind:value={newPassword}
				/>
			</div>
			<div class="col">
				<label for="new-password-wdh" class="form-label">Wiederholen</label>
				<input
					id="new-password-wdh"
					type="password"
					class="form-control {valid}"
					placeholder="Wiederholen"
					aria-label="Wiederholen"
					bind:value={wdhPassword}
				/>
			</div>
		</div>
		<button id="change-password-button" type="submit" class="btn btn-outline-danger m-3">
			{#await response}
				<span
					id="change-password-button-spinner"
					class="spinner-border spinner-border-sm"
					role="status"
					aria-hidden="true"
				/>
			{/await}
			Ändern
		</button>
	</form>
	<button class="btn btn-outline-danger m-2" type="button" on:click={() => stats()}
		>Schließen</button
	>
</div>
