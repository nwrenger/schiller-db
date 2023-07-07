<script lang="ts">
	import { enhance } from '$app/forms';

	let username = '';
	let password = '';
	async function handleLogin(username: string, password: string) {
		const auth = btoa(username + ':' + password);
		// getting all roles
		const url = '/login/fetch/' + encodeURIComponent(username);
		const response = await fetch(url, {
			method: 'GET',
			headers: {
				Authorization: 'Basic ' + auth,
				'Content-Type': 'application/json; charset=utf-8'
			}
		});

		const data = await response.json();

		if (response.status === 200) {
			//get with getItem and clear at logout completely with clear
			window.localStorage.setItem('auth', auth);
			window.localStorage.setItem('current_user', username);
			window.localStorage.setItem('permissions', JSON.stringify(data['Ok']));

			window.open('/', '_self');
		} else {
			const all_elements = document.getElementsByTagName('input');
			for (const element of all_elements) {
				element.classList.add('is-invalid');
			}
		}
	}
</script>

<svelte:head>
	<title>Login</title>
	<meta name="description" content="Login Page" />
</svelte:head>

<section>
	<div class="main">
		<nav class="nav navbar bg-secondary-subtle">
			<div class="container-fluid">
				<a href="/login.html" class="navbar-brand">SNDI</a>
			</div>
		</nav>
		<div class="container">
			<div class="row p-2 h-75 align-items-center justify-content-center">
				<div class="col-md-6">
					<div class="card">
						<div class="card-body">
							<h5 class="card-title p-1">Login</h5>
							<div class="card-text">
								<form
									method="POST"
									action="?/enter"
									use:enhance={() => {
										handleLogin(username, password);
									}}
								>
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
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.main {
		display: grid;
		grid-template:
			'nav' 60px
			'container' auto;
		height: 100%;
	}

	.nav {
		grid-area: nav;
	}

	.navbar-brand {
		font-size: x-large;
	}

	.container {
		grid-area: container;
	}
</style>
