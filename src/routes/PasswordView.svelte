<script lang="ts" context="module">
	export interface Password {
		ty: 'password';
	}
</script>

<script lang="ts">
	export var stats: () => void;
</script>

<div id="password-changer-container">
	<div>
		<label for="password-changer" class="form-label">Passwort ändern: </label>
		<div class="card-title row password-changer">
			<div class="col">
				<label for="new-password" class="form-label">Neues Passwort</label>
				<input
					id="new-password"
					type="password"
					class="form-control"
					placeholder="Neues Passwort"
					aria-label="Neues Passwort"
				/>
			</div>
			<div class="col">
				<label for="new-password-wdh" class="form-label">Wiederholen</label>
				<input
					id="new-password-wdh"
					type="password"
					class="form-control"
					placeholder="Wiederholen"
					aria-label="Wiederholen"
				/>
			</div>
		</div>
		<button id="change-password-button" type="button" class="btn btn-outline-danger m-3">
			<span
				hidden
				id="change-password-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
			Ändern
		</button>
	</div>
	<button class="btn btn-outline-danger m-2" type="button" on:click={() => stats()}
		>Schließen</button
	>
</div>
