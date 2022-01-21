<script lang="ts">
    import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
    
    import Textfield from '@smui/textfield';
    
    import AddAlt from '../icons/AddAlt.svelte';
    import SubtractAlt from '../icons/SubtractAlt.svelte';

    export let label: string;
    export let data: { year: number; amount: number; }; // array of arrays of data
    let inputYear: number = 0;
    let inputValue: number = 0;
    
    // Parse the object version of input back into a string for display
    function handleAdd() {
        data[inputYear] = inputValue
        bind:data = data;
    }

    function handleRemove(year:string) {
        delete data[year];
        bind:data = data;
    }
</script>

<h1>{label}</h1>
<DataTable table$aria-label={label} style="margin-bottom:10px;">
    <Head>
      <Row>
        <Cell>Year</Cell>
        <Cell>Amount</Cell>
        <Cell> </Cell>
      </Row>
    </Head>
    <Body>
        {#each Object.keys(data).sort() as thisYear}
            <Row>
                <Cell>{thisYear}</Cell>
                <Cell>{data[thisYear]}</Cell>
                <Cell>
                    <div on:click={() => handleRemove(thisYear)}>
                        <SubtractAlt />
                    </div>
                </Cell>
            </Row>
        {/each}

        <Row>
            <Cell>
                <Textfield
                    label="Year"
                    type="number"
                    input$step=0.01
                    bind:value={inputYear}
                />    
            </Cell>
            <Cell>
                <Textfield
                    label="Value"
                    type="number"
                    input$step=0.01
                    bind:value={inputValue}
                />   
            </Cell>
            <Cell>
                <div on:click={handleAdd}>
                    <AddAlt />
                </div>
            </Cell>
        </Row>
    </Body>

</DataTable>

<style>
    h1 {
        font-family: "Arial","sans-serif";
        font-size: 14px;
        font-weight: 400;
        color: rgba(255,255,255,0.6);
    }
</style>