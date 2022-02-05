<script lang="ts">
	import Scatter from "svelte-chartjs/src/Scatter.svelte";
	import { plot_data } from '../stores.js';
	import {dark} from '../stores.js';

	type Point = {x: number, y:number};
	type DataSet = { data: Point[], label: string;};
	
	export let id : string;
	export let title : string;
	export let xlabel : string;
	export let ylabel : string;

	let colors = [
		'rgba(180,0,0, .8)',
		'rgba(0,180,0, .8)',
		'rgba(0,0,180, .8)',
		'rgba(180,180,0, .8)',
		'rgba(0,180,180, .8)',
		'rgba(180,0,180, .8)'
	];

	let lightModeText = 'rgba(0,0,0,1)';
	let darkModeText = 'rgba(255,255,255,1)';
	
	let plugin = {
		id: 'custom_canvas_background_color',
		beforeDraw: (chart) => {
			const ctx = chart.canvas.getContext('2d');
			ctx.save();
			ctx.globalCompositeOperation = 'destination-over';
			ctx.fillStyle = 'rgba(0,0,0,0)'; // set background to transparent
			ctx.fillRect(0, 0, chart.width, chart.height);
			ctx.restore();
		}
	};
	let plugins=[plugin];
	
	$: data = {
		datasets: 
			$plot_data.hasOwnProperty(id) ? 
				$plot_data[id].map((element:DataSet,index:number) => {
					return {
						borderColor: colors[index], // outline color for the data point
						backgroundColor: colors[index], // color used for the line and data point fill
						borderWidth: 2, // Set width of line
						pointBorderWidth: 0, // Set width of outline of data point
						pointRadius: 2, // setting to 0 will make the data point not render
						pointStyle: 'rectRounded', // 'circle' 'cross' 'crossRot' 'dash' 'line' 'rect' 'rectRounded' 'rectRot' 'star' 'triangle'
						label: element.label,
						showLine: true,
						data: element.data
					}
				})
			: []
	};
	
	$: options = {
		title: {
			display: true,
			text: title
		},
		scales: {
			x: {
				type: 'linear',
				title: {
					text: xlabel,
					display: true,
					color: $dark ? darkModeText : lightModeText, // color of text
				},
				ticks: {
					callback: function (value,index,ticks) {
						return value.toString().replace(/,/g, '');
					},
					color: $dark ? darkModeText : lightModeText, // color of tick text
				},

				grid: {
					borderColor: $dark ? darkModeText : lightModeText,// color of horizontal border line on x-axis side
					color: $dark ? darkModeText : lightModeText, // color of vertical grid line
				}
			},
			y: {
				type: 'linear',
				title: {
					text: ylabel,
					display: true,
					color: $dark ? darkModeText : lightModeText, // color of text
				},
				ticks: {
					color: $dark ? darkModeText : lightModeText, // color of tick text
				},

				grid: {
					borderColor: $dark ? darkModeText : lightModeText, // color of vertical border line on y-axis side
					color: $dark ? darkModeText : lightModeText, // color of horizontal grid line
				}
			}
		},
		plugins: {
			legend: {
				labels: {
					boxWidth: 15,
					usePointStyle: true,
					color: $dark ? darkModeText : lightModeText, // color of legend text
				}
			},
			tooltip: {
				callbacks: {
					title: (context) => {
						return context[0].raw.x;
					},
					label: function(context) {
						var label = context.dataset.label || '';
						
						if (label) {
							label += ': ';
						}
						if (context.parsed.y !== null) {
							label += new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(context.parsed.y);
						}
						return label;
					}
				}
			}
		}
	};

</script>

{#if $plot_data.hasOwnProperty(id)}
	<Scatter data={data} options={options} {plugins} class="bg-slate-100 dark:bg-slate-800"/>
{/if}