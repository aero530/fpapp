<script lang="ts">
	import { AccountType, form_inputs } from '../stores';
	import {addTableRow, removeTableRow} from "../helper";
	
	import Line from "../components/Line.svelte";
	import YearInput from "../components/YearInput.svelte";
	import Expense from "../components/Expense.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import Table from "../components/Table.svelte";
	import AccountLink from "../components/AccountLink.svelte";
	import DeleteModal from "../components/DeleteModal.svelte";
	import AccountCard from "../components/AccountCard.svelte";
	
	import AddAlt from '../icons/AddAlt.svelte';
	
	
	let deleteModal = {
		open: false,
		id:""
	};
	
</script>

<DeleteModal id={deleteModal.id} open={deleteModal.open}/>

<div class="flex items-center">
	<div class="text-lg pr-2">Expenses</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.expense)} on:keypress={() => {}}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
	{#if $form_inputs.accounts[id].type == 'expense'}
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
				<NumberInput
				label="Expense Value"
				step={1}
				bind:value={$form_inputs.accounts[id].expenseValue}
				questionText="Yearly cost of the expense [in today's dollars]"
				/>
			</div>
			<div class="col-span-5">
				<Expense
				label="Expense Type"
				bind:value={$form_inputs.accounts[id].expenseType}
				/>
			</div>
			<div class="col-span-10">
				<AccountLink
				label="Healthcare cost?"
				bind:isOn={$form_inputs.accounts[id].isHealthcare}
				bind:account={$form_inputs.accounts[id].hsaLink}
				accounts={$form_inputs.accounts}
				accountTypeFilter='hsa'
				questionText="Is this a healthcare cost that should be paid for out of HSA"
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
		
		<Line slot="chart" id={id} title={$form_inputs.accounts[id].name} xLabel="Year" yLabel="Amount"/>
		
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