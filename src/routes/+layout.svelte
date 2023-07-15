<script lang="ts">
	import { onMount } from 'svelte';
	import { dialog, staticBackdropLabel, modalBody } from './store';
	
	let dialogElement: HTMLDivElement;
	let staticBackdropLabelElement: HTMLHeadingElement;
	let modalBodyElement: HTMLDivElement;

	onMount(() => {
		dialog.set(dialogElement);
		staticBackdropLabel.set(staticBackdropLabelElement);
		modalBody.set(modalBodyElement);
	});
</script>

<div class="app">
	<!-- Main -->
	<main style="height: 100%;">
		<slot />
	</main>
	<!-- Modal -->
	<div
		class="modal fade"
		bind:this={dialogElement}
		data-bs-backdrop="static"
		data-bs-keyboard="false"
		tabindex="-1"
		aria-labelledby="staticBackdropLabel"
		aria-hidden="true"
	>
		<div class="modal-dialog modal-dialog-centered modal-dialog-scrollable">
			<div class="modal-content">
				<div class="modal-header">
					<!-- svelte-ignore a11y-missing-content -->
					<h1 class="modal-title fs-5" bind:this={staticBackdropLabelElement} />
					<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
				</div>
				<div class="modal-body" bind:this={modalBodyElement} />
				<div class="modal-footer">
					<button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Schließen</button>
				</div>
			</div>
		</div>
	</div>
</div>
