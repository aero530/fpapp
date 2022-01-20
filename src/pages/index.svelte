<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
    import {
        Form,
        FormGroup,
        Checkbox,
        RadioButtonGroup,
        RadioButton,
        Select,
        SelectItem,
        Button,
        TextInput,
		NumberInput,
    } from "carbon-components-svelte";
	
	import { Grid, Row, Column } from "carbon-components-svelte";

	import "carbon-components-svelte/css/white.css";

	import { form_inputs } from '../stores.js';

	onMount(() => {
	    invoke('my_custom_command');
	});

// ageDie : 95
// ageRetire : 67
// inflationBase : 3
// retirementCostOfLiving : 100
// ssa : {"breakpoints":{"high":44000,"low":32000},"taxableIncomePercentage":{"high":85,"low":50}}
// taxCapitalGains : 15
// taxIncome : 22
// yearBorn : 1982
// yearStart : 2022

</script>

<h1>Dashboard</h1>

<Grid>
	<Row>
		<Column>
			<NumberInput
				label="ageDie"
				bind:value={$form_inputs.settings.ageDie}
			/>
		</Column>
		<Column>
			<NumberInput
				label="ageRetire"
				bind:value={$form_inputs.settings.ageRetire}
			/>
		</Column>
	</Row>
	<Row>
		<Column>
			<NumberInput
				label="yearBorn"
				bind:value={$form_inputs.settings.yearBorn}
			/>
		</Column>
		<Column>
			<NumberInput
				label="yearStart"
				bind:value={$form_inputs.settings.yearStart}
			/>
		</Column>
	</Row>
	<Row>
		<Column>
			<NumberInput
				label="inflationBase"
				bind:value={$form_inputs.settings.inflationBase}
			/>
		</Column>
		<Column>
			<NumberInput
				label="retirementCostOfLiving"
				bind:value={$form_inputs.settings.retirementCostOfLiving}
			/>
		</Column>
	</Row>
	<Row>
		<Column>
			<NumberInput
				label="taxCapitalGains"
				bind:value={$form_inputs.settings.taxCapitalGains}
			/>
		</Column>
		<Column>
			<NumberInput
				label="taxIncome"
				bind:value={$form_inputs.settings.taxIncome}
			/>
		</Column>
	</Row>
</Grid>

<style>
h1 {
    color: #00d9ff;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
}
</style>