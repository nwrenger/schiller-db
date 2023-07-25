<script lang="ts">
	let title: string;
	let message: string;
	let dialog: HTMLDialogElement;

	export function open(tit: string, msg: string) {
		title = tit;
		message = msg;
		if (!dialog.attributes.getNamedItem('open')) {
			dialog.showModal();
		}
	}
</script>

<dialog class="custom-dialog" bind:this={dialog} on:close>
	<div class="card">
		<div class="card-header">{title}</div>
		<div class="card-body">
			<div class="card-text">{message}</div>
			<button type="button" class="btn btn-secondary mt-3" on:click={() => dialog.close()}
				>Schließen</button
			>
		</div>
	</div>
</dialog>

<style>
	.custom-dialog {
		padding: 0px;
		height: fit-content;
		width: 20rem;
		border: none;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.4);
	}
</style>
