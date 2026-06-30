<script>
	import NumberInput from "../components/NumberInput.svelte";
	import YearInput from "../components/YearInput.svelte";
	import { form_inputs } from "../stores";
</script>

<div class="container mx-auto columns-1">
	{#if $form_inputs.settings.hasOwnProperty("ageDie")}
		<h1 class="mt-4 mb-2 text-xl font-bold">Settings</h1>

		<div
			class="grid grid-cols-2 gap-4 border rounded p-4 bg-background-600 dark:bg-darkbackground-600"
		>
			<NumberInput
				label="ageDie"
				bind:value={$form_inputs.settings.ageDie}
				step={1}
				questionText="Age when you might die (for financial purposes as least)"
			/>

			<NumberInput
				label="ageRetire"
				bind:value={$form_inputs.settings.ageRetire}
				step={1}
				questionText="The age at which you plan to retire"
			/>

			<YearInput
				label="yearBorn"
				bind:value={$form_inputs.settings.yearBorn}
				questionText="Year in which you were born"
			/>

			<YearInput
				label="yearStart"
				bind:value={$form_inputs.settings.yearStart}
				questionText="Year to start the simulation"
			/>

			<NumberInput
				label="inflationBase"
				bind:value={$form_inputs.settings.inflationBase}
				questionText="Base rate of inflation (percent)"
			/>

			<NumberInput
				label="retirementCostOfLiving"
				bind:value={$form_inputs.settings.retirementCostOfLiving}
				questionText="Fraction of current spending when retired (such as in retirement you will spend 80% of what you spend now)"
			/>

			<NumberInput
				label="taxCapitalGains"
				bind:value={$form_inputs.settings.taxCapitalGains}
				questionText="Tax rate for capital gains"
			/>

			<NumberInput
				label="taxIncome"
				bind:value={$form_inputs.settings.taxIncome}
				questionText="Tax rate for your income bracket"
			/>
		</div>

		<h1 class="mt-6 mb-2 text-xl font-bold">Social Security Settings</h1>

		<div
			class="border rounded p-4 bg-background-600 dark:bg-darkbackground-600"
		>
			<div class="grid grid-cols-2 gap-4">
				<NumberInput
					label="Lower Income Bound"
					bind:value={$form_inputs.settings.ssa.breakpoints.low}
					questionText="Lower income bound for social security"
				/>

				<NumberInput
					label="Upper Income Bound"
					bind:value={$form_inputs.settings.ssa.breakpoints.high}
					questionText="Upper income bound for social security"
				/>

				<NumberInput
					label="Lower taxableIncomePercentage"
					bind:value={$form_inputs.settings.ssa
						.taxableIncomePercentage.low}
					questionText="Lower taxableIncomePercentage for social security"
				/>

				<NumberInput
					label="Upper taxableIncomePercentage"
					bind:value={$form_inputs.settings.ssa
						.taxableIncomePercentage.high}
					questionText="Upper taxableIncomePercentage for social security"
				/>
			</div>
		</div>
	{:else}
		<div
			class="grid grid-cols-2 gap-4 border rounded p-4 bg-background-600 dark:bg-darkbackground-600"
		>
			Please load data
		</div>
	{/if}
</div>
