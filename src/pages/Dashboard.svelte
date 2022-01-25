<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Textfield from '@smui/textfield';

	import Scatter from "../components/Scatter.svelte"

	import Question from '../icons/Question.svelte';

	import { form_inputs } from '../stores.js';


	onMount(() => {
	    invoke('my_custom_command');
	});


	// let plugin = {
	// 	id: 'custom_canvas_background_color',
	// 	beforeDraw: (chart) => {
	// 		const ctx = chart.canvas.getContext('2d');
	// 		ctx.save();
	// 		ctx.globalCompositeOperation = 'destination-over';
	// 		ctx.fillStyle = 'white';
	// 		ctx.fillRect(0, 0, chart.width, chart.height);
	// 		ctx.restore();
	// 	}
	// };
	// let plugins=[plugin];

	// let data = {
  //   labels: ['Scatter'],
  //   datasets: [
  //     {
  //       borderColor: 'rgba(0,180,0, .8)', // outline color for the data point
  //       backgroundColor: 'rgba(255,0,125, .5)', // color used for the line and data point fill
	// 	borderWidth: 1, // Set width of line
	// 	pointBorderWidth: 2, // Set width of outline of data point
	// 	pointRadius: 4, // setting to 0 will make the data point not render
	// 	pointStyle: 'rectRounded', // 'circle' 'cross' 'crossRot' 'dash' 'line' 'rect' 'rectRounded' 'rectRot' 'star' 'triangle'
  //       label: 'V(node2)',
	// 	showLine: true,
  //       data: [
  //         {
  //           x: 1,
  //           y: -1.711e-2
  //         },
  //         {
  //           x: 1.26,
  //           y: -2.708e-2
  //         },
  //         {
  //           x: 1.58,
  //           y: -4.285e-2
  //         },
  //         {
  //           x: 2.0,
  //           y: -6.772e-2
  //         },
  //         {
  //           x: 2.51,
  //           y: -1.068e-1
  //         },
  //         {
  //           x: 3.16,
  //           y: -1.681e-1
  //         },
  //         {
  //           x: 3.98,
  //           y: -2.635e-1
  //         },
  //         {
  //           x: 5.01,
  //           y: -4.106e-1
  //         },
  //         {
  //           x: 6.31,
  //           y: -6.339e-1
  //         },
  //         {
  //           x: 7.94,
  //           y: -9.659e-1
  //         },
  //         {
  //           x: 10.0,
  //           y: -1.445
  //         },
  //         {
  //           x: 12.6,
  //           y: -2.11
  //         },
  //         {
  //           x: 15.8,
  //           y: -2.992
  //         },
  //         {
  //           x: 20.0,
  //           y: -4.102
  //         },
  //         {
  //           x: 25.1,
  //           y: -5.429
  //         },
  //         {
  //           x: 31.6,
  //           y: -6.944
  //         },
  //         {
  //           x: 39.8,
  //           y: -8.607
  //         },
  //         {
  //           x: 50.1,
  //           y: -1.038e1
  //         },
  //         {
  //           x: 63.1,
  //           y: -1.223e1
  //         },
  //         {
  //           x: 79.4,
  //           y: -1.413e1
  //         },
  //         {
  //           x: 100.0,
  //           y: -1.607e1
  //         },
  //         {
  //           x: 126,
  //           y: -1.803e1
  //         },
  //         {
  //           x: 158,
  //           y: -2e1
  //         },
  //         {
  //           x: 200,
  //           y: -2.199e1
  //         },
  //         {
  //           x: 251,
  //           y: -2.398e1
  //         },
  //         {
  //           x: 316,
  //           y: -2.597e1
  //         },
  //         {
  //           x: 398,
  //           y: -2.797e1
  //         },
  //         {
  //           x: 501,
  //           y: -2.996e1
  //         },
  //         {
  //           x: 631,
  //           y: -3.196e1
  //         },
  //         {
  //           x: 794,
  //           y: -3.396e1
  //         },
  //         {
  //           x: 1000,
  //           y: -3.596e1
  //         }
  //       ]
  //     }
  //   ]
  // };

  // let options = {
  //   title: {
  //     display: true,
  //     text: 'Scatter Chart - Logarithmic X-Axis'
  //   },
  //   scales: {
  //     x: {
  //         type: 'logarithmic',
	// 	  position: 'bottom',
  //         ticks: {
  //           callback: function (value,index,ticks) {
  //               return value.toString() + 'Hz';
  //           }
  //         },
  //         title: {
  //           text: 'Frequency',
  //           display: true
  //         }
  //       },
  //     y: {
  //         type: 'linear',
  //         ticks: {
  //           callback: function (value,index,ticks) {
  //             return value.toString() + 'dB';
  //           }
  //         },
  //         title: {
  //           text: 'Voltage',
  //           display: true
  //         }
  //       }
  //   },
	// plugins: {
	// 	legend: {
	// 		labels: {
	// 			boxWidth: 15,
	// 			usePointStyle: true,
	// 		}
	// 	}
	// }
  // };

