<script>
	import { form_inputs } from '../stores.js';
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

	import YearInput from "../components/YearInput.svelte";
	import Contribution from "../components/Contribution.svelte";

	export let id;
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

<Form on:submit>
  	<TextInput
		labelText="Account name"
		helperText="Human friendly name for the account"
		placeHolder="Account name..."
		bind:value={$form_inputs.accounts[id].name}
		/>
	<YearInput
		labelText="Start In"
		placeHolder="When money will start going into this account..."
		bind:value={$form_inputs.accounts[id].startIn}
		/>
	<YearInput
		labelText="End In"
		placeHolder="When money will stop going into this account..."
		bind:value={$form_inputs.accounts[id].endIn}
		/>
	<YearInput
		labelText="Start Out"
		placeHolder="When money will start coming out of this account..."
		bind:value={$form_inputs.accounts[id].startOut}
		/>
	<YearInput
		labelText="End Out"
		placeHolder="When money will stop coming out of this account..."
		bind:value={$form_inputs.accounts[id].endOut}
		/>
	<NumberInput
		label="Contribution Value"
		helperText="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]"
		min=0
		bind:value={$form_inputs.accounts[id].contributionValue}
	/>
	<Contribution
		labelText="Contribution Type"
		bind:selected={$form_inputs.accounts[id].contributionType}
	/>
  </Form>
