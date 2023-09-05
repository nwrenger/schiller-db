<script lang="ts">
	export let handleLogin: (username: string, password: string) => void;

	let username = "";
	let password = "";
</script>

<div class="card">
	<div class="card-body">
		<h5 class="card-title p-1">Login</h5>
		<div class="card-text">
			<form on:submit={() => handleLogin(username, password)}>
				<div class="form-floating mb-3">
					<input
						type="text"
						class="form-control"
						id="username"
						placeholder="Benutzername eingeben"
						bind:value={username}
						required
					/>
					<label for="username">Benutzername</label>
				</div>
				<div class="form-floating mb-3">
					<input
						type="password"
						class="form-control"
						id="password"
						bind:value={password}
						placeholder="Passwort eingeben"
						required
					/>
					<label for="password">Passwort</label>
				</div>
				<div class="d-grid gap-2">
					<button type="submit" class="btn btn-primary">Login</button>
				</div>
			</form>
		</div>
	</div>
</div>
