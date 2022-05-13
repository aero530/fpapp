<script lang="ts">
import { onMount } from 'svelte';

    import QuestionField from './QuestionField.svelte'

    export let label : string;
    export let value : number;
    export let questionText : string="";
    export let step : number = 1;
    let inputValue : number = 1;

    onMount(() => {
        inputValue = value;
    })

    function handleUpdate() {
        bind:value = round(inputValue, step);
    }

    function round(number: number, increment: number) {
        return Math.ceil((number) / increment ) * increment;
    }
</script>

<QuestionField {label}>
    <input
        type="number"
        bind:value={inputValue}
        class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
        on:change={() => handleUpdate()}
    />
    <div slot="helper">{questionText}</div>
</QuestionField>