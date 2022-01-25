<script lang="ts">
	import Scatter from "svelte-chartjs/src/Scatter.svelte";
	
	type Point = {x: number, y:number};
	type DataSet = { data: Point[], label: string;};
	
	export let inputdata : DataSet[];
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
	
	let data = {
		datasets: inputdata.map((element,index) => {
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
	};
	
	let options = {
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
					color: 'rgba(255,255,255, 1)', // color of text
				},
				ticks: {
					callback: function (value,index,ticks) {
						return value.toString().replace(/,/g, '');
					},
					color: 'rgba(255,255,255, 1)', // color of tick text
				},

				grid: {
					borderColor: 'rgba(255,255,255, 1)',// color of horizontal border line on x-axis side
					color: 'rgba(255,255,255, 1)', // color of vertical grid line
				}
			},
			y: {
				type: 'linear',
				title: {
					text: ylabel,
					display: true,
					color: 'rgba(255,255,255, 1)', // color of text
				},
				ticks: {
					color: 'rgba(255,255,255, 1)', // color of tick text
				},

				grid: {
					borderColor: 'rgba(255,255,0, 1)', // color of vertical border line on y-axis side
					color: 'rgba(0,0,255, 1)', // color of horizontal grid line
				}
			}
		},
		plugins: {
			legend: {
				labels: {
					boxWidth: 15,
					usePointStyle: true,
					color: 'rgba(255,255,255, 1)', // color of legend text
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

<Scatter {data} options={options} {plugins}/>
