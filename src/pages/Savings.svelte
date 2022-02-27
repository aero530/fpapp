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

<!-- /// String describing this account
    name: String,

    /// Table of account balance
    table: Table<T>,
    /// Table of contributions to this account
    contributions: Option<Table<T>>,
    /// Table of account earnings
    earnings: Option<Table<T>>,
    /// Table of withdrawals from this account
    withdrawals: Option<Table<T>>,

    /// Calendar year when money starts being added to this account
    start_in: YearInput,
    /// Calendar year when money is no longer added to this account (this value is inclusive)
    end_in: YearInput,
    /// Calendar year when money starts being withdrawn from this account
    start_out: YearInput,
    /// Calendar year when money stops being withdrawn from this account
    end_out: YearInput,

    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]
    contribution_value: f64,
    /// Determines how to interpret the value in yearly_contribution
    contribution_type: ContributionOptions,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret the value in withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]
    withdrawal_value: f64,
    /// How cashflow in this account is treated for tax purposes
    tax_status: TaxStatus,

    /// General information to store with this account
    notes: Option<String>, -->

<h1 class="text-lg">Savings</h1>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'savings'}
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

							<div class="col-span-5">
								<NumberInput
									label="Contribution Value"
									step={1}
									bind:value={$form_inputs.accounts[id].contributionValue}
									questionText="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]"
								/>
							</div>
							<div class="col-span-5">
								<Contribution
									label="Contribution Type"
									bind:value={$form_inputs.accounts[id].contributionType}
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Withdrawal Value"
									step={1}
									bind:value={$form_inputs.accounts[id].withdrawalValue}
									questionText="How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]"
								/>
							</div>
							<div class="col-span-5">
								<Withdrawal
									label="Withdrawal Type"
									bind:value={$form_inputs.accounts[id].withdrawalType}
								/>
							</div>

							<div class="col-span-5">
								<PercentInput
									label="Yearly Return"
									bind:value={$form_inputs.accounts[id].yearlyReturn}
									questionText="Percent interest earned each year"
								/>
							</div>
							<div class="col-span-5">
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
					{#if $form_inputs.accounts[id].hasOwnProperty('contributions')}
						<div>
							<Table
								label="Contributions"
								data={$form_inputs.accounts[id].contributions}
								on:add={(e)=>addTableRow(form_inputs, id, 'contributions', e.detail.year, e.detail.value)}
								on:remove={(e)=>removeTableRow(form_inputs, id, 'contributions', e.detail.year)}
							/>
						</div>
					{/if}
					{#if $form_inputs.accounts[id].hasOwnProperty('earnings')}
						<div>
							<Table
								label="Earnings"
								data={$form_inputs.accounts[id].earnings}
								on:add={(e)=>addTableRow(form_inputs, id, 'earnings', e.detail.year, e.detail.value)}
								on:remove={(e)=>removeTableRow(form_inputs, id, 'earnings', e.detail.year)}
							/>
						</div>
					{/if}
					{#if $form_inputs.accounts[id].hasOwnProperty('withdrawals')}
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