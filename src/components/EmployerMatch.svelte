<script lang="ts">
	import {v4 as uuidv4 } from 'uuid';

    import QuestionField from './QuestionField.svelte'
    import PercentInput from './PercentInput.svelte'

    type Match = { amount: number; limit: number;} | undefined;

    export let label="";            // label for the input field
    export let questionText="";     // help text
    export let matching : Match = {"amount":0, "limit":0}; // amount 

    let amount = (matching && matching.hasOwnProperty("amount")) ? matching.amount : 0;
    let limit = (matching && matching.hasOwnProperty("limit")) ? matching.limit : 0;
    let enabled = amount>0 || limit>0;

	let id = uuidv4();

    $:matching = enabled ? {
        "amount": amount,
        "limit": limit,
    } : undefined
</script>

<QuestionField {questionText} {label}>
    <div class="p-0 m-0 pl-1 grow">

        <label for={id} class="flex items-center cursor-pointer">
            <!-- toggle -->
            <div class="relative">
                <!-- input -->
                <input id={id} type="checkbox" class="sr-only" bind:checked={enabled}/>
                <!-- line -->
                <div class="w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner"></div>
                <!-- dot -->
                <div class="dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition"></div>
            </div>
        </label>

        {#if enabled}
            <PercentInput
            label="Employer Match"
            bind:value={amount}
            questionText="% of what you put in that the employer matches"
            />
            <PercentInput
            label="Match Limit"
            bind:value={limit}
            questionText="% of what you put in when the employer stops matching"
            />
        {/if}
    </div>
</QuestionField>

<style>
    input:checked ~ .dot {
        transform: translateX(200%);
        @apply bg-primary-500;
    }
</style>