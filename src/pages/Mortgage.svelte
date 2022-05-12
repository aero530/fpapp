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
	import DeleteModal from "../components/DeleteModal.svelte";
	import AccountCard from "../components/AccountCard.svelte";
	
	import AddAlt from '../icons/AddAlt.svelte';
	import {AccountType} from "../stores";
	
	let deleteModal = {
		open: false,
		id:""
	};
</script>

<DeleteModal id={deleteModal.id} open={deleteModal.open}/>

<div class="flex items-center">
	<div class="text-lg pr-2">Mortgage</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.mortgage)}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
	{#if $form_inputs.accounts[id].type == 'mortgage'}
	<AccountCard>
		
		<div slot="inputs" class="grid grid-cols-10 gap-2">
			<div class="col-span-7">
				<TextInput
				label="Account name"
				bind:value={$form_inputs.accounts[id].name}
				questionText="Human friendly name for the account"
				/>
			</div>
			<div class="col-span-3 flex grow items-center">
				<button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" on:click={()=>(deleteModal = {open: true, id})}>
					Delete Account
				</button>
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
		
		<Scatter slot="chart" id={id} title={$form_inputs.accounts[id].name} xlabel="Year" ylabel="Amount"/>
		
		<Table
		slot="balance"
		label="Balance"
		data={$form_inputs.accounts[id].table}
		on:add={(e)=>addTableRow(form_inputs, id, 'table', e.detail.year, e.detail.value)}
		on:remove={(e)=>removeTableRow(form_inputs, id, 'table', e.detail.year)}
		/>
	</AccountCard>
	{/if}
	{/each}
</div>