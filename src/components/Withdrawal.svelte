<script lang="ts">
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;
    
    type WithdrawalOption = { value: string; label: string; description: string };
    let options : WithdrawalOption[] = [
    {
        value: 'fixed',
        label: 'Fixed',
        description: 'Take out a fixed dollar amount',
    },
    {
        value: 'fixed_with_inflation',
        label: 'Fixed with inflation',
        description: 'Take out a fixed dollar amount with inflation compensation',
    },
    {
        value: 'end_at_zero',
        label: 'End at zero',
        description:
        'take money out in equal amounts each year such that the balance at endOut is zero',
    },
    {
        value: 'col_frac_of_savings',
        label: 'Cost of living',
        description:
        'Take out the current cost of living * (this accounts value / total savings)',
    },
    {
        value: 'percent_of_income',
        label: 'Percent of income',
        description: 'Take out a percent of income in each year',
    }];


    let questionText = 'Select how money is taken out of the account.';

    options.forEach((option) => {
        questionText += `${option.label} : ${option.description}`;
    });

</script>


<QuestionField {questionText} {label}>
    <select
        bind:value={value}
        class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    >
        {#each options as option (option.label)}
            <option value={option.value}>{option.label}</option>
        {/each}
    </select>
</QuestionField>
