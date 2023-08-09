<script lang="ts">
	import Navigation from '../../components/basic/Navigation.svelte'
	import Dialog from '../../components/basic/Dialog.svelte';
	import { goto } from '$app/navigation';
	import LoginForm from './LoginForm.svelte';

	let newDialog: Dialog;

	async function handleLogin(username: string, password: string) {
		const auth = btoa(username + ':' + password);
		// getting all roles
		const url = '/api/login/fetch/' + encodeURIComponent(username);
		const response = await fetch(url, {
			method: 'GET',
			headers: {
				Authorization: 'Basic ' + auth,
				'Content-Type': 'application/json; charset=utf-8'
			}
		});

		if (response.status === 200) {
			const data = await response.json();
			//get with getItem and clear at logout completely with clear
			window.localStorage.setItem('auth', auth);
			window.localStorage.setItem('current_user', username);
			window.localStorage.setItem('permission', JSON.stringify(data['Ok']));

			goto('/', { replaceState: true });
		} else {
			newDialog.open('Fehler', 'Falsche Anmeldedaten!');

			const all_elements = document.getElementsByTagName('input');
			for (const element of all_elements) {
				element.classList.add('is-invalid');
			}
		}
	}
</script>

<svelte:head>
	<title>SchillerDB - Login</title>
	<meta name="description" content="Login Page" />
</svelte:head>

<section>
	<div class="main">
		<Navigation currentUser={null} onSelect={null} permission={null} />

		<div class="container">
			<div class="row p-2 h-75 align-items-center justify-content-center">
				<div class="col-md-6">
					<LoginForm {handleLogin} />
				</div>
			</div>
		</div>
	</div>
	<Dialog bind:this={newDialog} fun={undefined} />
</section>

<style>
	.main {
		display: grid;
		grid-template:
			'nav' 60px
			'container' auto;
		height: 100%;
	}

	.container {
		grid-area: container;
	}
</style>
