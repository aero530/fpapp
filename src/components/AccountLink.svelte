<script>
	import {v4 as uuidv4 } from 'uuid';

    import QuestionField from './QuestionField.svelte'

    export let label="";            // label for the input field
    export let value=false;               // if this account is linked to another account
    export let account='';          // selected account id
    export let questionText="";     // help text
    export let accounts={};         // accounts object used to populated selection box
    export let accountTypeFilter='';// filter on a single account type

    $: options = Object.keys(accounts)
        .filter(id => accountTypeFilter ? accounts[id].type==accountTypeFilter : true)
        .map((id) => {return {value: id, label: accounts[id].name}})

	let id = uuidv4();

</script>

<QuestionField {questionText} {label}>
    <div class="flex items-center w-full gap-4">
        <label for={id} class="flex items-center cursor-pointer">
            <!-- toggle -->
            <div class="relative">
                <!-- input -->
                <input id={id} type="checkbox" class="sr-only" bind:checked={value}/>
                <!-- line -->
                <div class="w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner"></div>
                <!-- dot -->
                <div class="dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition"></div>
            </div>
        </label>
    
        {#if value}
            <select bind:value={account} class="p-0 m-0 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">
                {#each options as option (option.label)}
                    <option value={option.value}>{option.label}</option>
                {/each}
            </select>
        {/if}
    </div>
</QuestionField>

<style>
    input:checked ~ .dot {
        transform: translateX(200%);
        @apply bg-primary-500;
    }
</style>