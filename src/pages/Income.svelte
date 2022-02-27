<script lang="ts">
	import { form_inputs } from '../stores.js';
	import {addTableRow, removeTableRow} from "../helper";
	
	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import PercentInput from "../components/PercentInput.svelte";
	import Table from "../components/Table.svelte";
</script>

<!-- /// String describing this account
name: String,
/// Table of account income
table: Table<T>,
/// Base pay (with bonuses) [in today's dollars]
base: f64,

/// Calendar year when money starts being earned by this account
start_in: YearInput,
/// Calendar year when money stops being earned by this account
end_in: YearInput,

/// Yearly increase in income as a percent
raise: PercentInput,

/// General information to store with this account
notes: Option<String>,-->

<h1 class="text-lg">Income</h1>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'income'}
			<div class="grid grid-rows-1 even:bg-slate-200">
				<div class="grid grid-cols-10 gap-2 ">
					<div class="col-span-5">
						<div class="grid grid-cols-10 gap-2">
							<div class="col-span-10">
								<TextInput
									label="Account name"
									bind:value={$form_inputs.accounts[id].name}
									questionText="Human friendly name for the account"
								/>
							</div>
							<div class="col-span-5">
								<YearInput
									label="Start In"
									questionText="Calendar year when money starts being earned by this account"
									bind:value={$form_inputs.accounts[id].startIn}
								/>
							</div>
							<div class="col-span-5">
								<YearInput
									label="End In"
									questionText="Calendar year when money stops being earned by this account"
									bind:value={$form_inputs.accounts[id].endIn}
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Base Pay"
									step={1}
									bind:value={$form_inputs.accounts[id].base}
									questionText="Base pay (with bonuses) [in today's dollars]"
								/>
							</div>
							<div class="col-span-5">
								<PercentInput
									label="Raise"
									bind:value={$form_inputs.accounts[id].raise}
									questionText="Yearly increase in income as a percent"
								/>
							</div>

							<div class="col-span-10">
								<TextAreaInput
									label="Notes"
									bind:value={$form_inputs.accounts[id].notes}
									questionText="General information to store with this account"
								/>
							</div>

						</div>
					</div>
					<div class="col-span-5">
						<Scatter id={id} title={$form_inputs.accounts[id].name} xlabel="Year" ylabel="Amount"/>
					</div>
				</div>
				<div class="grid grid-cols-2 gap-0">
					<div>
						<Table
							label="Balance"
							data={$form_inputs.accounts[id].table}
							on:add={(e)=>addTableRow(form_inputs, id, 'table', e.detail.year, e.detail.value)}
							on:remove={(e)=>removeTableRow(form_inputs, id, 'table', e.detail.year)}
						/>
					</div>
				</div>
			</div>
		{/if}
	{/each}
</div>
