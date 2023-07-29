<script lang="ts">
	import Dialog from './Dialog.svelte';

	export let onHighlighted: boolean;
	export let editable: boolean;
	export let isNew: boolean;
	export let access: string | null = null;
	export var back: () => Promise<void>;
	export var del: () => Promise<void>;

	let dialog: Dialog;
</script>

<div class="bg-dark-subtle">
	<Dialog bind:this={dialog} fun={del} />
	<div class="btn-group p-2">
		<button
			id="add"
			class="btn btn-outline-primary {editable && isNew ? 'active' : ''}"
			type="button"
			aria-expanded="false"
			title="Hinzufügen"
			disabled={access === 'Write' ? false : true}
			on:click={() => {
				editable = true;
				isNew = true;
			}}
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
			class="btn btn-outline-primary {editable && !isNew ? 'active' : ''}"
			type="button"
			aria-expanded="false"
			title="Bearbeiten"
			disabled={access === 'Write' ? false : true}
			hidden={!onHighlighted}
			on:click={() => {
				editable = true;
				isNew = false;
			}}
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
			disabled={access === 'Write' ? false : true}
			hidden={!onHighlighted}
			on:click={() => {
				if (dialog) dialog.open('Warnung', 'Eintrag unwiederruflich löschen?');
			}}
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
			class="btn btn-outline-secondary"
			type="button"
			aria-expanded="false"
			title="Schließen"
			hidden={!onHighlighted}
			on:click={async () => await back()}
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

<style>
	.p-2 {
		padding-left: 15px !important;
	}
</style>
