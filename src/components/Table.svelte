<script lang="ts">
	import {v4 as uuidv4 } from 'uuid';
	import { createEventDispatcher } from "svelte";

    import type {Table} from "../stores";
    
    import AddAlt from '../icons/AddAlt.svelte';
    import SubtractAlt from '../icons/SubtractAlt.svelte';

    export let label: string;
    export let data: Table; // object of year / amount
    
    let inputYear: string = '';
    let inputValue: string = '';
    
    const dispatch = createEventDispatcher();
	const handleRemove = (year: string) => dispatch('remove',{year});
	const handleAdd = (year: string, value: number) => dispatch('add',{year, value});

    let id = uuidv4();
</script>

<div class="grid grid-col-1">
	<div class="mr-3 text-md">
		<label for="{id}" class="font-medium text-gray-700">{label}</label>
	</div>
	<div class="flex grow items-center mx-4">
        <table class="table-auto">
            <thead>
            <tr>
                <th>Year</th>
                <th>Amount</th>
                <th> </th>
            </tr>
            </thead>
            <tbody>
                {#if data !== null}
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
                {/if}

                <tr>
                    <td>
                        <input
                            type="number"
                            class="p-0 m-0 text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
                            bind:value={inputYear}
                        />    
                    </td>
                    <td>
                        <input
                            type="number"
                            class="p-0 m-0 text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
                            bind:value={inputValue}
                        />   
                    </td>
                    <td>
                        <div on:click={()=>handleAdd(inputYear,parseInt(inputValue))}>
                            <AddAlt />
                        </div>
                    </td>
                </tr>
            </tbody>

        </table>
	</div>
</div>