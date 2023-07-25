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
	import Dialog from './Dialog.svelte';
	import SearchList from './SearchList.svelte';
	import SidebarSearch from './SidebarSearch.svelte';
	import ChangeButtons from './ChangeButtons.svelte';

	import type { User } from './UserView.svelte';
	import type { Workless } from './WorklessView.svelte';
	import type { Criminal } from './CriminalView.svelte';
	import type { Login } from './LoginView.svelte';
	import type { Password } from './PasswordView.svelte';
	import type { Stats } from './StatsView.svelte';

	/// Request Function
	async function request(
		url: string,
		type: string,
		json: BodyInit | null | undefined
	): Promise<any> {
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
	let newDialog: Dialog;

	function error(error: string) {
		newDialog.open('Fehler', error);
		throw error;
	}

	function info(info: string) {
		newDialog.open('Info', info);
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
	/// Change Buttons
	var onHighlighted: boolean = false;

	/// Sidebar List
	type ListItem = User | Workless | Criminal | string;

	let mainView: Writable<ListItem | Login | Password | Stats | null> = writable(null);
	let sidebarState: Writable<string | null> = writable('user');

	$: console.log($mainView);

	$: if ($mainView && typeof $mainView == 'object')
		if ($mainView.ty == 'stats' || $mainView.ty == 'login' || $mainView.ty == 'password')
			deselect();

	$: if ($mainView && typeof $mainView == 'object')
		if ((nestedList && nestedList.isSelected()) || (searchList && searchList.isSelected())) {
			onHighlighted = true;
		} else {
			onHighlighted = false;
		}

	sidebarState.subscribe(() => {
		searchParams = '';
		searchRole = null;
		if (nestedList) {
			nestedList.reset();
		} else if (searchList) {
			nested = true;
		}
		stats();
	});

	function deselect() {
		if (nestedList) {
			nestedList.deselectAll();
		} else if (searchList) {
			searchList.deselectAll();
		}
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
			deselect();
		}
	}
</script>

<svelte:head>
	<title>SchillerDB</title>
	<meta name="description" content="Main Page" />
</svelte:head>

<section class="main">
	<!-- Header -->
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
		<ChangeButtons {onHighlighted} {stats} />
		<ul class="sidebar-list list-group list-group-flush" id="sidebar-list">
			{#if nested}
				<NestedList
					bind:this={nestedList}
					fetchItems={fetchNestedListItems}
					onSelect={onNestedListSelect}
					{stats}
					state={$sidebarState}
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
	<!-- View Containers -->
	<div class="mid p-3 bg-body-secondary">
		{#if $mainView && typeof $mainView == 'object' && $mainView.ty == 'user'}
			<UserView user={$mainView} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'workless'}
			<WorklessView workless={$mainView} {getUser} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal'}
			<CriminalView criminal={$mainView} {getUser} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'login'}
			<LoginView {request} {stats} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'password'}
			<PasswordView {request} {error} {info} {stats} bind:auth {current_user} />
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

	.mid {
		grid-area: mid;
	}
</style>