// ageDie : 95
// ageRetire : 67
// inflationBase : 3
// retirementCostOfLiving : 100
// ssa : {"breakpoints":{"high":44000,"low":32000},"taxableIncomePercentage":{"high":85,"low":50}}
// taxCapitalGains : 15
// taxIncome : 22
// yearBorn : 1982
// yearStart : 2022

let datasets = [ {
  label:"Hi there",
  data:[
          {
            x: 1,
            y: -1.711e-2
          },
          {
            x: 1.26,
            y: -2.708e-2
          },
          {
            x: 1.58,
            y: -4.285e-2
          },
          {
            x: 2.0,
            y: -6.772e-2
          },
          {
            x: 2.51,
            y: -1.068e-1
          },
          {
            x: 3.16,
            y: -1.681e-1
          },
          {
            x: 3.98,
            y: -2.635e-1
          },
          {
            x: 5.01,
            y: -4.106e-1
          },
          {
            x: 6.31,
            y: -6.339e-1
          },
          {
            x: 7.94,
            y: -9.659e-1
          },
          {
            x: 10.0,
            y: -1.445
          },
          {
            x: 12.6,
            y: -2.11
          },
          {
            x: 15.8,
            y: -2.992
          },
          {
            x: 20.0,
            y: -4.102
          },
          {
            x: 25.1,
            y: -5.429
          },
          {
            x: 31.6,
            y: -6.944
          },
          {
            x: 39.8,
            y: -8.607
          },
          {
            x: 50.1,
            y: -1.038e1
          },
          {
            x: 63.1,
            y: -1.223e1
          },
          {
            x: 79.4,
            y: -1.413e1
          },
          {
            x: 100.0,
            y: -1.607e1
          },
          {
            x: 126,
            y: -1.803e1
          },
          {
            x: 158,
            y: -2e1
          },
          {
            x: 200,
            y: -2.199e1
          },
          {
            x: 251,
            y: -2.398e1
          },
          {
            x: 316,
            y: -2.597e1
          },
          {
            x: 398,
            y: -2.797e1
          },
          {
            x: 501,
            y: -2.996e1
          },
          {
            x: 631,
            y: -3.196e1
          },
          {
            x: 794,
            y: -3.396e1
          },
          {
            x: 1000,
            y: -3.596e1
          }
        ]}];


</script>

<h1>Dashboard</h1>



<Question />



<LayoutGrid>
	<Cell span={6} >
		<Scatter
      inputdata={datasets}
      title="This is my graph"
      xlabel="x label"
      ylabel="why label?"
      />
	</Cell>
	<Cell span={6} >
	
		<span>iuqweroqiwuerqwoiuer</span>
	</Cell>


	{#if $form_inputs.settings.hasOwnProperty('ageDie')}
	
	<Cell span={6}>
		<Textfield
			label="ageDie"
			bind:value={$form_inputs.settings.ageDie}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="ageRetire"
			bind:value={$form_inputs.settings.ageRetire}
			type="number"
			input$step=1
		/>
	</Cell>

	<Cell span={6}>
		<Textfield
			label="yearBorn"
			bind:value={$form_inputs.settings.yearBorn}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="yearStart"
			bind:value={$form_inputs.settings.yearStart}
			type="number"
			input$step=1
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="inflationBase"
			bind:value={$form_inputs.settings.inflationBase}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="retirementCostOfLiving"
			bind:value={$form_inputs.settings.retirementCostOfLiving}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="taxCapitalGains"
			bind:value={$form_inputs.settings.taxCapitalGains}
			type="number"
		/>
	</Cell>
	<Cell span={6}>
		<Textfield
			label="taxIncome"
			bind:value={$form_inputs.settings.taxIncome}
			type="number"
		/>
	</Cell>
	{:else}
		<Cell span={12}>
			Please load data
		</Cell>
	{/if}
</LayoutGrid>

<style>
h1 {
    color: #00d9ff;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
}
</style>