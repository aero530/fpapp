<script lang="ts">
    
    import type { ChartFormat } from "./box.type";
    
    import type { Dispersion, Point, Limit } from "./chart.type";
    import * as d3 from "d3";
    import { onMount, afterUpdate } from "svelte";

    export let input: Dispersion;
    export let limit: Limit<number> = {
        min: Number.NaN,
        max: Number.NaN
    };

    export let format: ChartFormat = {
        box: {
            width : 100,
            height : 10
        },
        lineStrokeWidth : 0.3,
        limitOffset : {
            x: 15,
            y: 1.5
        },
    };

    afterUpdate(() => {
        // Remove all existing svg elements
        d3.select(el).selectAll('*').remove();
        makeChart();
    });

    let wr;
    let el;
    
    let imageWidth;
    let imageHeight;

    function makeChart() {

        const margin = {
            top: format.lineStrokeWidth,
            right: format.lineStrokeWidth,
            bottom: format.lineStrokeWidth,
            left: format.lineStrokeWidth,
        }
        const width = format.box.width - margin.left - margin.right;
        const height = format.box.height - margin.top - margin.bottom - 2*format.limitOffset.y;

        // Determine if upper and lower limits have been set and are different from the other extreme
        const hasLowerLimit = !isNaN(limit.min) && limit.min !== limit.max;
        const hasUpperLimit = !isNaN(limit.max) && limit.min !== limit.max;

        // If a limit is not defined then use the min/max value from the input data
        let localMin = isNaN(limit.min) ? input.q0 : limit.min*1;
        let localMax = isNaN(limit.max) ? input.q4 : limit.max*1;

        // Calculate the linear scale of the data in sensor units per display unit
        const scale = (localMax-localMin) / (width-2*format.limitOffset.x);

        // Compute the min and max values for the axis in sensor units
        const xMin = hasLowerLimit ? localMin-format.limitOffset.x*scale : input.q0;
        const xMax = hasUpperLimit ? localMax+format.limitOffset.x*scale : input.q4;

        const chartTranslate:Point<number,number> = {x:margin.left, y:margin.top+format.limitOffset.y};

        // append the svg object to the div called "chart"
        const svg = d3.select(el)
            .attr("preserveAspectRatio", "xMidYMid meet")
            .attr("viewBox", "0 0 "+format.box.width+" "+format.box.height)
            .classed("svg-content", true);
        
        
        var chartGroup = svg.append("g")
            .attr("transform", `translate(${chartTranslate.x},${chartTranslate.y})`);
            var overlayGroup = svg.append("g");
        // Define x-axis
        var x = d3.scaleLinear()
            .domain([xMin,xMax])
            .range([0, width]);

        // Add transparent limit overlay
        if (hasLowerLimit) {
            overlayGroup
                .append("rect")
                .attr("x", x(xMin))
                .attr("y", margin.top )
                .attr("height", height + 2*format.limitOffset.y)
                .attr("width", (x(limit.min) + x(xMin)) )
                .attr("stroke-width", 0)
                .attr("opacity", .5)
                .classed("fill-secondary-50",true)
                .classed("fill-graphics-7-100", input.q0 < limit.min)
        }
        if (hasUpperLimit) {
            overlayGroup
                .append("rect")
                .attr("x", x(limit.max))
                .attr("y", margin.top )
                .attr("height", height + 2*format.limitOffset.y)
                .attr("width", (x(limit.max) + x(xMax)) )
                .attr("stroke-width", 0)
                .attr("opacity", .5)
                .classed("fill-secondary-50",true)
                .classed("fill-graphics-7-100", input.q4 > limit.max)
        }

        // Add main horizontal line
        chartGroup
            .append("line")
            .attr("x1",x(input.q0))
            .attr("x2",x(input.q4))
            .attr("y1",height/2)
            .attr("y2",height/2)
            .attr("stroke-width", format.lineStrokeWidth)
            .classed("stroke-dark",true)

        // Add filled in rectangle
        chartGroup
            .append("rect")
            .attr("x", x(input.q1))
            .attr("y", 0 )
            .attr("height", height)
            .attr("width", (x(input.q3) - x(input.q1)) )
            .attr("stroke-width", format.lineStrokeWidth)
            .classed("stroke-dark",true)
            .classed("fill-secondary-100",true)

        // show median, min and max horizontal lines
        chartGroup
            .selectAll("lines")
            .data([input.q0, input.q2, input.q4])
            .enter()
            .append("line")
            .attr("x1", (d) => x(d))
            .attr("x2", (d) => x(d))
            .attr("y1", 0 )
            .attr("y2", height )
            .attr("stroke-width", format.lineStrokeWidth)
            .classed("stroke-dark",true)
    }

    onMount(() => {
        makeChart();
    });
</script>

<div bind:this={wr} bind:clientWidth={imageWidth} bind:clientHeight={imageHeight} class="overflow-visible relative">
    <svg bind:this={el}></svg>
</div>