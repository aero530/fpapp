<script lang="ts">
    // import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
    
    // import {TextField} from 'smelte';
    
    import AddAlt from '../icons/AddAlt.svelte';
    import SubtractAlt from '../icons/SubtractAlt.svelte';

    export let label: string;
    export let data: { year: number; amount: number; }; // array of arrays of data
    let inputYear: string = '';
    let inputValue: string = '';
    
    // Parse the object version of input back into a string for display
    function handleAdd() {
        data[parseInt(inputYear)] = parseInt(inputValue);
        inputYear = '';
        inputValue = '';
        bind:data = data;
    }

    function handleRemove(year:string) {
        delete data[year];
        bind:data = data;
    }
</script>

<div class="subtitle-1">{label}</div>
<table class="table-auto">
    <thead>
      <tr>
        <th>Year</th>
        <th>Amount</th>
        <th> </th>
      </tr>
    </thead>
    <tbody>
        {#each Object.keys(data).sort() as thisYear}
            <tr>
                <td>{thisYear}</td>
                <td>{data[thisYear]}</td>
                <td>
                    <div on:click={() => handleRemove(thisYear)}>
                        <SubtractAlt />
                    </div>
                </td>
            </tr>
        {/each}

        <tr>
            <td>
                <input
                    label="Year"
                    type="number"
                    bind:value={inputYear}
                />    
            </td>
            <td>
                <input
                    label="Value"
                    type="number"
                    bind:value={inputValue}
                />   
            </td>
            <td>
                <div on:click={handleAdd}>
                    <AddAlt />
                </div>
            </td>
        </tr>
    </tbody>

</table>
