<script lang="ts">
	/// Imports
	import { writable, type Writable } from 'svelte/store';
	import { goto } from '$app/navigation';
	import Navigation from './Navigation.svelte';
	import NestedList from './NestedList.svelte';
	import UserView from './UserView.svelte';
	import WorklessView from './WorklessView.svelte';
	import LoginView from './LoginView.svelte';
	import CriminalView from './CriminalView.svelte';
	import PasswordView from './PasswordView.svelte';
	import StatsView from './StatsView.svelte';

	import type { User } from './UserView.svelte';
	import type { Workless } from './WorklessView.svelte';
	import type { Criminal } from './CriminalView.svelte';
	import type { Login } from './LoginView.svelte';
	import type { Password } from './PasswordView.svelte';
	import type { Stats } from './StatsView.svelte';

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
	import SearchList from './SearchList.svelte';
	import SidebarSearch from './SidebarSearch.svelte';
	let newDialog: Dialog;

	function error(error: string) {
		newDialog.open(error);
		throw error;
	}

	/// Stats
	async function stats() {
		if (!($mainView && typeof $mainView == 'object' && $mainView.ty == 'stats')) {
			const statsData = await request('/api/stats', 'GET', null);
			const devs = statsData.developer.split(':');

			$mainView = {
				ty: 'stats',
				name: statsData.name,
				version: statsData.version,
				developers: devs,
				repo: statsData.repo,
				description: statsData.description,
				users: statsData.users
			};
		}
	}

	/// NestedList
	let nestedList: NestedList<ListItem>;

	function onNestedListSelect(parents: ListItem[]): boolean {
		// console.log(`List Parents: ${parents.length}`);
		if ($mainView && typeof $mainView == 'object' && Array.isArray(parents)) {
			if (parents.length == 1) return true;
			const data = parents[1] as User | Workless | Criminal;
			$mainView = { ...data, ty: $sidebarState } as User | Workless | Criminal;
		}
		return false;
	}

	async function fetchNestedListItems(parents: ListItem[]): Promise<ListItem[]> {
		// console.log(`Fetch Parents: ${parents.at(-1)}`);
		if (parents && Array.isArray(parents)) {
			if ($sidebarState === 'user') {
				searchRole = parents.at(-1)?.toString() as string;
			} else if ($sidebarState === 'workless') {
				searchDate = parents.at(-1)?.toString() as string;
			}
		}
		return await nestedListData(parents.at(-1) ?? null);
	}

	async function nestedListData(nested: ListItem | null) {
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

	// temporarily setting this to true
	var nested: boolean = true;

	/// Search
	async function search(
		params: string,
		kind: string | null,
		role: string | null,
		date: string | null,
		limit: number | null
	) {
		var data: User[] | Workless[] | Criminal[] = [];
		if (role) {
			if (kind === 'user') {
				data = await request(
					`/api/user/search?name=${encodeURIComponent(params)}&role=${encodeURIComponent(
						role
					)}&limit=${limit}`,
					'GET',
					null
				);
			} else if (kind === 'workless') {
				if (date) {
					data = await request(
						`/api/workless/search_role?name=${encodeURIComponent(params)}&role=${encodeURIComponent(
							role
						)}&date=${encodeURIComponent(date)}&limit=${limit}`,
						'GET',
						null
					);
				} else {
					data = await request(
						`/api/workless/search_role?name=${encodeURIComponent(params)}&role=${encodeURIComponent(
							role
						)}&limit=${limit}`,
						'GET',
						null
					);
				}
			} else if (kind === 'criminal') {
				data = await request(
					`/api/criminal/search_role?name=${encodeURIComponent(params)}&role=${encodeURIComponent(
						role
					)}&limit=${limit}`,
					'GET',
					null
				);
			}
		} else {
			if (kind === 'user') {
				data = await request(
					`/api/user/search?name=${encodeURIComponent(params)}&limit=${limit}`,
					'GET',
					null
				);
			} else if (kind === 'workless') {
				if (date) {
					data = await request(
						`/api/workless/search?name=${encodeURIComponent(params)}&date=${encodeURIComponent(
							date
						)}&limit=${limit}`,
						'GET',
						null
					);
				} else {
					data = await request(
						`/api/workless/search?name=${encodeURIComponent(params)}&limit=${limit}`,
						'GET',
						null
					);
				}
			} else if (kind === 'criminal') {
				data = await request(
					`/api/criminal/search?name=${encodeURIComponent(params)}&limit=${limit}`,
					'GET',
					null
				);
			}
		}
		return data;
	}

	/// Search List
	let searchList: SearchList<User | Workless | Criminal>;

	var searchParams: string = '';
	var searchRole: string | null = null;
	var searchDate: string | null = null;

	async function fetchSearchListItems(
		params: string,
		role: string | null,
		date: string | null
	): Promise<(User | Workless | Criminal)[]> {
		// console.log(`Fetch Search: ${params}, ${role}`);
		return await search(params, $sidebarState, role, date, null);
	}

	function onSearchListSelect(item: User | Workless | Criminal | null) {
		// console.log(`List Item: ${JSON.stringify(item)}`);
		if ($mainView && typeof $mainView == 'object' && item) {
			$mainView = { ...item, ty: $sidebarState } as User | Workless | Criminal;
		}
	}

	/// Advanced Search
	async function selectData(params: string, date: string | null): Promise<[]> {
		var data = [];
		if ($sidebarState === 'user') {
			data = await request(`/api/user/all_roles?name=${encodeURIComponent(params)}`, 'GET', null);
		} else if ($sidebarState === 'workless') {
			if (date) {
				data = await request(
					`/api/workless/all_roles?name=${encodeURIComponent(params)}&date=${encodeURIComponent(
						date
					)}`,
					'GET',
					null
				);
			} else {
				data = await request(
					`/api/workless/all_roles?name=${encodeURIComponent(params)}`,
					'GET',
					null
				);
			}
		} else if ($sidebarState === 'criminal') {
			data = await request(
				`/api/criminal/all_roles?name=${encodeURIComponent(params)}`,
				'GET',
				null
			);
		}
		return data;
	}

	async function fetchRoleSelectItems(params: string, date: string | null) {
		// console.log(`Fetch Role Select: ${params}, ${date}`);
		return await selectData(params, date);
	}

	/// Sidebar List
	type ListItem = User | Workless | Criminal | string;

	let mainView: Writable<ListItem | Login | Password | Stats | null> = writable(null);
	let sidebarState: Writable<string | null> = writable('user');

	$: console.log($mainView);

	$: if ($mainView && typeof $mainView == 'object')
		if ($mainView.ty == 'stats' || $mainView.ty == 'login' || $mainView.ty == 'password')
			if (nested && nestedList) {
				nestedList.deselectAll();
			} else if (searchList) {
				searchList.deselectAll();
			}

	$: if ($mainView && typeof $mainView == 'object')
		if ($mainView.ty == 'user' || $mainView.ty == 'workless' || $mainView.ty == 'criminal') {
			console.log('edit buttons');
		} else {
			console.log('no edit buttons');
		}

	sidebarState.subscribe(() => {
		searchParams = '';
		searchRole = null;
		if (nested && nestedList) {
			nestedList.reset();
		} else if (searchList) {
			nested = true;
		}
		stats();
	});

	function formatDate(date: string) {
		const [year, month, day] = JSON.parse(date).split('-');
		return `${day}.${month}.${year}`;
	}

	/// Other
	async function getUser() {
		if (
			$mainView &&
			typeof $mainView == 'object' &&
			($mainView.ty == 'user' || $mainView.ty == 'workless' || $mainView.ty == 'criminal')
		) {
			const current = $mainView.account;
			var data: User = await request(`/api/user/fetch/${encodeURIComponent(current)}`, 'GET', null);
			$mainView = { ...data, ty: 'user' } as User;
			if (nested && nestedList) {
				nestedList.deselectAll();
			} else if (searchList) {
				searchList.deselectAll();
			}
		}
	}
</script>

<svelte:head>
	<title>SchillerDB</title>
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
		<ul class="sidebar-list list-group list-group-flush" id="sidebar-list">
			{#if nested}
				<NestedList
					bind:this={nestedList}
					fetchItems={fetchNestedListItems}
					onSelect={onNestedListSelect}
					{stats}
				/>
			{:else}
				<SearchList
					bind:this={searchList}
					fetchItems={fetchSearchListItems}
					onSelect={onSearchListSelect}
					{stats}
					bind:params={searchParams}
					bind:role={searchRole}
					date={searchDate}
					bind:nested
				/>
			{/if}
		</ul>
		<SidebarSearch
			bind:params={searchParams}
			bind:role={searchRole}
			date={searchDate}
			bind:nested
			{sidebarState}
			{stats}
			{fetchRoleSelectItems}
		/>
	</div>
	<!-- Input Containers -->
	<div class="mid p-3 bg-body-secondary">
		{#if $mainView && typeof $mainView == 'object' && $mainView.ty == 'user'}
			<UserView user={$mainView} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'workless'}
			<WorklessView workless={$mainView} {getUser} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal'}
			<CriminalView criminal={$mainView} {getUser} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'login'}
			<LoginView {stats} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'password'}
			<PasswordView {stats} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'stats'}
			<StatsView stats={$mainView} />
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

	.sidebar-list {
		flex: 1;
		overflow-y: scroll;
	}

	.p-2 {
		padding-left: 15px !important;
	}

	.mid {
		grid-area: mid;
	}
</style>
