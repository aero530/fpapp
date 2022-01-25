<script>
	import { form_inputs, plot_data } from '../stores.js';
	
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	// import Textfield from '@smui/textfield';

	// import { Tile, Grid, Row, Column } from "carbon-components-svelte";
	// import { LineChart } from "@carbon/charts-svelte";
  	// import "@carbon/charts/styles.min.css";
  	// import "carbon-components/css/carbon-components.min.css";

	  
	import CollegeForm from "../components/CollegeForm.svelte";
	import Scatter from "../components/Scatter.svelte";

</script>

<h3>College Savings</h3>

{#each Object.entries($form_inputs.accounts) as [id, account]}
	{#if account.type == 'college'}
		<Paper>
			<LayoutGrid>
				<Cell span={4}>{id} : <CollegeForm id={id}/></Cell>
				<Cell span={8}>
					<Scatter inputdata={$plot_data[id]} title={account.name} xlabel="Year" ylabel="Value"/>
				</Cell>
			</LayoutGrid>
		</Paper>
	{/if}
{/each}