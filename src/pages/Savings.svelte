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
	<div class="text-lg pr-2">Savings</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.savings)}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
	{#if $form_inputs.accounts[id].type == 'savings'}
	<AccountCard>
		<div  slot="inputs" class="flex-initial w-fit">
			
			<div class="grid grid-cols-10 gap-2">
				<div class="col-span-7">
					<TextInput
					label="Account name"
					bind:value={$form_inputs.accounts[id].name}
					questionText="Human friendly name for the account"
					/>
				</div>
				<div class="col-span-3 flex grow items-center">
					<button 
					class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200"
					on:click={()=>(deleteModal = {open: true, id})}
					>
					Delete Account
				</button>
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
	<div slot="chart" class="flex-auto w-[38rem]">
		<Scatter id={id} title={$form_inputs.accounts[id].name} xlabel="Year" ylabel="Amount"/>
	</div>
	
	<div slot="tables" class="flex flex-wrap gap-4">
		
		<Table
		label="Balance"
		data={$form_inputs.accounts[id].table}
		on:add={(e)=>addTableRow(form_inputs, id, 'table', e.detail.year, e.detail.value)}
		on:remove={(e)=>removeTableRow(form_inputs, id, 'table', e.detail.year)}
		/>
		
		{#if $form_inputs.accounts[id].hasOwnProperty('contributions')}
		
		<Table
		label="Contributions"
		data={$form_inputs.accounts[id].contributions}
		on:add={(e)=>addTableRow(form_inputs, id, 'contributions', e.detail.year, e.detail.value)}
		on:remove={(e)=>removeTableRow(form_inputs, id, 'contributions', e.detail.year)}
		/>
		{/if}
		{#if $form_inputs.accounts[id].hasOwnProperty('earnings')}
		
		<Table
		label="Earnings"
		data={$form_inputs.accounts[id].earnings}
		on:add={(e)=>addTableRow(form_inputs, id, 'earnings', e.detail.year, e.detail.value)}
		on:remove={(e)=>removeTableRow(form_inputs, id, 'earnings', e.detail.year)}
		/>
		{/if}
		{#if $form_inputs.accounts[id].hasOwnProperty('withdrawals')}
		
		<Table
		label="Withdrawals"
		data={$form_inputs.accounts[id].withdrawals}
		on:add={(e)=>addTableRow(form_inputs, id, 'withdrawals', e.detail.year, e.detail.value)}
		on:remove={(e)=>removeTableRow(form_inputs, id, 'withdrawals', e.detail.year)}
		/>
		{/if}
	</div>
</AccountCard>
{/if}
{/each}
</div>