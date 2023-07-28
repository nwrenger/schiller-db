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
	interface Permissions {
		access_user: string;
		access_workless: string;
		access_criminal: string;
	}

	var auth = localStorage.getItem('auth');
	const current_user = localStorage.getItem('current_user');
	var permissions: Permissions | string | null = localStorage.getItem('permissions');
	if (permissions) {
		permissions = JSON.parse(permissions) as Permissions;
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

	/// Back = Default + Stats
	async function back() {
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
		editable = false;
		isNew = false;
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
				searchRole = parents.at(-1) as string;
			} else if ($sidebarState === 'workless') {
				searchDate = parents.at(-1) as string;
			} else if ($sidebarState === 'criminal') {
				searchAccount = parents.at(-1) as string;
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
					`/api/criminal/search?account=${encodeURIComponent(nested)}`,
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
	var searchAccount: string | null = null;

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
	let onHighlighted: boolean = false;

	let editable: boolean = false;
	let isNew: boolean = false;
	let userView: UserView;
	let worklessView: WorklessView;
	let criminalView: CriminalView;

	$: if (
		isNew &&
		$mainView &&
		typeof $mainView == 'object' &&
		$sidebarState &&
		$mainView.ty !== $sidebarState
	) {
		$mainView = {} as User | Workless | Criminal | Login | Password | Stats;
		$mainView.ty = $sidebarState as
			| 'login'
			| 'password'
			| 'user'
			| 'workless'
			| 'criminal'
			| 'stats';
	}

	/// Sidebar List
	type ListItem = User | Workless | Criminal | string;

	let mainView: Writable<ListItem | Login | Password | Stats | null> = writable(null);
	let sidebarState: Writable<string | null> = writable('user');

	$: console.log($mainView);

	$: if ($mainView && typeof $mainView == 'object')
		if ($mainView.ty == 'stats' || $mainView.ty == 'login' || $mainView.ty == 'password')
			deselect();

	sidebarState.subscribe(() => {
		searchParams = '';
		searchRole = null;
		back();
		if (nestedList) {
			nestedList.reset();
		} else if (searchList) {
			nested = true;
		}
	});

	function deselect() {
		if (nestedList) {
			nestedList.deselectAll();
		} else if (searchList) {
			searchList.deselectAll();
		}
	}

	function reload() {
		if (nestedList) {
			nestedList.reload();
		} else if (searchList) {
			searchList.reload();
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
		onSelect={async (val) => {
			if (val == 'password' || val == 'login') {
				$mainView = { ty: val };
			} else {
				await back();
			}
		}}
		accessUser={typeof permissions == 'object' ? permissions?.access_user : null}
		currentUser={current_user}
	/>
	<!-- Sidebar -->
	<div class="sidebar bg-dark">
		<ChangeButtons
			del={() =>
				userView ? userView.del() : worklessView ? worklessView.del() : criminalView.del()}
			{onHighlighted}
			{back}
			access={typeof permissions == 'object'
				? $sidebarState === 'user'
					? permissions?.access_user
					: $sidebarState === 'workless'
					? permissions?.access_workless
					: permissions?.access_criminal
				: null}
			bind:editable
			bind:isNew
		/>
		<ul class="sidebar-list list-group list-group-flush" id="sidebar-list">
			{#if nested}
				<NestedList
					bind:this={nestedList}
					fetchItems={fetchNestedListItems}
					onSelect={onNestedListSelect}
					{back}
					bind:onHighlighted
					currentEntry={($mainView && typeof $mainView == 'object' && $mainView.ty == 'user') ||
					($mainView && typeof $mainView == 'object' && $mainView.ty == 'workless') ||
					($mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal')
						? $mainView
						: null}
					state={$sidebarState}
				/>
			{:else}
				<SearchList
					bind:this={searchList}
					fetchItems={fetchSearchListItems}
					onSelect={onSearchListSelect}
					{back}
					bind:params={searchParams}
					bind:onHighlighted
					bind:role={searchRole}
					bind:date={searchDate}
					bind:nested
					currentEntry={($mainView && typeof $mainView == 'object' && $mainView.ty == 'user') ||
					($mainView && typeof $mainView == 'object' && $mainView.ty == 'workless') ||
					($mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal')
						? $mainView
						: null}
				/>
			{/if}
		</ul>
		<SidebarSearch
			bind:params={searchParams}
			bind:role={searchRole}
			bind:date={searchDate}
			bind:nested
			{sidebarState}
			accessUser={typeof permissions == 'object' ? permissions?.access_user : null}
			accessWorkless={typeof permissions == 'object' ? permissions?.access_workless : null}
			accessCriminal={typeof permissions == 'object' ? permissions?.access_criminal : null}
			{back}
			{fetchRoleSelectItems}
		/>
	</div>
	<!-- View Containers -->
	<div class="mid p-3 bg-body-secondary">
		{#if $mainView && typeof $mainView == 'object' && $mainView.ty == 'user'}
			<UserView
				bind:this={userView}
				bind:user={$mainView}
				bind:editable
				bind:isNew
				{onHighlighted}
				{request}
				{back}
				{reload}
				{searchRole}
			/>
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'workless'}
			<WorklessView
				bind:this={worklessView}
				bind:workless={$mainView}
				{getUser}
				{search}
				bind:editable
				bind:isNew
				{onHighlighted}
				{back}
				{reload}
				{request}
				{searchDate}
			/>
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'criminal'}
			<CriminalView
				bind:this={criminalView}
				bind:criminal={$mainView}
				{getUser}
				{search}
				bind:editable
				bind:isNew
				{onHighlighted}
				{back}
				{reload}
				{request}
				{searchAccount}
			/>
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'login'}
			<LoginView {request} {back} {search} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'password'}
			<PasswordView {request} {error} {info} {back} bind:auth {current_user} />
		{:else if $mainView && typeof $mainView == 'object' && $mainView.ty == 'stats'}
			<StatsView stats={$mainView} />
		{/if}
	</div>
	<Dialog bind:this={newDialog} fun={undefined} />
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
