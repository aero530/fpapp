<script>
	import { form_inputs } from '../stores.js';
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
</script>
  
<h3>HSA</h3>

{#each Object.entries($form_inputs.hsa) as [id, account]}
	<Paper variant="outlined">
		<Title>{id}</Title>
		<Subtitle>subtitle</Subtitle>
		<Content>JSON.stringify(account)</Content>
	</Paper>
{/each}