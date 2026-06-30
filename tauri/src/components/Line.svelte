<script lang="ts">
	import Line from "./Charts/Line.svelte"; 
	import { plot_data } from '../stores.js';
	
	export let id : string;
	export let title : string;
	export let xLabel : string;
	export let yLabel : string;
	export let displayLegend: boolean = false;

	let zeroBase = true;

	let data;

    $: data = $plot_data.hasOwnProperty(id) ? $plot_data[id] : [];

	// Force domain to be reactive so it is recalculated when 
	// the data in the chart gets updated
	$: domain = $plot_data.hasOwnProperty(id) ? {
		x: {min:null, max:null},
        y: {min: zeroBase ? 0 : null, max:null}
	} : {
		x: {min:null, max:null},
        y: {min: zeroBase ? 0 : null, max:null}
	};

</script>

{#if $plot_data.hasOwnProperty(id)}
	<Line 
		{data}
		{title}
		{xLabel}
		{yLabel}
		{domain}
		{displayLegend}
	/>
{/if}