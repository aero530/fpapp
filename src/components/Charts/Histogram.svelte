<script lang="ts">
	import * as d3 from "d3";
	import { onMount, createEventDispatcher } from "svelte";
	import { v4 as uuidv4 } from 'uuid';
	
	import type {Histogram, Point, PointLimit, Limit} from "./chart.type";
	import type { ChartFormat } from "./xyChart.type";
	import {NumberFormat} from "./chart"
	import {XyChart} from "./xyChart";
	
	export let editable: boolean = false;
	export let input: Histogram;
	export let limit: Limit<number> = { // In data units
		min: Number.NaN,
		max: Number.NaN
	};
	export let xLabel: string;
	
	export let domain: PointLimit<number, number> = {
        x: {min:null, max:null},
        y: {min:0, max:null},
    }
	
	// px value of how 
	export let limitOffset = {
		x: 15, // max width the limit bars are allowed to be in px
		y: 1.5
	};
	
	export let format: ChartFormat = {
		box: {
			width : 150,
			height : 100,
		},
		title: {
			fontSize : 5,
			paddingBottom : 8,
			translate : {x:0, y:0},
		},
		axis: {
			tick: {
				fontSize: 3,
				padding: 2,
				size: 2,
			},
			label: {
				fontSize: 4
			},
			strokeWidth: 0.3,
			format: {
				x: NumberFormat.Decimal,
				y: NumberFormat.Decimal,
			},
		},
		tooltip: {
			circleRadius : 1,
			circleStrokeWidth : .4,
			borderWidth : 1,
			borderRadius : 2,
			padding : 5,
			format : {
				x: NumberFormat.Decimal,
				y: NumberFormat.Decimal,
			}
		},
		lineStrokeWidth : 0.3,
		dataPointSize : 0,
		fontAR : 0.5,
	};
	
	
	const dispatch = createEventDispatcher();
	const dispatchLimitUpdate = (updatedLimit:Limit<number>) => dispatch('updateLimit',{"limit": updatedLimit});

	let wr;
	let el;
	
	let imageWidth;
	let imageHeight;
	
	// A uuid for the chart is needed so the svg clipping paths have a unique name & don't conflict with paths from other svgs
	const chartId = uuidv4();
	let update = null;

	/// Function to allow the limit value to be updated from outside this component without re-rendering the entire thing.
	export const updateLimit = (newLimit:Limit<number>) => {
		limit = {min:newLimit.min, max:newLimit.max}
		update();
	};

	onMount(() => {

		const inputData = input.bins.map((bin) => {
			return {"x":bin.center, "y":bin.count}
		});

		let chartData = [];
		input.bins.forEach((bin) => {
			chartData.push({"x":bin.min, "y":bin.count})
			chartData.push({"x":bin.max, "y":bin.count})
		});
		
		let counts = input.bins.map(bin => bin.count);
		let displayDomain : PointLimit<number, number> = {
			x:{
				min: input.bins[0].min,
				max: input.bins[input.bins.length-1].max
			},
			y:{
				min: 0,
				max: d3.max(counts)
			},
			
		};
		
		let chart = new XyChart(xLabel, "", "", format, chartData, domain, el);
		
		d3.select(`#axisMinInput-${chartId}`).on("change", (e) =>  handleTextBoxLimit(e,"lower"));
		d3.select(`#axisMaxInput-${chartId}`).on("change", (e) =>  handleTextBoxLimit(e,"upper"));

		// If a limit is not defined then use the min/max value from the input data
		displayDomain.x.min = (isNaN(limit.min)) ? displayDomain.x.min : limit.min;
		displayDomain.x.max = (isNaN(limit.max)) ? displayDomain.x.max : limit.max;
		
		// calculate the linear scale of the data in data units per display unit (px)
		// [data units per px]
		let scaleX = (displayDomain.x.max-displayDomain.x.min) / (chart.width-2*limitOffset.x);
		let scaleY = (displayDomain.y.max-displayDomain.y.min) / (chart.height);
		
		// Compute chart domain based on limit bars
		domain = {
			x: {
				min: displayDomain.x.min-limitOffset.x*scaleX,
				max: displayDomain.x.max+limitOffset.x*scaleX
			},
			y: { 
				min: displayDomain.y.min,
				max: displayDomain.y.max+format.lineStrokeWidth*scaleY
			},
		}
		
		chart.domain = domain;
		chart.addAxis();
		
		update = () => {
			updateDomain(limit);
			updateChart();
		};

		// Update the domain of the chart (zoom the x axis)
		// This is based on the fingerprint limits instead of the whole chart domain so that 
		// the limit values stay in the same place on the chart relative to the limit rectangles
		// and the corresponding x value of the histogram at that point.
		function updateDomain(newLimit: Limit<number>) {
			displayDomain.x.min = (isNaN(newLimit.min)) ? input.bins[0].min : newLimit.min;
			displayDomain.x.max = (isNaN(newLimit.max)) ? input.bins[input.bins.length-1].max : newLimit.max;

			scaleX = (displayDomain.x.max - displayDomain.x.min) / (chart.width-2*limitOffset.x);
		
			let domainMin = displayDomain.x.min - limitOffset.x*scaleX;
			let domainMax = displayDomain.x.max + limitOffset.x*scaleX;
			
			chart.x.domain([domainMin, domainMax]);
			chart.domain.x.min = domainMin;
			chart.domain.x.max = domainMax;
			chart.chartXaxis.transition().duration(10).call(chart.xAxis);
		}
		
		// Rerender the histograms
		function updateChart() {
			// Rerender the histograms
			chart.chartGroup.selectAll("path")
				.datum(inputData)
				.transition()
				.duration(10)
				.attr("d", histogramArea)

			chart.chartGroup
				.selectAll(`#limit-rect-upper-${chartId}`)
				.attr("opacity",() => {return isNaN(limit.max) ? 0 : 0.3});
			chart.chartGroup
				.selectAll(`#limit-rect-lower-${chartId}`)
				.attr("opacity",() => {return isNaN(limit.min) ? 0 : 0.3});	
		}
		
		// Handle changing the limit from the input text box
		function handleTextBoxLimit(event, d: string) {
			let newLimit : Limit<number> = limit;
			let valueInput = parseFloat(event.srcElement.value);
			
			// Compute the new min and max values for the limit rectangle
			if (d === "lower") {
				newLimit.min = valueInput;
			} else {
				newLimit.max = valueInput;
			}
			newLimit.min = newLimit.min > (limit.max-.1) ? (limit.max-.1) : newLimit.min;
			newLimit.max = newLimit.max < (limit.min+.1) ? (limit.min+.1) : newLimit.max;

			updateDomain(newLimit);
			updateChart();
			limit = newLimit;
		}
		
		// When drag starts change the cursor style back to closed hand (grabbing)
		function dragstarted() {
			if (editable) {
				chart.chartGroup.attr("cursor", "grabbing");
			} else {
				chart.chartGroup.attr("cursor", "auto");
			}
		}
		
		// While dragging update the limit rectangle, axis domain, and histogram display
		function dragged(event, d: string) {
			// Only act on a drag event if the chart is editable AND there is a limit on this half of the chart.
			if (editable && (d==="lower" ? !isNaN(limit.min) : !isNaN(limit.max))) {
				let newLimit : Limit<number> = {
					min: (isNaN(limit.min)) ? displayDomain.x.min : limit.min,
					max: (isNaN(limit.max)) ? displayDomain.x.max : limit.max
				};
				
				// Compute the drag change in x in sensor units event.dx gives a pixel movement value which must be converted to
				// axis scale units then we subtract x.invert(0) (where 0 is in px) to account for the axis scale not starting at 0.
				let deltaX = chart.x.invert(0)-chart.x.invert(event.dx);
				
				// Compute the new min and max values for the limit rectangle
				if (d === "lower") {
					newLimit.min = newLimit.min+deltaX;
				} else {
					newLimit.max = newLimit.max+deltaX;
				}
				
				newLimit.min = newLimit.min > (limit.max-.1) ? (limit.max-.1) : newLimit.min;
				newLimit.max = newLimit.max < (limit.min+.1) ? (limit.min+.1) : newLimit.max;

				
				updateDomain(newLimit)
				updateChart();

				newLimit.min = (isNaN(limit.min)) ? Number.NaN : newLimit.min;
				newLimit.max = (isNaN(limit.max)) ? Number.NaN : newLimit.max;
				
				limit = newLimit;

			}
		}
		
		// When drag ends change the cursor style back to open hand (grab)
		function dragended() {
			if (editable) {
				dispatchLimitUpdate(limit);
				chart.chartGroup.attr("cursor", "grab");
			} else {
				chart.chartGroup.attr("cursor", "auto");
			}
		}
		
		// Define the histogram area.  We actually display 3 copies of this image.  The first 
		// is the base histogram image.  The second two are clipped versions that are used to show
		// the highlighted regions (on either side of the histogram).
		let histogramArea = d3.area()
			.curve(d3.curveStep) // curveMonotoneX curveBumpX curveStep 
			.x((d:Point<number, number>) => chart.x(d.x))
			.y0(chart.y(0))
			.y1((d:Point<number, number>) => chart.y(d.y));
		
		// Create a dataset used to make a 'min' & 'max'.  This allows code reuse for making the min & max versions
		let limitData = {
			"lower": {"min": 0, "max": limitOffset.x}, // lower limit on left side of chart
			"upper": {"min": chart.width-limitOffset.x, "max": chart.width}, // upper limit on rigth side of chart
		};
		
		// Create the clip paths that will be used to clip a copy of the histogram to show in a highlight color
		chart.chartGroup
			.selectAll("clipRect")
			.data(Object.keys(limitData))
			.join("clipPath")
			.attr("id", (d: string) => `clip-rect-${d}-${chartId}`)
			.append("rect")
			.attr("x", (d: string) => limitData[d].min)
			.attr("y", 0)
			.attr("height", chart.height)
			.attr("width", (d: string) => (limitData[d].max-limitData[d].min))
		
		// Create a clip path that will be applied to the base histogram data.  This is to ensure the 
		// histogram visualization does not extend past the bounds (domain) of the chart
		chart.chartGroup
			.append("clipPath")
			.attr("id", () => `clip-rect-full-${chartId}`)
			.append("rect")
			.attr("x", () => limitData.lower.max)
			.attr("y", 0)
			.attr("height", chart.height)
			.attr("width", () => (limitData.upper.min-limitData.lower.max))
		
		// Add the base histogram image
		chart.chartGroup
			.append("path")
			.datum(inputData)
			.classed("fill-background-700",true)
			.classed("stroke-background-800",true)
			.attr("stroke-width", format.lineStrokeWidth)
			.attr("d", histogramArea)
			.attr("clip-path", () => `url(#clip-rect-full-${chartId})`)
		
		// Create the clipped hisograms and link to the clip path made above by id
		chart.chartGroup
			.selectAll("clippedHistogram")
			.data(Object.keys(limitData))
			.join("path")
			.attr("id", (d: string) => `clipped-histogram-${d}`)
			.attr("clip-path", (d: string) => `url(#clip-rect-${d}-${chartId})`)
			.datum(inputData)
			.attr("d", histogramArea)
			.classed("fill-graphics-4-400",true)
			.classed("stroke-graphics-4-800",true)
			.attr("stroke-width", format.lineStrokeWidth)
		
		// Create the overlay rectangle to show the min/max limits.  Include calls to enable user interaction on drag.
		chart.chartGroup
			.selectAll("limitRect")
			.data(Object.keys(limitData))
			.join("rect")
			.attr("id", (d: string) => `limit-rect-${d}-${chartId}`)
			.attr("x", (d: string) => limitData[d].min)
			.attr("y", 0)
			.attr("height", chart.height)
			.attr("width", (d: string) => (limitData[d].max-limitData[d].min))
			.attr("stroke-width", 0)
			.attr("opacity", .3)
			.classed("fill-secondary-100",true)
		
		// Create overlay rectangles to allow dragging in the data window
		chart.chartGroup
			.selectAll("dragRect")
			.data(Object.keys(limitData))
			.join("rect")
			.attr("id", (d: string) => `drag-rect-${d}-${chartId}`)
			.attr("x", (d: string) => d==='lower' ? 0 : chart.width/2)
			.attr("y", 0)
			.attr("height", chart.height)
			.attr("width", (d: string) => chart.width/2)
			.attr("stroke-width", 0)
			.attr("opacity", 0)
			.call(d3.drag()
			.on("start", dragstarted)
			.on("drag", dragged)
			.on("end", dragended))
	});
	
