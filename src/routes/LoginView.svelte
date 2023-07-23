<script lang="ts" context="module">
	export interface Login {
		ty: 'login';
	}
</script>

<script lang="ts">
	export var stats: () => void;
</script>

<div id="login-container">
	<div>
		<label for="add-login" class="form-label">Einen Login hinzufügen: </label>
		<div class="card-title row add-login">
			<div class="col">
				<label for="login-users" class="form-label">Benutzer</label>
				<div class="input-group mb-3 login-users">
					<button
						id="criminal-select-button"
						class="btn btn-outline-danger dropdown-toggle hide-arrow"
						type="button"
						data-bs-toggle="dropdown"
						aria-expanded="false"
						title="Auswählen"
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
					<ul id="login-add-select-dropdown" class="dropdown-menu" />
					<input
						id="login-add-user"
						type="text"
						class="form-control"
						placeholder="Benutzer"
						aria-label="Benutzer"
					/>
				</div>
			</div>
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
		<div class="card-title row col delete-login">
			<label for="login-users" class="form-label">Benutzer</label>
			<div class="input-group mb-3 login-users">
				<button
					id="criminal-select-button"
					class="btn btn-outline-danger dropdown-toggle hide-arrow"
					type="button"
					data-bs-toggle="dropdown"
					aria-expanded="false"
					title="Auswählen"
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
				<ul id="login-delete-select-dropdown" class="dropdown-menu" />
				<input
					id="login-delete-user"
					type="text"
					class="form-control"
					placeholder="Benutzer"
					aria-label="Benutzer"
				/>
			</div>
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
