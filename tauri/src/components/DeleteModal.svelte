

<script lang="ts">
	import { form_inputs } from '../stores.js';
	import Modal from "../components/Modal.svelte";
    export let open:boolean=false;
    export let id:string="";
</script>

{#if open}
<Modal title={`Delete Account ${$form_inputs.accounts[id].name}?`} on:close={() => (open = false)}>
    <button 
        class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 dark:bg-primary-300 dark:hover:bg-primary-200"
        on:click={()=>{
            form_inputs.removeAccount(id);
            open=false;
        }}
    >
        Yes
    </button>
    <button 
        class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 dark:bg-primary-300 dark:hover:bg-primary-200"
        on:click={()=>(open=false)}
    >
        No
    </button>
</Modal>
{/if}