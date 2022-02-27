<script lang="ts">
	import { form_inputs } from '../stores.js';
	import {addTableRow, removeTableRow} from "../helper";

	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import Contribution from "../components/Contribution.svelte";
	import Withdrawal from "../components/Withdrawal.svelte";
	import TaxStatus from "../components/TaxStatus.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import PercentInput from "../components/PercentInput.svelte";
	import Table from "../components/Table.svelte";
</script>

<h1 class="text-lg">College Savings</h1>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'college'}
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
									questionText="When money will start going into this account..."
									bind:value={$form_inputs.accounts[id].startIn}
								/>
							
							</div>
							<div class="col-span-5">
								<YearInput
									label="End In"
									questionText="When money will stop going into this account..."
									bind:value={$form_inputs.accounts[id].endIn}
								/>
							</div>
							<div class="col-span-5">
								<YearInput
									label="Start Out"
									questionText="When money will start coming out of this account..."
									bind:value={$form_inputs.accounts[id].startOut}
								/>
							</div>
							<div class="col-span-5">
								<YearInput
									label="End Out"
									questionText="When money will stop coming out of this account..."
									bind:value={$form_inputs.accounts[id].endOut}
								/>
							</div>
							<div class="col-span-10">
								<NumberInput
									label="Contribution Value"
									step={1}
									bind:value={$form_inputs.accounts[id].contributionValue}
									questionText="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]"
								/>
							</div>
							<div class="col-span-10">
								<Contribution
									label="Contribution Type"
									bind:value={$form_inputs.accounts[id].contributionType}
								/>
							</div>
							<div class="col-span-10">
								<NumberInput
									label="Withdrawal Value"
									step={1}
									bind:value={$form_inputs.accounts[id].withdrawalValue}
									questionText="How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]"
								/>
							</div>
							<div class="col-span-10">
								<Withdrawal
									label="Withdrawal Type"
									bind:value={$form_inputs.accounts[id].withdrawalType}
								/>
							</div>
							<div class="col-span-10">
								<PercentInput
									label="Yearly Return"
									bind:value={$form_inputs.accounts[id].yearlyReturn}
									questionText="Percent interest earned each year"
								/>
							</div>
							<div class="col-span-10">
								<TaxStatus
									label="Tax Status"
									bind:value={$form_inputs.accounts[id].taxStatus}
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
					{#if $form_inputs.accounts[id].hasOwnProperty('contributions') && Object.keys($form_inputs.accounts[id].contributions).length > 0}
						<div>
							<Table
								label="Contributions"
								data={$form_inputs.accounts[id].contributions}
								on:add={(e)=>addTableRow(form_inputs, id, 'contributions', e.detail.year, e.detail.value)}
								on:remove={(e)=>removeTableRow(form_inputs, id, 'contributions', e.detail.year)}
							/>
						</div>
					{/if}
					{#if $form_inputs.accounts[id].hasOwnProperty('earnings') && Object.keys($form_inputs.accounts[id].earnings).length > 0}
						<div>
							<Table
								label="Earnings"
								data={$form_inputs.accounts[id].earnings}
								on:add={(e)=>addTableRow(form_inputs, id, 'earnings', e.detail.year, e.detail.value)}
								on:remove={(e)=>removeTableRow(form_inputs, id, 'earnings', e.detail.year)}
							/>
						</div>
					{/if}
					{#if $form_inputs.accounts[id].withdrawals }
						<div>
							<Table
								label="Withdrawals"
								data={$form_inputs.accounts[id].withdrawals}
								on:add={(e)=>addTableRow(form_inputs, id, 'withdrawals', e.detail.year, e.detail.value)}
								on:remove={(e)=>removeTableRow(form_inputs, id, 'withdrawals', e.detail.year)}
							/>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	{/each}
</div>