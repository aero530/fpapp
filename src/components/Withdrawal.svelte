<script lang="ts">
    import Select, { Option } from '@smui/select';
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;
    
    type WithdrawalOption = { id: number; value: string; label: string; description: string };
    let options : WithdrawalOption[] = [
    {
        id: 1,
        value: 'fixed',
        label: 'fixed',
        description: 'Take out a fixed dollar amount',
    },
    {
        id: 2,
        value: 'fixed_with_inflation',
        label: 'fixed with inflation',
        description: 'Take out a fixed dollar amount with inflation compensation',
    },
    {
        id: 3,
        value: 'end_at_zero',
        label: 'end at zero',
        description:
        'take money out in equal amounts each year such that the balance at endOut is zero',
    },
    {
        id: 4,
        value: 'col_frac_of_savings',
        label: 'cost of living fraction of total savings',
        description:
        'Take out the current cost of living * (this accounts value / total savings)',
    },
    {
        id: 5,
        value: 'percent_of_income',
        label: 'percent of income',
        description: 'Take out a percent of income in each year',
    }];




</script>


<QuestionField>
    <span slot="input">
        <Select
        label={label}
        bind:value={value}
    >
        {#each options as option (option.label)}
            <Option value={option.value}>{option.label}</Option>
        {/each}
    </Select>
    </span>
    <span slot="questionTip">
        Select how money is taken out of the account.
        {#each options as option (option.label)}
            <p>{option.label} : {option.description}</p>
        {/each}
    </span>
</QuestionField>
