<script lang="ts">
    export let title: string = "Fehler!";
    
    let message: string;
    let dialog: HTMLDialogElement;

    export function open(msg: string) {
        message = msg;
        dialog.showModal();
    }
</script>

<dialog class="custom-dialog" bind:this={dialog} on:close>
    <div class="card">
        <div class="card-header">{title}</div>
        <div class="card-body">
            <div class="card-text">{message}</div>
            <button type="button" class="btn btn-secondary mt-3" on:click={() => dialog.close()}>Schließen</button>
        </div>
    </div>
</dialog>

<style>
    .custom-dialog {
        padding: 0px;
        height: fit-content;
        width: 18rem;
        border: none;
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, 0.4);
    }
</style>
