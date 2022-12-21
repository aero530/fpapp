<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import Line from "../components/Charts/Line.svelte";
	import { summary_data } from '../stores.js';

	import type { PointArray, PointLimit } from "../components/Charts/chart.type";

    export let zeroBase: boolean=true;

	onMount(() => {
	    invoke('my_custom_command');
	});

	let data: PointArray<number, number>;
	data = [{x:0,y:0},{x:1,y:3},{x:2,y:4}]

    let domain: PointLimit<Date, number> = {
        x: {min:null, max:null},
        y: {
            min: zeroBase ? 0 : null,
            max:null
        },
    }

	function toPoints(data) {
		let arr = [];
		Object.entries(data).forEach(entry => {
			const [key, value] = entry;
			arr.push({ x:parseInt(key), y:value })
		});
		return arr;
	}

</script>

<h1 class="text-lg">Dashboard</h1>


<!-- {#if Object.keys($summary_data.income).length !== 0 }
<Line 
	data={toPoints($summary_data.income)}
	title="Income"
	xLabel="Year"
	yLabel="i"
	{domain}
/>
{/if}

<!-- {#if Object.keys($summary_data.saving).length !== 0 }
<Line 
	data={toPoints($summary_data.saving)}
	title="Savings"
	xLabel="Year"
	yLabel="s"
	{domain}
/>
{/if} -->

{#if Object.keys($summary_data.col).length !== 0 }
<Line 
	data={toPoints($summary_data.col)}
	title="Cost of Living"
	xLabel="Year"
	yLabel="c"
	{domain}
/>
{/if}