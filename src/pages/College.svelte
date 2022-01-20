<script>
	import { form_inputs, plot_data } from '../stores.js';
	import { Tile, Grid, Row, Column } from "carbon-components-svelte";
	import { LineChart } from "@carbon/charts-svelte";
  	import "@carbon/charts/styles.min.css";
  	import "carbon-components/css/carbon-components.min.css";

	import CollegeForm from "../components/CollegeForm.svelte";

</script>

<h3>College Savings</h3>

{#each Object.entries($form_inputs.accounts) as [id, account]}
	{#if account.type == 'college'}
		<Tile style="margin: 10px">
			<Grid>
				<Row>
					<Column>{id} : <CollegeForm id={id}/></Column>
					<Column>
						<LineChart data={$plot_data[id]} options={{
							"title": account.name,
							"axes": {
								"bottom": {
									"title": "Year",
									"mapsTo": "year",
									"scale": "linear",
									"domain": [$plot_data[id][0].year, $plot_data[id][$plot_data[id].length-1].year]
								},
								"left": {
									"mapsTo": "value",
									"scale": "linear"
								}
							},
							"curve": "curveMonotoneX",
							"points": {radius: 1},
							"height": "600px",
							"tooltip": {
								showTotal: false
							}
						}}/>
					</Column>
				</Row>
			</Grid>
		</Tile>
	{/if}
{/each}