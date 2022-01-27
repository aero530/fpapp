<script>
	import { invoke } from "@tauri-apps/api/tauri";


	import NumberInput from "../components/NumberInput.svelte"
	import YearInput from "../components/YearInput.svelte"
	import { form_inputs } from '../stores.js';
	
    function performRequest() {
		invoke("do_a_thing", {
			body: {
				id: 5,
				name: "test",
			},
		})
		.then((reply) => alert(reply));
	}

</script>
  
<h3>Settings</h3>
  
<button on:click={performRequest}>
    Perform Request
</button>


{#if $form_inputs.settings.hasOwnProperty('ageDie')}
<div class="grid grid-cols-2 gap-4">

	<div>
		<NumberInput
			label="ageDie"
			bind:value={$form_inputs.settings.ageDie}
			step=1
			questionText=""
		/>
	</div>
	<div>
		<NumberInput
			label="ageRetire"
			bind:value={$form_inputs.settings.ageRetire}
			step=1
			questionText=""
		/>
  </div>
	<div>
		<YearInput
			label="yearBorn"
			bind:value={$form_inputs.settings.yearBorn}
			questionText=""
		/>
  </div>
	<div>
		<YearInput
			label="yearStart"
			bind:value={$form_inputs.settings.yearStart}
			questionText=""
		/>
  </div>
	<div>
		<NumberInput
			label="inflationBase"
			bind:value={$form_inputs.settings.inflationBase}
			questionText=""
		/>
  </div>
	<div>
		<NumberInput
			label="retirementCostOfLiving"
			bind:value={$form_inputs.settings.retirementCostOfLiving}
			questionText=""
		/>
  </div>
	<div>
		<NumberInput
			label="taxCapitalGains"
			bind:value={$form_inputs.settings.taxCapitalGains}
			questionText=""
		/>
  </div>
	<div>
		<NumberInput
			label="taxIncome"
			bind:value={$form_inputs.settings.taxIncome}
			questionText=""
		/>
  </div>

</div>
{:else}
<span>
  Please load data
</span>
{/if}