</script>

<div bind:this={wr} bind:clientWidth={imageWidth} bind:clientHeight={imageHeight} class="overflow-visible relative">
	<div class="h-fit grid grid-cols-2 pt-2">
		<div class="flex flex-wrap place-content-start ml-4">
			<div 
				class="align-middle mr-2 grid content-center"
				class:hidden={!editable && isNaN(limit.min)}
			>
				<div>Lower Limit</div>
			</div>
			<input 
				class="w-22"
				class:hidden={isNaN(limit.min)}
				type="number"
				id={`axisMinInput-${chartId}`}
				bind:value={limit.min}
				disabled={editable?false:true}
				on:change={() => dispatchLimitUpdate(limit)}
			>
			<button 
				class="bg-primary-500 hover:bg-primary-400 disabled:bg-secondary-100 px-2 py-1 mx-2 my-1 text-light font-semibold rounded"
				class:hidden={!editable}
				disabled={!editable}
				on:click={() => {
					limit.min = (isNaN(limit.min)) ? input.bins[0].min : Number.NaN;
					dispatchLimitUpdate(limit);
					update();
				}}
			>
				{(isNaN(limit.min)) ? "Add" : "Remove"}
			</button>
		</div>

		<div class="flex flex-wrap place-content-end mr-4">
			<div
				class="align-middle mr-2 grid content-center"
				class:hidden={!editable && isNaN(limit.max)}
			>
				<div>Upper Limit</div>
			</div>
			<input
				class="w-22"
				class:hidden={isNaN(limit.max)}
				type="number"
				id={`axisMaxInput-${chartId}`}
				bind:value={limit.max}
				disabled={editable?false:true}
				on:change={() => dispatchLimitUpdate(limit)}
			>
			<button 
				class="bg-primary-500 hover:bg-primary-400 disabled:bg-secondary-100 px-2 py-1 mx-2 my-1 text-light font-semibold rounded"
				class:hidden={!editable}
				disabled={!editable}
				on:click={() => {
					limit.max = (isNaN(limit.max)) ? input.bins[input.bins.length-1].max : Number.NaN;
					dispatchLimitUpdate(limit);
					update();
				}}

			>
				{(isNaN(limit.max)) ? "Add" : "Remove"}
			</button>
		</div>
	</div>
	<div class="h-full w-full">
		<svg bind:this={el}></svg>
		
	</div>
</div>