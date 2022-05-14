<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import Line from "../components/Charts/Line.svelte"; 

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

</script>

<h1 class="text-lg">Dashboard</h1>

<Line 
	{data}
	title="asdf"
	xLabel="xlab"
	yLabel="ylab"
	{domain}
/>