<script lang="ts">
    import { onMount, afterUpdate } from "svelte";
    import * as d3 from "d3";
    
    import type {Point, PointArray, PointLimit} from "./chart.type";
    import type { ChartFormat } from "./xyChart.type";
    import {NumberFormat, toFormattedString} from "./chart"
    import {XyChart} from "./xyChart";

    import {theme} from "../../../tailwind.config";

    type xType = Date | number;
    type yType = number;
    type DataSet = {data: PointArray<xType, yType>, label: string};

    // data can be an array of data sets or an array of points (a single data set)
    export let data: Array<DataSet> | PointArray<xType, yType>;
    export let title: string="";
    export let xLabel: string="";
    export let yLabel: string="";

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
        lineStrokeWidth : 0.6,
        dataPointSize : 0,
        fontAR : 0.5,
    };

    export let domain: PointLimit<xType, yType> = {
        x: {min:null,max:null},
        y: {min:null,max:null},
    }

    let dataSets: Array<DataSet>
    // If data was input as a single dataset then put into an array of DataSets
    if ("x" in data[0]) {
        dataSets = [{"data": data, "label": yLabel}];
    } else {
        dataSets = data;
    }

    // Make an array with all the data points to find the extents of the data
    let fullDataset: PointArray<xType, yType> = [];
    dataSets.forEach(element => {
        fullDataset = [...fullDataset, ...element.data];
    });

    let wr;
    let el;
    
    let imageWidth:number;
    let imageHeight:number;

    const colors = theme.colors.graphics;

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
            const hover: Point<xType, number> = {"x":chart.x.invert(pointer.x), "y":chart.y.invert(pointer.y)};

            // Precompute the span of data on each axis.  This is used to scale the distance computation thereby
            // accounting for differences in the x and y range of data.  The user really wants the distance calcuation
            // to effectively be how many pixels is the data point from the hover point but since hover is in 'data' units
            // not pixels, we need to scale that back down.
            //
            //
            // To do -- 
            // I think there should also be an extra scale factor to x or yScale to account for the aspect ratio of the image.
            let xScale = (chart.domain.x.max instanceof Date ? chart.domain.x.max.getTime() : chart.domain.x.max) - (chart.domain.x.min instanceof Date ? chart.domain.x.min.getTime() : chart.domain.x.min)
            let yScale = (chart.domain.y.max - chart.domain.y.min);

            // Calculate the distance between input x value and the mouse hover x value
            const getDistanceFromHoveredDate = (d: Point<xType, yType>) => {
                if (typeof d.x==="number" && typeof hover.x==="number") {
                    return Math.sqrt(Math.pow((d.x - hover.x)/xScale,2) + Math.pow((d.y - hover.y)/yScale,2))
                } else if (d.x instanceof Date && hover.x instanceof Date) {
                    return Math.sqrt(Math.pow((d.x.getTime() - hover.x.getTime())/xScale,2) + Math.pow((d.y - hover.y)/yScale,2));
                }
            };

            // Scan through all the input data and find the closest x value
            const closestIndex = d3.minIndex(
                fullDataset,
                (a: Point<xType, yType>) => {return getDistanceFromHoveredDate(a)}
            );
            const closestDataPoint = fullDataset[closestIndex];
            
            // Get the current scale factors for the svg image (as the svg rendered size is reactive on the page)
            const scale: Point<number, number> = {"x":imageWidth/format.box.width, "y":imageHeight/format.box.height};

            // Calculate the position for the tooltip in pixels relative to the wrapper div
            const tooltipPx : Point<xType, yType> = {
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

        const chartGroup = chart.chartGroup;

        // Add line
        dataSets.forEach((dataSet,index) => {
            chartGroup
                .append("path")
                .datum(dataSet.data)
                .attr("id",`data-line-${index}`)
                .attr("fill", "none")
                .attr("stroke", () => `${colors[index+1][500]}`)
                .attr("stroke-width", format.lineStrokeWidth)
                .attr("d", d3.line()
                    // .curve(d3.curveBumpX)
                    .x((d:Point<xType, yType>) => chart.x(d.x))
                    .y((d:Point<xType, yType>) => chart.y(d.y))
                )
        })
        

        // Add circle to highlight data point
        const tooltipCircle = chartGroup
            .append("circle")
            .attr("id", "tooltip-circle")
            .attr("r", format.tooltip.circleRadius)
            .attr("stroke-width", format.tooltip.circleStrokeWidth)
            .attr("class", "stroke-graphics-1-500 fill-light")
            .style("opacity", 0);

        // Add rectangle for background action listening area
        chartGroup
            .append("rect")
            .attr("id", "listening-rect")
            .attr("width", chart.width)
            .attr("height", chart.height)
            .attr("fill", "rgba(0,0,0,0)")
            .on("pointermove", onPointerMove)
            .on("pointerout", hideTooltip);

    };

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