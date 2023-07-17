<script lang="ts">
	import { writable, type Writable } from 'svelte/store';

	import { goto } from '$app/navigation';
	import Navigation from './Navigation.svelte';
	import User from './User.svelte';

	/// Request Function
	async function request(url: string, type: string, json: BodyInit | null | undefined) {
		const response = await fetch(url, {
			method: type,
			headers: {
				Authorization: 'Basic ' + auth,
				'Content-Type': 'application/json; charset=utf-8'
			},
			body: json
		});

		let data = await response.json();

		if (response.status === 200 && !data['Err']) {
			return data['Ok'];
		} else {
			error(data['Err']);
		}
	}

	/// Storage
	var auth = localStorage.getItem('auth');
	const current_user = localStorage.getItem('current_user');
	var permissions = localStorage.getItem('permissions');
	if (permissions) {
		permissions = JSON.parse(permissions);
	}

	if (!auth || !current_user || !permissions) {
		goto('/login', { replaceState: true });
		error('InvalidLocalKeys');
	}

	/// Modals
	import Dialog from './Dialog.svelte';
	let newDialog: Dialog;

	function error(error: string) {
		newDialog.open(error);
		throw error;
	}

	async function stats() {
		if (!($mainView && typeof $mainView == 'object' && $mainView.ty == 'stats')) {
			const statsData = await request('/api/stats', 'GET', null);
			const devs = statsData.developer.split(':');
	
			$mainView = {
				ty: 'stats',
				name: statsData.name,
				version: statsData.version,
				developers: 'Programmer/Project Lead ' + devs[0] + ' und Assistant Programmer ' + devs[1],
				repo: statsData.repo,
				description: statsData.description,
				users: statsData.users
			};
		}
	}

	/// Sidebar List
	import NestedList from './NestedList.svelte';

	interface User {
		ty: 'user';
		forename: string;
		surname: string;
		account: string;
		role: string;
	}

	interface Workless {
		ty: 'workless';
		account: string;
		old_company: string;
		date_of_dismiss: string;
		currently: boolean;
		new_company: string;
		total_time: string;
	}
	interface Criminal {
		ty: 'criminal';
		account: string;
		kind: string;
		accuser: string;
		police_consultant: string;
		lawyer_culprit: string;
		lawyer_accuser: string;
		facts: string;
		time_of_crime: string;
		location_of_crime: string;
		note: string;
		verdict: string;
	}
	interface Login {
		ty: 'login';
	}
	interface Password {
		ty: 'password';
	}
	interface Stats {
		ty: 'stats';
		name: string;
		version: string;
		developers: string;
		repo: string;
		description: string;
		users: string;
	}

	type ListItem = User | Workless | Criminal | string;

	async function sidebarData(nested: ListItem | null) {
		var data: ListItem[] = [];
		if (nested && typeof nested == 'string') {
			if ($sidebarState === 'user') {
				data = await request(`/api/user/search?role=${encodeURIComponent(nested)}`, 'GET', null);
			} else if ($sidebarState === 'workless') {
				data = await request(
					`/api/workless/search?date=${encodeURIComponent(nested)}`,
					'GET',
					null
				);
			} else if ($sidebarState === 'criminal') {
				data = await request(
					`/api/criminal/search?name=${encodeURIComponent(nested)}`,
					'GET',
					null
				);
			}
		} else {
			if ($sidebarState === 'user') {
				data = await request('/api/user/all_roles', 'GET', null);
			} else if ($sidebarState === 'workless') {
				data = await request('/api/workless/all_dates', 'GET', null);
			} else if ($sidebarState === 'criminal') {
				data = await request('/api/criminal/all_accounts', 'GET', null);
			}
		}
		return data;
	}

	let nestedList: NestedList<ListItem>;

	let mainView: Writable<ListItem | Login | Password | Stats | null> = writable(null);
	let sidebarState: Writable<string | null> = writable('user');

	$: console.log($mainView);
	
	$: if ($mainView && typeof $mainView == 'object')
		if ($mainView.ty == 'stats' || $mainView.ty == 'login' || $mainView.ty == 'password')
			if (nestedList) nestedList.deselectAll();

	sidebarState.subscribe(() => {
		if (nestedList) nestedList.reset();
		stats();
	});

	function formatDate(date: string) {
		const [year, month, day] = JSON.parse(date).split('-');
		return `${day}.${month}.${year}`;
	}

	/// Container
	async function getUser() {
		if (
			$mainView &&
			typeof $mainView == 'object' &&
			($mainView.ty == 'user' || $mainView.ty == 'workless' || $mainView.ty == 'criminal')
		) {
			const current = $mainView.account;
			var data: User = await request(`/api/user/fetch/${encodeURIComponent(current)}`, 'GET', null);
			$mainView = { ...data, ty: 'user' } as User;
		}
	}

	/// Other
	function onListSelect(parents: ListItem[]): boolean {
		console.log(`List Parents: ${parents.length}`);
		if ($mainView && typeof $mainView == 'object' && Array.isArray(parents)) {
			if (parents.length == 1) return true;
			const data = parents[1] as User | Workless | Criminal;
			$mainView = { ...data, ty: $sidebarState } as User | Workless | Criminal;
		}
		return false;
	}

	async function fetchListItems(parents: ListItem[]): Promise<ListItem[]> {
		console.log(`Fetch Parents: ${parents.at(-1)}`);
		return await sidebarData(parents.at(-1) ?? null);
	}
