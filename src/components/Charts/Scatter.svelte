<script lang="ts">
    import { onMount, afterUpdate } from "svelte";
    import * as d3 from "d3";

    import type { Point, PointArray, PointLimit } from "./chart.type";
    import type { ChartFormat } from "./xyChart.type";
    import type { BoundInput, DataInput } from "./scatter.type";
    import {NumberFormat, toFormattedString} from "./chart"
    import {XyChart} from "./xyChart";

    import {graphics} from "../../../tailwind_colors_graphics";
        
    export let data: DataInput | null;
    export let bounds: BoundInput | null;
    export let xLabel: string;
    export let yLabel: string;
    export let title: string="";

    export let domain: PointLimit<number, number> = {
        x: {min:null,max:null},
        y: {min:null,max:null},
    }

    export let format: ChartFormat = {
        box: {
            width : 150,
            height : 120
        },
        title: {
            fontSize : 5,
            paddingBottom : 8,
            translate : {x:0, y:0},
        },
        axis: {
            tick: {
                fontSize : 3,
                padding : 2,
                size : 2,
            },
            label: {
                fontSize: 4,
            },
            strokeWidth: 0.3,
            format: {
                x: NumberFormat.Decimal,
                y: NumberFormat.Decimal,
            }
        },
        tooltip: {
            circleRadius : 1,
            circleStrokeWidth : 0.4,
            borderWidth : 1,
            borderRadius : 2,
            padding : 4,
            format : {
                x: NumberFormat.Decimal,
                y: NumberFormat.Decimal,
            }
        },
        legend: {
            fontSize : 2,
            translate : {x:2, y:2},
        },
        lineStrokeWidth : 0.6,
        dataPointSize : .3,
        fontAR : 0.5,
    };

    let wr;
    let el;
    
    let imageWidth;
    let imageHeight;

    const colors = graphics;

    // Calculated chart dimensions

    // Combine the datasets from 'data' and 'bounds' as they may have different ranges
    let fullDataset: PointArray<number, number> = [];
    
    if (data) {
        fullDataset = [...fullDataset, ...data.points]
    };
    if (bounds) {
        fullDataset = [...fullDataset, ...bounds.points]
    };

    const epsilon = bounds ? {
        x: bounds.radius.x,
        y: bounds.radius.y
    } : {x:0, y:0};


    function makeChart() {
        let chart = new XyChart(xLabel, yLabel, title, format, fullDataset, domain, el);
        chart.addAxis();

        var hideTooltip = () => {
            tooltip.style("opacity", 0);
            tooltipCircle.style("opacity", 0);
        }

        // create a tooltip
        var tooltip = d3.select(wr)
            .append("div")
            .style("opacity", 0)
            .style("position", "absolute")
            .style("border", "solid")
            .style("border-width", `${format.tooltip.borderWidth}px`)
            .style("border-radius", `${format.tooltip.borderRadius}px`)
            .style("padding", `${format.tooltip.padding}px`)
            .attr("class","bg-background-500")

        var onPointerMove = (event) => {
            event.preventDefault();

            const pointer: Point<number, number> = {"x":d3.pointer(event)[0], "y":d3.pointer(event)[1]};

            // Get the x location of the mouse (in terms of x axis value)
            // If x-axis is rendered rendered as a time series, x.invert returns a JS Date object which must 
            // be converted to epoch in seconds to match the incoming data.
            const hover: Point<number, number> = {"x":chart.x.invert(pointer.x), "y":chart.y.invert(pointer.y)};

            // Calculate the distance between input x value and the mouse hover x value
            const getDistanceFromHover = (d: Point<number, number>) => Math.sqrt(Math.pow((d.x - hover.x),2) + Math.pow((d.y - hover.y),2));

            // Scan through all the input data and find the closest x value
            const closestIndex = d3.scan(
                data.points,
                (a: Point<number, number>, b: Point<number, number>) => getDistanceFromHover(a) - getDistanceFromHover(b)
            );
            const closestDataPoint = data.points[closestIndex];

            // Drop out of callback if no closest point was found
            if (closestDataPoint === undefined) {
                return
            }

            // Get the current scale factors for the svg image (as the svg rendered size is reactive on the page)
            const scale: Point<number, number> = {"x":imageWidth/format.box.width, "y":imageHeight/format.box.height};

            // Calculate the position for the tooltip in pixels relative to the wrapper div
            const tooltipPx : Point<number, number> = {
                "x": scale.x * (chart.x(closestDataPoint.x) + chart.margin.left + 0.7071*format.tooltip.circleRadius),
                "y": scale.y * (chart.y(closestDataPoint.y) + chart.margin.top + 0.7071*format.tooltip.circleRadius)
            };

            tooltip
                .html(`${toFormattedString(closestDataPoint.x, format.tooltip.format.x)}, ${toFormattedString(closestDataPoint.y, format.tooltip.format.y)}`)
                .style("left", tooltipPx.x+"px")
                .style("top", tooltipPx.y+"px")
                .style("opacity", 1);
            
            tooltipCircle
                .attr("cx", chart.x(closestDataPoint.x))
                .attr("cy", chart.y(closestDataPoint.y))
                .style("opacity", 1);
            
        }


        if (bounds) {
            // If there are no groups defined
            if (!bounds.groups) {
                bounds.groups = bounds.points.map(() => 0)
            }

            // Get a list of unique bounds labels
            let uniqueGroups = [...new Set(bounds.groups)];

            // For each bounds group
            uniqueGroups.forEach((groupIndex) => {
                
                // Filter all bounds points to only points in this group
                let boundsSubset = bounds.points.filter((_v:Point<number, number>, index:number) => bounds.groups[index]===groupIndex)
                
                // Add bounds outline as an ellipse with a stroked outline
                chart.chartGroup
                    .append("g")
                    .selectAll(`ellipseOutline-${groupIndex}`)
                    .data(boundsSubset)
                    .join("ellipse")
                        .attr("cx", (d: Point<number, number>) => chart.x(d.x))
                        .attr("cy", (d: Point<number, number>) => chart.y(d.y))
                        // Here we need to convert epsilon (in data units) to pixel units so we use the axis functions
                        // In addition we want a radius value so we take the differenece from the pixel value at 0 on the axis
                        // Also note that the y direction is inverted so we do y(0) - y(epsilon)
                        .attr("rx", () => chart.x(epsilon.x)-chart.x(0))
                        .attr("ry", () => chart.y(0)-chart.y(epsilon.y))
                        .attr("stroke-width", format.lineStrokeWidth)
                        .attr("stroke", () => `${colors[groupIndex+1][300]}`)

                // Add bounds ellipse as a filled shape (which covers up the stroke lines from above within the bounds boundary)
                chart.chartGroup
                    .append("g")
                    .selectAll(`ellipse-${groupIndex}`)
                    .data(boundsSubset)
                    .join("ellipse")
                        .attr("cx", (d: Point<number, number>) => chart.x(d.x))
                        .attr("cy", (d: Point<number, number>) => chart.y(d.y))
                        .attr("rx", () => chart.x(epsilon.x)-chart.x(0))
                        .attr("ry", () => chart.y(0)-chart.y(epsilon.y))
                        .style("fill", () => `${colors[groupIndex+1][50]}`)

            })
        }

        const tooltipCircle = chart.chartGroup
                .append("circle")
                .attr("class", "tooltip-circle")
                .attr("r", format.tooltip.circleRadius)
                .attr("stroke-width", format.tooltip.circleStrokeWidth)
                .attr("fill", "rgba(0,0,0,0)")
                .attr("class", "stroke-graphics-1-500")
                .style("opacity", 0);

        if (data) {
            let size = data.points.length < 50 ? format.dataPointSize*2 : format.dataPointSize;
            // Add points
            chart.chartGroup
                .append("g")
                .selectAll("dot")
                .data(data.points)
                .join("circle")
                    .attr("cx", (d: Point<number, number>) => chart.x(d.x))
                    .attr("cy", (d: Point<number, number>) => chart.y(d.y))
                    .attr("r", size)
                    .style("fill", (_d: Point<number, number>, index:number) => data.groups ? `${colors[data.groups[index]+1][500]}` : `${colors[1][500]}`)

            // Add rectangle for background action listening area
            chart.chartGroup
                .append("rect")
                .attr("class", "listening-rect")
                .attr("width", chart.width)
                .attr("height", chart.height)
                .attr("fill", "rgba(0,0,0,0)")
                .on("pointermove", onPointerMove)
                .on("pointerout", hideTooltip);
        }
    }

    onMount(() => {
        makeChart();
    });

    afterUpdate(() => {
        // Remove all existing svg elements
        d3.select(el).selectAll('*').remove();
        makeChart();
    })

</script>

<div bind:this={wr} bind:clientWidth={imageWidth} bind:clientHeight={imageHeight} class="overflow-visible relative">
    <svg bind:this={el}></svg>
</div>