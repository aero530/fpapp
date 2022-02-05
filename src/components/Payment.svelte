<script lang="ts">
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;

    type PaymentOption = { value: string; label: string; description: string };
    let options : PaymentOption[] = [{
        value: 'fixed',
        label: 'fixed',
        description: 'fixed dollar amount',
    },
    {
        value: 'fixed_with_inflation',
        label: 'fixed with inflation',
        description:
        'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
    }];

    let questionText = 'Select how payment value should be interpreted.';

    options.forEach((option) => {
        questionText += `${option.label} : ${option.description}`;
    });

</script>

<QuestionField {questionText} {label}>
    <select
        bind:value={value}
        class="p-0 m-0 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    >
        {#each options as option (option.label)}
            <option value={option.value}>{option.label}</option>
        {/each}
    </select>
</QuestionField>
