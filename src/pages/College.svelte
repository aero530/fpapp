<script>
	import { form_inputs, plot_data } from '../stores.js';
	import { Tile, Grid, Row, Column } from "carbon-components-svelte";

	import { LineChart } from "@carbon/charts-svelte";
  	import "@carbon/charts/styles.min.css";
  	import "carbon-components/css/carbon-components.min.css";
//<Column>{id} : {JSON.stringify(account)}</Column>
</script>
  
<h3>College</h3>

{#each Object.entries($form_inputs.college) as [id, account]}
	<Tile style="margin: 10px">
		<Grid>
			<Row>
				<Column>{id}</Column>
				
				<Column>
					<LineChart data={$plot_data[id]} options={{
						"title": $form_inputs.college[id].name,
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
{/each}