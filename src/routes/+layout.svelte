<div class="app">
	<!-- Main -->
	<main style="height: 100%;">
		<slot />
	</main>
	<!-- Modal -->
	<div
		class="modal fade"
		id="dialog"
		data-bs-backdrop="static"
		data-bs-keyboard="false"
		tabindex="-1"
		aria-labelledby="staticBackdropLabel"
		aria-hidden="true"
	>
		<div class="modal-dialog modal-dialog-centered modal-dialog-scrollable">
			<div class="modal-content">
				<div class="modal-header">
					<h1 class="modal-title fs-5" id="staticBackdropLabel"> </h1>
					<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
				</div>
				<div class="modal-body" id="modal-body" />
				<div class="modal-footer">
					<button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Schließen</button>
				</div>
			</div>
		</div>
	</div>
</div>
