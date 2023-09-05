<script lang="ts" context="module">
	export interface Stats {
		ty: "stats";
		name: string;
		version: string;
		developers: string;
		repo: string;
		description: string;
		users: string;
	}
</script>

<script lang="ts">
	export let stats: Stats;
</script>

<div id="stats-container">
	<div class="row p-3">
		<div class="col-sm-6 mb-3 mb-sm-0">
			<div class="card">
				<div class="card-body">
					<h5 class="card-title">Name</h5>
					<p class="card-text" id="name">{stats.name}</p>
				</div>
			</div>
		</div>
		<div class="col-sm-6">
			<div class="card">
				<div class="card-body">
					<h5 class="card-title">Version</h5>
					<p class="card-text" id="version">{stats.version}</p>
				</div>
			</div>
		</div>
	</div>
	<div class="row p-3">
		<div class="col-sm-6 mb-3 mb-sm-0">
			<div class="card">
				<div class="card-body">
					<h5 class="card-title">Entwickler</h5>
					<p class="card-text" id="devs">{stats.developers}</p>
				</div>
			</div>
		</div>
		<div class="col-sm-6">
			<div class="card">
				<div class="card-body">
					<h5 class="card-title">Repository</h5>
					<p class="card-text">
						<a target="_blank" id="repo" href={stats.repo}>{stats.repo}</a>
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
					<p class="card-text" id="description">{stats.description}</p>
				</div>
			</div>
		</div>
		<div class="col-sm-6">
			<div class="card">
				<div class="card-body">
					<h5 class="card-title">Bürger insgesammt</h5>
					<p class="card-text" id="users">{stats.users}</p>
				</div>
			</div>
		</div>
	</div>
</div>
