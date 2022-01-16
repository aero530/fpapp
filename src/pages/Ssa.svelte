<script>
	import { form_inputs } from '../stores.js';
	import { Tile } from "carbon-components-svelte";
</script>
  
<h3>SSA</h3>

{#each Object.entries($form_inputs.ssa) as [id, account]}
	<Tile style="margin: 10px">
		{id} : {JSON.stringify(account)}
	</Tile>
{/each}