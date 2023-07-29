<script lang="ts">
	import { goto } from '$app/navigation';

	export let currentUser: string;
	export let accessUser: string | null;
	export let onSelect: ((val: string) => void) | null;
</script>

<div class="d-flex">
	<div class="btn-group dropdown">
		<button
			class="btn btn-outline-secondary dropdown-toggle hide-arrow"
			type="button"
			title="Profil"
			data-bs-toggle="dropdown"
			aria-expanded="false"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentColor"
				class="bi bi-person"
				viewBox="0 0 16 16"
			>
				<path
					d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0Zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4Zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10Z"
				/>
			</svg>
		</button>
		<ul class="dropdown-menu dropdown-menu-end">
			<li>
				<h6 class="dropdown-header">{currentUser}</h6>
			</li>
			<li>
				<button
					class="dropdown-item"
					type="button"
					on:click={() => {
						if (onSelect) onSelect('password');
					}}>Passwort ändern</button
				>
			</li>
			<li>
				<button
					id="login-creator"
					class="dropdown-item"
					type="button"
					disabled={accessUser === 'Write' ? false : true}
					on:click={() => {
						if (onSelect) onSelect('login');
					}}>Logins Verwalten</button
				>
			</li>
			<li>
				<button
					class="dropdown-item"
					type="button"
					on:click={() => {
						localStorage.clear();
						goto('/login', { replaceState: true });
					}}>Ausloggen</button
				>
			</li>
		</ul>
	</div>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}
</style>
