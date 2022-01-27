<script>
	import { form_inputs } from '../stores.js';
</script>
  
<h3>Savings</h3>

{#each Object.entries($form_inputs.savings) as [id, account]}
<div>
	<div>{id}</div>
	<div>subtitle</div>
	<div>JSON.stringify(account)</div>
</div>
{/each}