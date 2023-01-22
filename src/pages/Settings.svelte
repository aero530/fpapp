<script>
	// import { invoke } from "@tauri-apps/api/tauri";
	// 	function performRequest() {
	// 		invoke("do_a_thing", {
	// 			body: {
	// 				id: 5,
	// 				name: "test",
	// 			},
	// 		})
	// 		.then((reply) => alert(reply));
	// 	}
		
	//  <button on:click={performRequest}>
	// 	    Perform Request
	//  </button>
	
	import NumberInput from "../components/NumberInput.svelte"
	import YearInput from "../components/YearInput.svelte"
	import { form_inputs } from '../stores';
</script>

<h1 class="text-lg">Settings</h1>

{#if $form_inputs.settings.hasOwnProperty('ageDie')}
<div class="grid grid-cols-2 gap-4">
	<div>
		<NumberInput
		label="ageDie"
		bind:value={$form_inputs.settings.ageDie}
		step=1
		questionText="Age when you migth die (for financial purposes as least)"
		/>
	</div>
	<div>
		<NumberInput
		label="ageRetire"
		bind:value={$form_inputs.settings.ageRetire}
		step={1}
		questionText="Age you plan to retire at"
		/>
	</div>
	<div>
		<YearInput
		label="yearBorn"
		bind:value={$form_inputs.settings.yearBorn}
		questionText="Year you were born in"
		/>
	</div>
	<div>
		<YearInput
		label="yearStart"
		bind:value={$form_inputs.settings.yearStart}
		questionText="Year to start the simulation"
		/>
	</div>
	<div>
		<NumberInput
		label="inflationBase"
		bind:value={$form_inputs.settings.inflationBase}
		questionText="Base rate of inflation (percent)"
		/>
	</div>
	<div>
		<NumberInput
		label="retirementCostOfLiving"
		bind:value={$form_inputs.settings.retirementCostOfLiving}
		questionText="Fraction of current spending when retired (such as in retirement you will spend 80% of what you spend now)"
		/>
	</div>
	<div>
		<NumberInput
		label="taxCapitalGains"
		bind:value={$form_inputs.settings.taxCapitalGains}
		questionText="Tax rate for capital gains"
		/>
	</div>
	<div>
		<NumberInput
		label="taxIncome"
		bind:value={$form_inputs.settings.taxIncome}
		questionText="Tax rate for your income bracket"
		/>
	</div>
</div>
<div class="mt-4">
	<h1 class="text-md">Social Security Settings</h1>
	<div class="grid grid-cols-2 gap-4">
		<div>
			<NumberInput
			label="Lower Income Bound"
			bind:value={$form_inputs.settings.ssa.breakpoints.low}
			questionText="Lower income bound for social security"
			/>
		</div>
		<div>
			<NumberInput
			label="Upper Income Bound"
			bind:value={$form_inputs.settings.ssa.breakpoints.high}
			questionText="Upper income bound for social security"
			/>
		</div>
		<div>
			<NumberInput
			label="Lower taxableIncomePercentage"
			bind:value={$form_inputs.settings.ssa.taxableIncomePercentage.low}
			questionText="Lower taxableIncomePercentage for social security"
			/>
		</div>
		<div>
			<NumberInput
			label="Upper taxableIncomePercentage"
			bind:value={$form_inputs.settings.ssa.taxableIncomePercentage.high}
			questionText="Upper taxableIncomePercentage for social security"
			/>
		</div>
	</div>
</div>
{:else}
<span>
	Please load data
</span>
{/if}