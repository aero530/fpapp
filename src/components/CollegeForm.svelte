<script>
	import { form_inputs } from '../stores.js';
	
	import LayoutGrid, { Cell } from '@smui/layout-grid';
    // import { onMount, onDestroy } from "svelte";
	

	import YearInput from "./YearInput.svelte";
	import Contribution from "./Contribution.svelte";
	import Withdrawal from "./Withdrawal.svelte";
	import TaxStatus from "./TaxStatus.svelte";
	import TextInput from "./TextInput.svelte";
	import TextAreaInput from "./TextAreaInput.svelte";
	import NumberInput from "./NumberInput.svelte";
	import PercentInput from "./PercentInput.svelte";
	import Table from "./Table.svelte";
	import Scatter from "./Scatter.svelte";

	export let id;

	// onMount(async () => {
    //     console.log($form_inputs.accounts[id].withdrawals);		
	// 	console.log($form_inputs.accounts[id].hasOwnProperty('withdrawals'));
	// 	console.log(Object.keys($form_inputs.accounts[id].withdrawals).length > 0);
	// 	console.log($form_inputs.accounts[id].hasOwnProperty('withdrawals') && Object.keys($form_inputs.accounts[id].withdrawals).length > 0);
    // })

</script>

<!-- pub struct College<T: std::cmp::Ord> {
	/// String describing this account
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
	/// Determines how to interpret yearly_contribution
	contribution_type: ContributionOptions,
	/// Percent interest earned each year
	yearly_return: PercentInput,
	/// Determines how to interpret withdrawal_value
	withdrawal_type: WithdrawalOptions,
	/// How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]
	withdrawal_value: f64,
	/// How cashflow in this account is treated for tax purposes
	tax_status: TaxStatus,
	/// General information to store with this account
	notes: Option<String>,
-->
<LayoutGrid>
	<Cell span={12}>
		<TextInput
			label="Account name"
			bind:value={$form_inputs.accounts[id].name}
			questionText="Human friendly name for the account"
		/>
	</Cell>
	<Cell span={12}>
		<Table
			label="Balance"
			bind:data={$form_inputs.accounts[id].table}
		/>
	</Cell>
	{#if $form_inputs.accounts[id].hasOwnProperty('contributions') && Object.keys($form_inputs.accounts[id].contributions).length > 0}
		<Cell span={12}>
			<Table
				label="Contributions"
				bind:data={$form_inputs.accounts[id].contributions}
			/>
		</Cell>
	{/if}
	{#if $form_inputs.accounts[id].hasOwnProperty('earnings') && Object.keys($form_inputs.accounts[id].earnings).length > 0}
		<Cell span={12}>
			<Table
				label="Earnings"
				bind:data={$form_inputs.accounts[id].earnings}
			/>
		</Cell>
	{/if}
	{#if $form_inputs.accounts[id].withdrawals }
		<Cell span={12}>
			<Table
				label="Withdrawals"
				bind:data={$form_inputs.accounts[id].withdrawals}
			/>
		</Cell>
	{/if}
	<Cell span={6}>
		<YearInput
			label="Start In"
			questionText="When money will start going into this account..."
			bind:value={$form_inputs.accounts[id].startIn}
		/>
	</Cell>
	<Cell span={6}>
		<YearInput
			label="End In"
			questionText="When money will stop going into this account..."
			bind:value={$form_inputs.accounts[id].endIn}
		/>
	</Cell>
	<Cell span={6}>
		<YearInput
			label="Start Out"
			questionText="When money will start coming out of this account..."
			bind:value={$form_inputs.accounts[id].startOut}
		/>
	</Cell>
	<Cell span={6}>
		<YearInput
			label="End Out"
			questionText="When money will stop coming out of this account..."
			bind:value={$form_inputs.accounts[id].endOut}
		/>
	</Cell>
	<Cell span={6}>
		<NumberInput
			label="Contribution Value"
			step=1
			bind:value={$form_inputs.accounts[id].contributionValue}
			questionText="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]"
		/>
	</Cell>
	<Cell span={6}>
		<Contribution
			label="Contribution Type"
			bind:value={$form_inputs.accounts[id].contributionType}
		/>
	</Cell>
	<Cell span={12}>
		<PercentInput
			label="Yearly Return"
			bind:value={$form_inputs.accounts[id].yearlyReturn}
			questionText="Percent interest earned each year"
		/>
	</Cell>
	<Cell span={6}>
		<NumberInput
			label="Withdrawal Value"
			step=1
			bind:value={$form_inputs.accounts[id].withdrawalValue}
			questionText="How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]"
		/>
	</Cell>
	<Cell span={6}>
		<Withdrawal
			label="Withdrawal Type"
			bind:value={$form_inputs.accounts[id].withdrawalType}
		/>
	</Cell>
	<Cell span={12}>
		<TaxStatus
			label="Tax Status"
			bind:value={$form_inputs.accounts[id].taxStatus}
		/>
	</Cell>
	<Cell span={12}>
		<TextAreaInput
			label="Notes"
			bind:value={$form_inputs.accounts[id].notes}
			questionText="General information to store with this account"
		/>
	</Cell>
</LayoutGrid>