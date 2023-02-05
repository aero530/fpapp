<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import Line from "../components/Charts/Line.svelte";
	import { summary_data } from '../stores';

	import type { PointLimit } from "../components/Charts/chart.type";

    export let zeroBase: boolean=true;

	onMount(() => {
	    invoke('my_custom_command');
	});

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

	{#if Object.keys($summary_data.col).length == 0 }
		<div class="grid grid-cols-3 gap-4 place-content-center h-48">
			<div></div>
			<div><h2 class="text-lg text-center">to begin load data from file or configure settings & add accounts</h2></div>
			<div></div>
		</div>
	{:else}
	<div class="grid grid-cols-1 gap-1 mx-32">
		<Line 
			data={[
				{label:"Cost of Living", data:toPoints($summary_data.col)},
				{label:"Expenses", data:toPoints($summary_data.expense)},
				{label:"Income", data:toPoints($summary_data.income)},
				{label:"Net", data:toPoints($summary_data.net)},
			]}
			title="Overview"
			xLabel="Year"
			yLabel="$"
			displayLegend={true}
		/>

		<Line 
			data={toPoints($summary_data.saving)}
			title="Savings"
			xLabel="Year"
			yLabel="$"
		/>

		<Line 
			data={toPoints($summary_data.col)}
			title="Cost of Living"
			xLabel="Year"
			yLabel="$"
			{domain}
		/>
	</div>
	{/if}