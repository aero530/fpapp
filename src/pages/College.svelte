<script>
	import { form_inputs, plot_data } from '../stores.js';
	  
	import CollegeForm from "../components/CollegeForm.svelte";
	import Scatter from "../components/Scatter.svelte";

</script>

<h3>College Savings</h3>

<div class="grid grid-cols-1 gap-4">
	{#each Object.entries($form_inputs.accounts) as [id, account]}
		{#if account.type == 'college'}
			<div class="grid grid-cols-10 gap-0">
				<div class="col-span-6">
					<CollegeForm id={id}/>
				</div>
				<div class="col-span-4">
					<Scatter inputdata={$plot_data[id]} title={account.name} xlabel="Year" ylabel="Amount"/>
				</div>
			</div>
		{/if}
	{/each}
</div>