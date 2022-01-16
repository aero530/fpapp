<script>
	import { input_data } from '../stores.js';
	import { Tile } from "carbon-components-svelte";
</script>
  
<h3>Retirement</h3>

{#each Object.entries($input_data.retirement) as [id, account]}
	<Tile style="margin: 10px">
		{id} : {JSON.stringify(account)}
	</Tile>
{/each}