<script lang="ts">
	import { form_inputs } from '../stores.js';
	import {addTableRow, removeTableRow} from "../helper";
	
	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import Expense from "../components/Expense.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import Table from "../components/Table.svelte";
	import AccountLink from "../components/AccountLink.svelte";
</script>
  
<h1 class="text-lg">Expenses</h1>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'expense'}
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
									bind:value={$form_inputs.accounts[id].isHealthcare}
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
