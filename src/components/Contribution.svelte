<script>
	import {
		Select,
        SelectItem ,
	} from "carbon-components-svelte";

    export let labelText;
    export let selected;
    
    let options =   [{
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


<Select
    labelText={labelText}
    bind:selected={selected}
>
    {#each options as { value, text, description }}
        <SelectItem value={value} text={text} />
    {/each}
</Select>
	