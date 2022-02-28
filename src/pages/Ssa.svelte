<script>
	import { form_inputs } from '../stores.js';
	
	import Scatter from "../components/Scatter.svelte";
	import YearInput from "../components/YearInput.svelte";
	import TextInput from "../components/TextInput.svelte";
	import TextAreaInput from "../components/TextAreaInput.svelte";
	import NumberInput from "../components/NumberInput.svelte";
	import DeleteModal from "../components/DeleteModal.svelte";

	import AddAlt from '../icons/AddAlt.svelte';
	import {AccountType} from "../stores";

	let deleteModal = {
		open: false,
		id:""
	};

</script>

<!--     /// String describing this account
    name: String,
    /// Base income from social security
    base: f64,
    /// Calendar year when money starts being earned by this account
    start_in: YearInput,
    /// Calendar year when money stops being earned by this account
    end_in: YearInput,
    /// General information to store with this account
    notes: Option<String>, -->

<DeleteModal id={deleteModal.id} open={deleteModal.open}/>

<div class="flex items-center">
	<div class="text-lg pr-2">SSA</div>
	<div on:click={()=>form_inputs.addAccount(AccountType.ssa)}>
		<AddAlt />
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#each Object.keys($form_inputs.accounts) as id}
		{#if $form_inputs.accounts[id].type == 'ssa'}
			<div class="grid grid-rows-1 even:bg-slate-200">
				<div class="grid grid-cols-10 gap-2 ">
					<div class="col-span-5">
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
					</div>
					<div class="col-span-5">
						<Scatter id={id} title={$form_inputs.accounts[id].name} xlabel="Year" ylabel="Amount"/>
					</div>
				</div>
			</div>
		{/if}
	{/each}
</div>