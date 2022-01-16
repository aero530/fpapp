<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
  	import { UnorderedList, ListItem } from "carbon-components-svelte";

	import "carbon-components-svelte/css/white.css";

	import { input_data } from '../stores.js';

	onMount(() => {
	    invoke('my_custom_command');
	});

</script>

<h1>Dashboard</h1>

<UnorderedList>
{#each Object.entries($input_data.settings) as [setting, value]}
    <ListItem>{setting} : {JSON.stringify(value)}</ListItem>
{/each}
</UnorderedList>

<style>
h1 {
    color: #00d9ff;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
}
</style>