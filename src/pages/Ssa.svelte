<script>
	import { AccountType, form_inputs } from '../stores';
	
	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
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
	<div class="text-lg pr-2">SSA</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.ssa)} on:keypress={() => {}}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
	{#if $form_inputs.accounts[id].type == 'ssa'}
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
				<NumberInput
				label="Base"
				step={1}
				bind:value={$form_inputs.accounts[id].base}
				questionText="Base income from social security"
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
	</AccountCard>
	{/if}
	{/each}
</div>