</script>

<svelte:head>
	<title>Main</title>
	<meta name="description" content="Main Page" />
</svelte:head>

<section class="main">
	<Navigation
		onSelect={(val) => {
			if (val == 'password' || val == 'login') {
				$mainView = { ty: val };
			} else {
				stats();
			}
		}}
		currentUser={current_user}
	/>

	<!-- Sidebar -->
	<div class="sidebar bg-dark">
		<div class="bg-dark-subtle">
			<div class="btn-group p-2">
				<button
					id="add"
					class="btn btn-outline-danger"
					type="button"
					aria-expanded="false"
					title="Hinzufügen"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-plus-lg"
						viewBox="0 0 16 16"
					>
						<path
							fill-rule="evenodd"
							d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z"
						/>
					</svg>
				</button>
			</div>
			<div class="btn-group p-2">
				<button
					id="edit"
					class="btn btn-outline-danger"
					type="button"
					aria-expanded="false"
					title="Bearbeiten"
					hidden
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-pencil-square"
						viewBox="0 0 16 16"
					>
						<path
							d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"
						/>
						<path
							fill-rule="evenodd"
							d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5v11z"
						/>
					</svg>
				</button>
			</div>
			<div class="btn-group p-2">
				<button
					id="del"
					class="btn btn-outline-danger"
					type="button"
					aria-expanded="false"
					title="Entfernen"
					hidden
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-trash"
						viewBox="0 0 16 16"
					>
						<path
							d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5Zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5Zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6Z"
						/>
						<path
							d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1ZM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118ZM2.5 3h11V2h-11v1Z"
						/>
					</svg>
				</button>
			</div>
			<div class="btn-group p-2">
				<button
					id="cancel"
					class="btn btn-outline-danger"
					type="button"
					aria-expanded="false"
					title="Schließen"
					hidden
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-x-lg"
						viewBox="0 0 16 16"
					>
						<path
							d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"
						/>
					</svg>
				</button>
			</div>
		</div>

		<NestedList
			bind:this={nestedList}
			fetchItems={fetchListItems}
			onSelect={onListSelect}
			{stats}
		/>

		<div class="sidebar-search input-group pb-1 px-1">
			<button
				id="advanced"
				class="btn btn-outline-secondary dropdown-toggle hide-arrow"
				type="button"
				aria-expanded="false"
				data-bs-toggle="dropdown"
				data-bs-auto-close="outside"
				title="Nach Parametern Suchen"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					fill="currentColor"
					class="bi bi-gear"
					viewBox="0 0 16 16"
				>
					<path
						d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492zM5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0z"
					/>
					<path
						d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52l-.094-.319zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115l.094-.319z"
					/>
				</svg>
			</button>
			<ul class="dropdown-menu" id="group-select-dropdown">
				<li>
					<h6 class="dropdown-header">Gruppe</h6>
				</li>
				<form class="px-3 py-1" action="javascript:handleAdvanced()">
					<div class="mb-2">
						<select id="group-select" class="form-select" aria-label="Group Select" />
					</div>
					<button id="button-group-select" type="submit" class="btn btn-primary">
						<span
							id="spinner-group-select"
							class="spinner-border spinner-border-sm"
							role="status"
							aria-hidden="true"
							hidden
						/>
						Suchen
					</button>
				</form>
			</ul>
			<input type="text" class="form-control" placeholder="Suche" id="search" />
			<button
				id="select-button"
				class="btn btn-outline-secondary dropdown-toggle"
				type="button"
				title="Kategorie"
				data-bs-toggle="dropdown"
				aria-expanded="false"
				>{$sidebarState === 'user' ? 'Bürger' : ''}{$sidebarState === 'workless'
					? 'Arbeitslosenreg.'
					: ''}{$sidebarState === 'criminal' ? 'Kriminalregister' : ''}</button
			>
			<ul class="dropdown-menu dropdown-menu-end">
				<li>
					<h6 class="dropdown-header">Kategorie</h6>
				</li>
				<li>
					<button
						id="user"
						class={$sidebarState === 'user' ? 'dropdown-item active' : 'dropdown-item'}
						type="button"
						on:click={() => {
							sidebarState.set('user');
						}}>Bürger</button
					>
				</li>
				<li>
					<button
						id="workless"
						class={$sidebarState === 'workless' ? 'dropdown-item active' : 'dropdown-item'}
						type="button"
						on:click={() => {
							sidebarState.set('workless');
						}}>Arbeitslosenreg.</button
					>
				</li>
				<li>
					<button
						id="criminal"
						class={$sidebarState === 'criminal' ? 'dropdown-item active' : 'dropdown-item'}
						type="button"
						on:click={() => {
							sidebarState.set('criminal');
						}}>Kriminalregister</button
					>
				</li>
			</ul>
		</div>
	</div>
	<!-- Input Containers -->
	<div class="mid p-3 bg-body-secondary">
		{#if $mainView && typeof $mainView == 'object' && $mainView.ty == 'user'}
			<User user={$mainView} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'workless'}
			<div id="workless-container">
				<div class="card-title row">
					<div class="col">
						<label for="workless-select" class="form-label">Account</label>
						<div class="input-group mb-3 workless-select">
							<button
								id="workless-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="workless-select-dropdown" class="dropdown-menu" />
							<input
								id="workless-account"
								type="text"
								class="form-control"
								placeholder="Account"
								aria-label="Account"
								readonly
								value={$mainView.account}
							/>
						</div>
					</div>
					<div class="col">
						<label for="old-company" class="form-label">Vorheriger Betrieb</label>
						<input
							id="old-company"
							type="text"
							class="form-control"
							placeholder="Vorgeriger Betrieb"
							aria-label="Vorgeriger Betrieb"
							readonly
							value={$mainView.old_company}
						/>
					</div>
				</div>
				<div class="row">
					<div class="col form-group">
						<label for="date-of-dismiss" class="form-label">Datum der Entlassung</label>
						<input
							type="date"
							class="form-control"
							id="date-of-dismiss"
							readonly
							value={$mainView.date_of_dismiss}
						/>
					</div>
					<div class="col">
						<label for="currently-workless" class="form-label">Aktuell Arbeitslos</label>
						<div class="input-group mb-3 currently-select">
							<button
								id="currently-select-button"
								class="btn btn-outline-danger dropdown-toggle"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswahl"
								disabled>Auswahl</button
							>
							<ul id="currently-select-dropdown" class="dropdown-menu">
								<li><button id="yes-currently" class="dropdown-item" type="button">Ja</button></li>
								<li><button id="no-currently" class="dropdown-item" type="button">Nein</button></li>
							</ul>
							<input
								id="currently-workless"
								type="text"
								class="form-control"
								placeholder="Auswahl"
								aria-label="Auswahl"
								readonly
								value={$mainView.currently}
							/>
						</div>
					</div>
				</div>
				<div id="only-on-currently-no" class="row" hidden>
					<div class="col">
						<label for="new-company" class="form-label">Neuer Betrieb</label>
						<input
							id="new-company"
							type="text"
							class="form-control"
							placeholder="Neuer Betrieb"
							aria-label="Neuer Betrieb"
							readonly
							value={$mainView.new_company}
						/>
					</div>
					<div class="col">
						<label for="total-time" class="form-label">Insgeammte arbeitslose Zeit</label>
						<input
							id="total-time"
							type="text"
							class="form-control"
							placeholder="Insgeammte arbeitslose Zeit"
							aria-label="Insgeammte arbeitslose Zeit"
							readonly
							value={$mainView.total_time}
						/>
					</div>
				</div>
				<button id="workless-add-button" class="btn btn-outline-danger m-3" type="button" hidden
					><span
						id="workless-add-button-spinner"
						class="spinner-border spinner-border-sm"
						role="status"
						aria-hidden="true"
						hidden
					/>Hinzufügen</button
				>
				<button id="workless-confirm-button" class="btn btn-outline-danger m-3" type="button" hidden
					><span
						id="workless-confirm-button-spinner"
						class="spinner-border spinner-border-sm"
						role="status"
						aria-hidden="true"
						hidden
					/>Bestätigen</button
				>
				<button id="workless-abort-button" class="btn btn-outline-danger m-3" type="button" hidden
					>Abbrechen</button
				>
				<button
					type="button"
					class="btn btn-outline-danger m-3 justify-content-center get-user"
					on:click={() => getUser()}
					style="max-width: 160px;">Bürger abrufen</button
				>
			</div>
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal'}
			<div id="criminal-container">
				<div class="card-title row">
					<div class="col">
						<label for="criminal-select" class="form-label">Beschuldigter</label>
						<div class="input-group mb-3 criminal-select">
							<button
								id="criminal-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="criminal-select-dropdown" class="dropdown-menu" />
							<input
								id="criminal-account"
								type="text"
								class="form-control"
								placeholder="Beschuldigter"
								aria-label="Beschuldigter"
								readonly
								value={$mainView.account}
							/>
						</div>
					</div>
					<div class="col">
						<label for="kind" class="form-label">Art</label>
						<input
							id="kind"
							type="text"
							class="form-control"
							placeholder="Art"
							aria-label="Art"
							readonly
							value={$mainView.kind}
						/>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="accuser-select" class="form-label">Anzeiger</label>
						<div class="input-group mb-3 accuser-select">
							<button
								id="accuser-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="accuser-select-dropdown" class="dropdown-menu" />
							<input
								id="accuser"
								type="text"
								class="form-control"
								placeholder="Anzeiger"
								aria-label="Anzeiger"
								readonly
								value={$mainView.accuser}
							/>
						</div>
					</div>
					<div class="col">
						<label for="police-consultant" class="form-label">Sachberater Polizei</label>
						<div class="input-group mb-3 police-consultant-select">
							<button
								id="police-consultant-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="police-consultant-select-dropdown" class="dropdown-menu" />
							<input
								id="police-consultant"
								type="text"
								class="form-control"
								placeholder="Sachberater Polizei"
								aria-label="Sachberater Polizei"
								readonly
								value={$mainView.police_consultant}
							/>
						</div>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="lawyer-culprit" class="form-label">Anwalt des Beschuldigtens</label>
						<div class="input-group mb-3 lawyer-culprit-select">
							<button
								id="lawyer-culprit-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="lawyer-culprit-select-dropdown" class="dropdown-menu" />
							<input
								id="lawyer-culprit"
								type="text"
								class="form-control"
								placeholder="Anwalt des Beschuldigtens"
								aria-label="Anwalt des Beschuldigtens"
								readonly
								value={$mainView.lawyer_culprit}
							/>
						</div>
					</div>
					<div class="col">
						<label for="lawyer-accuser" class="form-label">Anwalt des Anzeigers</label>
						<div class="input-group mb-3 lawyer-accuser-select">
							<button
								id="lawyer-accuser-select-button"
								class="btn btn-outline-danger dropdown-toggle hide-arrow"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled
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
							<ul id="lawyer-accuser-select-dropdown" class="dropdown-menu" />
							<input
								id="lawyer-accuser"
								type="text"
								class="form-control"
								placeholder="Anwalt des Anzeigers"
								aria-label="Anwalt des Anzeigers"
								readonly
								value={$mainView.lawyer_accuser}
							/>
						</div>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="facts" class="form-label">Tatbestand</label>
						<input
							id="facts"
							type="text"
							class="form-control"
							placeholder="Tatbestand"
							aria-label="Tatbestand"
							readonly
							value={$mainView.facts}
						/>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="time-of-crime" class="form-label">Zeitpunkt der Tat</label>
						<input
							id="time-of-crime"
							type="text"
							class="form-control"
							placeholder="Zeitpunkt der Tat"
							aria-label="Zeitpunkt der Tat"
							readonly
							value={$mainView.time_of_crime}
						/>
					</div>
					<div class="col">
						<label for="location-of-crime" class="form-label">Ort der Tat</label>
						<input
							id="location-of-crime"
							type="text"
							class="form-control"
							placeholder="Ort der Tat"
							aria-label="Ort der Tat"
							readonly
							value={$mainView.location_of_crime}
						/>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="note" class="form-label">Kommentar</label>
						<input
							id="note"
							type="text"
							class="form-control"
							placeholder="Kommentar"
							aria-label="Kommentar"
							readonly
							value={$mainView.note}
						/>
					</div>
				</div>
				<div class="row">
					<div class="col">
						<label for="verdict" class="form-label">Urteil</label>
						<div class="input-group mb-3 verdict-select">
							<button
								id="verdict-select-button"
								class="btn btn-outline-danger dropdown-toggle"
								type="button"
								data-bs-toggle="dropdown"
								aria-expanded="false"
								title="Auswählen"
								disabled>Urteil</button
							>
							<ul id="verdict-select-dropdown" class="dropdown-menu">
								<li>
									<button id="no-yet" class="dropdown-item" type="button"
										>a.) Noch kein Verfahren</button
									>
								</li>
								<li>
									<button id="guilty" class="dropdown-item" type="button">b.) Schuldig</button>
								</li>
								<li>
									<button id="innocent" class="dropdown-item" type="button">c.) Unschuldig</button>
								</li>
							</ul>
							<input
								id="verdict"
								type="text"
								class="form-control"
								placeholder="Urteil"
								aria-label="Urteil"
								readonly
								value={$mainView.verdict}
							/>
						</div>
					</div>
				</div>
				<button id="criminal-add-button" type="button" class="btn btn-outline-danger m-3" hidden
					><span
						id="criminal-add-button-spinner"
						class="spinner-border spinner-border-sm"
						role="status"
						aria-hidden="true"
						hidden
					/>Hinzufügen</button
				>
				<button id="criminal-confirm-button" type="button" class="btn btn-outline-danger m-3" hidden
					><span
						id="criminal-confirm-button-spinner"
						class="spinner-border spinner-border-sm"
						role="status"
						aria-hidden="true"
						hidden
					/>Bestätigen</button
				>
				<button id="criminal-abort-button" type="button" class="btn btn-outline-danger m-3" hidden
					>Abbrechen</button
				>
				<button
					type="button"
					class="btn btn-outline-danger m-3 get-user"
					style="max-width: 160px;"
					on:click={() => getUser()}>Bürger abrufen</button
				>
			</div>
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'login'}
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
							<label for="login-add-workless-permissions" class="form-label"
								>Rechte für Arbeitslose</label
							>
							<select
								id="login-add-workless-permissions"
								class="form-select"
								aria-label="Permissions"
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
							>
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
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'password'}
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
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'stats'}
			<div id="stats-container">
				<div class="row p-3">
					<div class="col-sm-6 mb-3 mb-sm-0">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Name</h5>
								<p class="card-text" id="name">{$mainView.name}</p>
							</div>
						</div>
					</div>
					<div class="col-sm-6">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Version</h5>
								<p class="card-text" id="version">{$mainView.version}</p>
							</div>
						</div>
					</div>
				</div>
				<div class="row p-3">
					<div class="col-sm-6 mb-3 mb-sm-0">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Entwickler</h5>
								<p class="card-text" id="devs">{$mainView.developers}</p>
							</div>
						</div>
					</div>
					<div class="col-sm-6">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Repository</h5>
								<p class="card-text">
									<a target="_blank" id="repo" href={$mainView.repo}>{$mainView.repo}</a>
								</p>
							</div>
						</div>
					</div>
				</div>
				<div class="row p-3">
					<div class="col-sm-6 mb-3 mb-sm-0">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Beschreibung</h5>
								<p class="card-text" id="description">{$mainView.description}</p>
							</div>
						</div>
					</div>
					<div class="col-sm-6">
						<div class="card">
							<div class="card-body">
								<h5 class="card-title">Bürger insgesammt</h5>
								<p class="card-text" id="users">{$mainView.users}</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
	<Dialog bind:this={newDialog} />
</section>

<style>
	.main {
		display: grid;
		grid-template:
			'nav nav' 60px
			'sidebar mid' calc(100% - 60px) / 350px auto;
		height: 100%;
	}

	@media only screen and (max-width: 768px) {
		.main {
			grid-template:
				'nav' 60px
				'sidebar' 250px
				'mid' auto / auto;
		}
	}

	.sidebar {
		grid-area: sidebar;
		display: flex;
		flex-direction: column;
	}

	.hide-arrow::after {
		display: none !important;
	}

	.p-2 {
		padding-left: 15px !important;
	}

	.sidebar-search {
		flex: 0;
	}

	.mid {
		grid-area: mid;
	}
</style>
