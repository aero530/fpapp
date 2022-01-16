<script>
	import { form_inputs, plot_data } from '../stores.js';
	import { Tile, Grid, Row, Column } from "carbon-components-svelte";

	import { LineChart } from "@carbon/charts-svelte";
  	import "@carbon/charts/styles.min.css";
  	import "carbon-components/css/carbon-components.min.css";

</script>
  
<h3>College</h3>

{#each Object.entries($form_inputs.college) as [id, account]}
	<Tile style="margin: 10px">
		<Grid>
			<Row>
				<Column>{id} : {JSON.stringify(account)}</Column>
				
				<Column>
					<LineChart data={$plot_data[id]} options={{
						"title": "Line (time series)",
						"axes": {
							"bottom": {
								"title": "Year",
								"mapsTo": "year",
								"domain": [$plot_data[id][0].year, $plot_data[id][$plot_data[id].length-1].year]
							},
							"left": {
								"mapsTo": "value",
							}
						},
						"curve": "curveMonotoneX",
						"height": "400px"
					}}/>
				</Column>
			</Row>
		</Grid>
		
	</Tile>
{/each}