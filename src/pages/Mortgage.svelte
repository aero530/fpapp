<script lang="ts">
	import { form_inputs } from '../stores.js';
	import {addTableRow, removeTableRow} from "../helper";
	
	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import Payment from "../components/Payment.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import PercentInput from "../components/PercentInput.svelte";
	import Table from "../components/Table.svelte";
</script>

<!-- /// String describing this account
name: String,
/// Table of outstanding mortgage balance
table: Table<T>,

/// Calendar year when payments to this account start
start_out: YearInput,
/// Calendar year when payments to this account stop
end_out: YearInput,

/// Determines how to interpret payment_value
payment_type: PaymentOptions,
/// How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]
payment_value: f64,

/// Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.
rate: PercentInput,
/// Number of times per year that interest is compounded. (1=yearly, 12=monthly)
compound_time: f64,
/// Mortgage insurance payment expressed as a yearly fixed number [in today's dollars]
mortgage_insurance: f64,
/// Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment
ltv_limit: f64,
/// Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase) [in today's dollars]
escrow_value: f64,
/// Current value of the home.  This is used to compute loan to value [in today's dollars]
home_value: f64,

/// General information to store with this account
notes: Option<String>, -->

<h1 class="text-lg">Mortgage</h1>


<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'mortgage'}
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

							<!-- /// Determines how to interpret payment_value
							payment_type: PaymentOptions,
							/// How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]
							payment_value: f64, -->

							
							<div class="col-span-5">
								<Payment
									label="Payment Type"
									bind:value={$form_inputs.accounts[id].paymentType}
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Payment Value"
									step={1}
									bind:value={$form_inputs.accounts[id].paymentValue}
									questionText="How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]"
								/>
							</div>
	

							<div class="col-span-5">
								<PercentInput
									label="Interest Rate"
									bind:value={$form_inputs.accounts[id].rate}
									questionText="Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types."
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Compound Freq"
									step={1}
									bind:value={$form_inputs.accounts[id].compoundTime}
									questionText="Number of times per year that interest is compounded. (1=yearly, 12=monthly)"
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Mortgage Insurance"
									step={1}
									bind:value={$form_inputs.accounts[id].mortgageInsurance}
									questionText="Mortgage insurance payment expressed as a yearly fixed number [in today's dollars]"
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Loan to Value"
									step={1}
									bind:value={$form_inputs.accounts[id].ltvLimit}
									questionText="Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment"
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Escrow"
									step={1}
									bind:value={$form_inputs.accounts[id].escrowValue}
									questionText="Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase) [in today's dollars]"
								/>
							</div>
							<div class="col-span-5">
								<NumberInput
									label="Home Value"
									step={1}
									bind:value={$form_inputs.accounts[id].homeValue}
									questionText="Current value of the home.  This is used to compute loan to value [in today's dollars]"
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