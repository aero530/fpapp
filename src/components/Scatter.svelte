<script lang="ts">
	import Line from "./Charts/Line.svelte"; 
	import type { Point, PointArray, PointLimit } from "../components/Charts/chart.type";

	import { plot_data } from '../stores.js';
	import {dark} from '../stores.js';
	
	export let id : string;
	export let title : string;
	export let xLabel : string;
	export let yLabel : string;

	let zeroBase = true;

    let domain: PointLimit<Date, number> = {
        x: {min:null, max:null},
        y: {
            min: zeroBase ? 0 : null,
            max:null
        },
    }
</script>

{#if $plot_data.hasOwnProperty(id)}
	<Line 
		data={$plot_data[id]}
		{title}
		{xLabel}
		{yLabel}
		{domain}
	/>
{/if}