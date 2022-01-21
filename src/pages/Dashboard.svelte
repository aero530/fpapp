<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Textfield from '@smui/textfield';

	import Question from '../icons/Question.svelte';

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

<Question />

<LayoutGrid>
	{#if $form_inputs.settings.hasOwnProperty('ageDie')}
	<Cell span={6}>
		<Textfield
			label="ageDie"
			bind:value={$form_inputs.settings.ageDie}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="ageRetire"
			bind:value={$form_inputs.settings.ageRetire}
			type="number"
			input$step=1
		/>
	</Cell>

	<Cell span={6}>
		<Textfield
			label="yearBorn"
			bind:value={$form_inputs.settings.yearBorn}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="yearStart"
			bind:value={$form_inputs.settings.yearStart}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="inflationBase"
			bind:value={$form_inputs.settings.inflationBase}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="retirementCostOfLiving"
			bind:value={$form_inputs.settings.retirementCostOfLiving}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="taxCapitalGains"
			bind:value={$form_inputs.settings.taxCapitalGains}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="taxIncome"
			bind:value={$form_inputs.settings.taxIncome}
			type="number"
		/>
	</Cell>
	{:else}
		<Cell span={12}>
			Please load data
		</Cell>
	{/if}
</LayoutGrid>

<style>
h1 {
    color: #00d9ff;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
}
</style>