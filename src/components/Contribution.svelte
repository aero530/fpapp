<script lang="ts">
    import Select, { Option } from '@smui/select';
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;
    
    type ContributionOption = { id: number; value: string; label: string; description: string };
    let options : ContributionOption[] = [{
        id: 1,
        value: 'fixed',
        label: 'fixed',
        description: 'fixed dollar amount',
    },
    {
        id: 2,
        value: 'percent_of_income',
        label: 'percent of income',
        description: 'percent of cost of current living',
    },
    {
        id: 3,
        value: 'fixed_with_inflation',
        label: 'fixed with inflation',
        description:
        'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
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
        Select how money is put into the account.
        {#each options as option (option.label)}
            <p>{option.label} : {option.description}</p>
        {/each}
    </span>
</QuestionField>
