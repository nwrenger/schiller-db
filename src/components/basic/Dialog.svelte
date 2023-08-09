<script lang="ts">
	let title: string;
	let message: string;
	let dialog: HTMLDialogElement;
	let response: Promise<any>;
	export var fun: (() => Promise<any>) | undefined;

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
			{#if fun}
				<button
					type="button"
					class="btn btn-danger mt-3 ok"
					on:click={async () => {
						if (fun) response = fun();
						response.then(() => dialog.close());
					}}
				>
					{#await response}
						<span
							id="add-login-button-spinner"
							class="spinner-border spinner-border-sm"
							role="status"
							aria-hidden="true"
						/>
					{/await}
					Ja</button
				>
				<button type="button" class="btn btn-secondary mt-3" on:click={() => dialog.close()}
					>Abbrechen</button
				>
			{:else}
				<button type="button" class="btn btn-secondary mt-3" on:click={() => dialog.close()}
					>Schließen</button
				>
			{/if}
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
