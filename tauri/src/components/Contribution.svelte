<script lang="ts">
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;

    type ContributionOption = { value: string; label: string; description: string };
    let options : ContributionOption[] = [{
        value: 'fixed',
        label: 'fixed',
        description: 'fixed dollar amount',
    },
    {
        value: 'percent_of_income',
        label: 'percent of income',
        description: 'percent of cost of current living',
    },
    {
        value: 'fixed_with_inflation',
        label: 'fixed with inflation',
        description:
        'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
    }];

</script>

<QuestionField {label}>
    <select
        bind:value={value}
        class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    >
        {#each options as option (option.label)}
            <option value={option.value}>{option.label}</option>
        {/each}
    </select>
    <div slot="helper">
        <b>Select how money is put into the account.</b>
        <ul>
            {#each options as option}
                <li>{option.label} : {option.description}</li>
            {/each}
        </ul>
    </div>
</QuestionField>
