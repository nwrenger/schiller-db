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
</div>
