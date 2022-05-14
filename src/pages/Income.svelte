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
	<div class="text-lg pr-2">Income</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.income)}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
	{#if $form_inputs.accounts[id].type == 'income'}
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
		
		<Scatter slot="chart" id={id} title={$form_inputs.accounts[id].name} xLabel="Year" yLabel="Amount"/>
		
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