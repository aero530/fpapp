<!-- 
This visualization shows a single metric over time.  It is split into three sections (top, middle, bottom).
The top (timeline) section is a one dimensional histogram of the data including two selectors to control the zoom 
scale of the bottom (detail) section.  The middle section shows a sweepy connection between the selection on the timeline 
and the data shown in the detail section.
-->

<script lang="ts">
    
    import type { ChartFormat, EventData } from "./event.type";
    import type { Point, PointLimit, Limit } from "./chart.type";
    import * as d3 from "d3";
    import { onMount } from "svelte";
	import { v4 as uuidv4 } from 'uuid';
    import { timestampToDateInput } from "../../time";

    export let events: EventData<Date>;
    export let title: string="";
    /// Number of days to include as the total span for linking out to power search results.
    /// The dots in the events chart link out to allow a user to see a table of all the anomalies that make up that dot.
    /// The dots are intended to be binned daily (or something of the sort) from the user-api so when transferring this
    /// to a power search query we need min and max timestamp to filter with.  We use the date the event is centered on
    /// + and - the linkDomainSpan/2 [as defined in number of days].  So setting this value to 1 will result in a 
    /// power search query that spans a total of 24 hours centered on the timestamp from the event dot.
    export let linkDomainSpan: number;
    /// Customer number to use for link to power search results
    export let customerNumber: string="";
    
    export let format: ChartFormat = {
        box: {
            width : 200,
            height : 55
        },
        detail: {
            height: 15,
            circleMinDiameter: 2,
            circleHoverStroke: 1,
            strokeWidth: 0.2,
            padding: 14,
            initialDayRange: 7,
            tooltipFontSize: 4,
            tickSize: 2,
            tickPadding: 2,
            tickCount: 6,
            tickStrokeWidth: .2,
            axisFontSize: 3,
        },
        timeline: {
            height: 5,
            outlineStrokeWidth: 0.08,
            selectionHeight: 3,
            binCount: 80,
            selectorWidth: 5,
            selectorRadius: 2.5,
            horizInset: 14,
            dayPadding: 7,
            minDateRange: 3,
            axisFontSize: 3,
            tickPadding: 1,
            tickSize: 1,
        },
        title: {
            height: 5,
            fontSize: 5,
        }
    };
    
    let wr;
    let el;
    
    let imageWidth;
    let imageHeight;
    
    enum Side {
        min="min",
        max="max"
    }

    // Add a set number of days to the input Date
    function offsetDateByDays(date:Date, days:number) {
        return new Date( date.getTime() + 86400000*days)
    }
    
    // A uuid for the chart is needed so the svg clipping paths have a unique name & don't conflict with paths from other svgs
	const chartId = uuidv4();

    onMount(() => {
        
        // Show visual effects of the tooltip when hovering over the detail circle
        let showTooltip = (event, d:Point<Date, number>) => {
            event.preventDefault();

            tooltip
                .text(`${d.y}`)
                .attr("x", xDetail(d.x))
                .attr("dy", -ySizeScale(d.y)-format.detail.tooltipFontSize/10 )
                .style("text-anchor", "middle")
                .style("opacity", 1);

            d3.select(`#${event.target.id}`).style("stroke-opacity", 1);
        }

        // Hide visual effects of the detail circle tooltip
        let hideTooltip = (event,d) => {
            tooltip.style("opacity", 0);
            d3.select(`#${event.target.id}`).style("stroke-opacity", 0);
        }

        let showDate = (d: Side, setting:boolean) => {
            hovering[d] = setting;
            updateChart(d);
        }

        let dragstarted = (event, d:Side) => {
            timelineGroup.attr("cursor", "grabbing");
            dragging[d]=true;
            d3.select(`#selection_${d}`)
                .classed("fill-secondary-500",false)
                .classed("hover:stroke-secondary-500",false)
                .classed("fill-primary-500",true)
                .classed("hover:stroke-primary-500",true);
        }

        let dragended = (event, d:Side) => {
            timelineGroup.attr("cursor", "grab");
            dragging[d]=false;
            d3.select(`#selection_${d}`)
                .classed("fill-primary-500",false)
                .classed("hover:stroke-primary-500",false)
                .classed("fill-secondary-500",true)
                .classed("hover:stroke-secondary-500",true);
            updateChart(d);
        }

        // When a drag event happens get the distance dragged and update the timelineSelection values
        // with that delta.  Then check to make sure the timeline selection values go not go out of bounds
        // and update the detail domain.  Then run the chart rerender function.
        let dragged = (event, d: Side) => {
            
            let deltaX = (xTimeline.invert(event.dx)-xTimeline.invert(0))
            
            timelineSelection[d] = new Date(timelineSelection[d].getTime()+deltaX);
            
            if (timelineSelection.min < timelineDomain.x.min) {
                timelineSelection.min = timelineDomain.x.min;
            }
            if (timelineSelection.max > timelineDomain.x.max) {
                timelineSelection.max = timelineDomain.x.max;
            }
            if (d===Side.min) {
                if (timelineSelection.min > offsetDateByDays(timelineSelection.max,-format.timeline.minDateRange)) {
                    timelineSelection.min = offsetDateByDays(timelineSelection.max,-format.timeline.minDateRange);
                }
            } else {
                if (timelineSelection.max < offsetDateByDays(timelineSelection.min,format.timeline.minDateRange)) {
                    timelineSelection.max = offsetDateByDays(timelineSelection.min,format.timeline.minDateRange);
                }
            }
            detailDomain.x[d] = timelineSelection[d];
            
            updateChart(d);
        }

        // Main function to rerender things that change in the chart based on selecting a new timerange
        let updateChart = (d:Side) => {

            // Update the location of the selection bars
            timelineGroup
                .selectAll(`#selection_${d}`)
                .attr("x", xTimeline(detailDomain.x[d]));
            
            // Update the opacity and text of the selection label
            timelineGroup
                .selectAll(`#selectionLabel_${d}`)
                .attr("x", xTimeline(detailDomain.x[d]))
                .attr("opacity", (hovering[d] || dragging[d]) ? 1 : 0)
                .text(d3.timeFormat("%m/%d/%y")(detailDomain.x[d]));
            
            // Update the domain of the detail bar
            xDetail.domain([detailDomain.x.min, detailDomain.x.max])
            
            detailAxis
            .tickFormat(axisFormat())

            chartXaxis.transition().duration(50).call(detailAxis)

            // Update the locations of the circles
            detailGroup
                .selectAll("circle")
                .data(events.data)
                .transition()
                .duration(50)
                .attr("cx", (d:Point<Date, number>) => xDetail(d.x))
            
            // Redraw the middle connecting section
            drawMiddle(false)
        }

        // Draw the middle section that connects the top detail bar to the bottom timeline bar
        let drawMiddle = (append:boolean) => {
            
            middlePoints = [
                [xTimeline(detailDomain.x.min), -format.timeline.height+(format.timeline.height-format.timeline.selectionHeight)/2],
                [xTimeline(detailDomain.x.min), 0],
                [xDetail(detailDomain.x.min),   midHeight-format.detail.height/2],
                [xDetail(detailDomain.x.min),   midHeight+format.detail.height-(format.detail.tickSize+format.detail.tickPadding+format.detail.axisFontSize)],
                [xDetail(detailDomain.x.max),   midHeight+format.detail.height-(format.detail.tickSize+format.detail.tickPadding+format.detail.axisFontSize)],
                [xDetail(detailDomain.x.max),   midHeight-format.detail.height/2],
                [xTimeline(detailDomain.x.max), 0],
                [xTimeline(detailDomain.x.max), -format.timeline.height+(format.timeline.height-format.timeline.selectionHeight)/2]
            ];

            if (append) {
                middleGroup
                .append("path")
                .attr("d", d3.line().curve(d3.curveBumpY)(middlePoints))
                .attr("opacity",".6")
                .classed("fill-graphics-7-50",true);
                
            } else {
                middleGroup
                .select("path") 
                .attr("d", d3.line().curve(d3.curveBumpY)(middlePoints));
            }
        }


        // Compute the domain for the detail bar
        let detailDomain : PointLimit<Date,number> = {
            x:{
                min: offsetDateByDays(events.domain.max, -format.detail.initialDayRange),
                max: events.domain.max,
            },
            y:{
                min: 0,
                max: d3.max(events.data, (d:Point<Date, number>) => d.y)
            },
        };

        // Compute the domain for the timeline bar      
        let timelineDomain : PointLimit<Date,number> = {
            x:{
                min: offsetDateByDays(events.domain.min, -format.timeline.dayPadding),
                max: offsetDateByDays(events.domain.max, format.timeline.dayPadding),
            },
            y:{
                min: 0,
                max: d3.max(events.data, (d:Point<Date, number>) => d.y)
            },
        };
        
        // Maintain the current values of the timeline selection bars
        let timelineSelection : Limit<Date> = {
            min: detailDomain.x.min,
            max: detailDomain.x.max
        }

        // Compute the height of the middle section
        let midHeight = format.box.height
            -format.title.height    
            -format.detail.height
            -format.timeline.height
            -format.timeline.axisFontSize
            -format.timeline.tickPadding
            -format.timeline.tickSize;

        // Init array for the points that define the middle section curve.  These are populated in drawMiddle
        let middlePoints = [];

        // Keep track of the drag & hover status of each handle so we can always show the date while drag is happening
        // regarless of the current location of the mouse
        let dragging : Limit<boolean> = {
            min:false,
            max:false
        }
        let hovering : Limit<boolean> = {
            min:false,
            max:false
        }
        
        // Define the axis
        // Both y axis have the same domain but have different ranges as one is used to size the detail circles
        // and the other is used to set the circle opacity.  Since the circles are colored and sized by y value 
        // the low end of size and opacity are limited to help ensure visability.
        let xDetail = d3.scaleTime()
            .domain([detailDomain.x.min, detailDomain.x.max])
            .range([format.detail.padding, format.box.width-2*format.detail.padding]);
        let xTimeline = d3.scaleTime()
            .domain([timelineDomain.x.min, timelineDomain.x.max])
            .range([format.timeline.horizInset, format.box.width-2*format.timeline.horizInset]);
        let ySizeScale = d3.scaleLinear()
            .domain([timelineDomain.y.min, timelineDomain.y.max])
            .range([format.detail.circleMinDiameter/2, format.detail.height/2]);
        let yUnitScale = d3.scaleLinear()
            .domain([timelineDomain.y.min, timelineDomain.y.max])
            .range([0.2, 1]);
        // Define histogram to create bins for the timeline bar (bottom bar) and another scale to color the histogram
        let bins = d3.histogram()
            .domain(xTimeline.domain())
            .value(function(d) { return d.x; })
            .thresholds(xTimeline.ticks(format.timeline.binCount))
            (events.data)
        let yBin = d3.scaleLinear()
            .domain([0, d3.max(bins, function(d) { 
                return d.reduce((acc, value) => acc+value.y, 0)
            })])
            .range([0, 1]);

        // Append the svg object to the div
        const svg = d3.select(el)
        .attr("preserveAspectRatio", "xMidYMid meet")
        .attr("viewBox", "0 0 "+format.box.width+" "+format.box.height)
        .classed("svg-content", true);
        
        // Create groups for different regions of the visualization
        let titleGroup = svg.append("g")
        .attr("transform", `translate(${0},${0})`);
        let middleGroup = svg.append("g")
        .attr("transform", `translate(0,${format.title.height+format.timeline.axisFontSize+format.timeline.tickPadding+format.timeline.tickSize+format.timeline.height})`);
        let timelineGroup = svg.append("g")
        .attr("transform", `translate(0,${format.title.height+format.timeline.axisFontSize+format.timeline.tickPadding+format.timeline.tickSize})`);
        let detailGroup = svg.append("g")
        .attr("transform", `translate(0,${format.box.height-(
            format.detail.height+format.detail.tickSize+format.detail.tickPadding+format.detail.axisFontSize+format.detail.tickStrokeWidth
        )})`);
        
        // Create the clip path that will be used to clip the detail group (which has the data circles in it)
		detailGroup
			.append("clipPath")
			.attr("id", () => `clip-rect-${chartId}`)
			.append("rect")
			.attr("x", xDetail.range()[0])
			.attr("y", 0)
			.attr("height", format.detail.height)
			.attr("width", xDetail.range()[1]-xDetail.range()[0])

        // Add circles to the detail group (top)
        detailGroup
        .selectAll("circle")
        .data(events.data)
        .enter()
        .append("a")
        .attr("xlink:href", (d:Point<Date, number>) => "http://172.16.4.134/data-view?selected=1&query=anomalyCreated+%3E%3D+"+
            `${timestampToDateInput(offsetDateByDays(d.x,-linkDomainSpan/2).getTime()/1000)}`+
            "%3B+anomalyCreated+%3C%3D+"+
            `${timestampToDateInput(offsetDateByDays(d.x,linkDomainSpan/2).getTime()/1000)}`+
            "%3B+customerNumber+%3D+"+
            `${customerNumber}`+
            "%3B")
        .append("circle")
        .attr("id", (d:Point<Date, number>, i:number)=>`circle-${i}`)
        .attr("cx", (d:Point<Date, number>) => xDetail(d.x))
        .attr("cy", format.detail.height/2)
        .attr("r", (d:Point<Date, number>) => ySizeScale(d.y) )
        .classed("fill-graphics-1-600",true)
        .attr("fill-opacity", (d:Point<Date, number>) => yUnitScale(d.y))
        .classed("stroke-dark",true)
        .attr("stroke-width", format.detail.strokeWidth)
        .attr("stroke-opacity", 0)
        .on("pointerover", showTooltip)
        .on("pointerout", hideTooltip)
        .attr("clip-path", () => `url(#clip-rect-${chartId})`);

        // Define a tooltop text element to show circle count on hover
        let tooltip = detailGroup
        .append("text")
        .attr("x", 0)
        .attr("y", format.detail.height/2)
        .attr("text-anchor", "left")
        .style("font-size", `${format.detail.tooltipFontSize}px`)
        .style("opacity", 0)
        .text("");

        // Define how the axis tick labels will be formatted based on the domain of the detail section
        // If the domain is less than 5 days then display the date as month/day hour otherwise show month/day/year
        let axisFormat = () => {
            if (detailDomain.x.max.getTime()-detailDomain.x.min.getTime() < 86400000*5) {
                return d3.timeFormat("%m/%d %I%p")
            } else {
                return d3.timeFormat("%m/%d/%y")
            }
        }

        // Add axis to the detail bar
        let detailAxis = d3.axisBottom(xDetail)
        .tickSize(format.detail.tickSize)
        .tickPadding(format.detail.tickPadding)
        .ticks(format.detail.tickCount)
        .tickFormat(axisFormat())

        let chartXaxis = detailGroup.append("g")
        .attr("transform", "translate(0," + format.detail.height + ")")
        .attr("stroke-width", format.detail.tickStrokeWidth)
        .style("font-size", `${format.detail.axisFontSize}px`)
        .attr("class","text-dark")
        .call(detailAxis);

        // Add rectangle to surround the timeline bar
        timelineGroup
        .append("rect")
        .attr("x", xTimeline(timelineDomain.x.min))
        .attr("y", (format.timeline.height-format.timeline.selectionHeight)/2 )
        .attr("height", format.timeline.selectionHeight)
        .attr("width", xTimeline(timelineDomain.x.max)-xTimeline(timelineDomain.x.min))
        .attr("stroke-width", format.timeline.outlineStrokeWidth)
        .classed("stroke-dark",true)
        .attr("fill-opacity", 0)
        
        // Add tick for end of timeline axis
        timelineGroup
            .append("path")
            .attr("d", d3.line().curve(d3.curveLinear)([[xTimeline(timelineDomain.x.min),-format.timeline.tickSize],[xTimeline(timelineDomain.x.min),(format.timeline.height-format.timeline.selectionHeight)/2]]))
            .attr("stroke-width", format.timeline.outlineStrokeWidth*2)
            .classed("stroke-dark",true);

        // Add timeline axis tick label
        timelineGroup
        .append("text")
        .attr("id","timelineLabelExtent_min")
        .attr("x", xTimeline(timelineDomain.x.min))
        .attr("y", -format.timeline.tickSize-format.timeline.tickPadding)
        .attr("text-anchor", "start")
        .attr("class","fill-dark")
        .style("font-size", `${format.timeline.axisFontSize}px`)
        .text(d3.timeFormat("%m/%d/%y")(timelineDomain.x.min));

        // Add tick for end of timeline axis
        timelineGroup
            .append("path")
            .attr("d", d3.line().curve(d3.curveLinear)([[xTimeline(timelineDomain.x.max),-format.timeline.tickSize],[xTimeline(timelineDomain.x.max),(format.timeline.height-format.timeline.selectionHeight)/2]]))
            .attr("stroke-width", format.timeline.outlineStrokeWidth*2)
            .classed("stroke-dark",true);

        // Add timeline axis tick label
        timelineGroup
        .append("text")
        .attr("id","timelineLabelExtent_max")
        .attr("x", xTimeline(timelineDomain.x.max))
        .attr("y", -format.timeline.tickSize-format.timeline.tickPadding)
        .attr("text-anchor", "end")
        .attr("class","fill-dark")
        .style("font-size", `${format.timeline.axisFontSize}px`)
        .text(d3.timeFormat("%m/%d/%y")(timelineDomain.x.max));
   
        // Add the bins to color the timeline bar
        timelineGroup
        .selectAll("bins")
        .data(bins)
        .enter()
        .append("rect")
        .attr("x", (d) => xTimeline(d.x0))
        .attr("width", (d) => (xTimeline(d.x1)-xTimeline(d.x0)))
        .attr("y", (format.timeline.height-format.timeline.selectionHeight)/2)
        .attr("height", format.timeline.selectionHeight)
        .classed("fill-graphics-1-600",true)
        .attr("opacity", (d) => yBin(d.reduce((acc, value) => acc+value.y, 0)))

        // Add the lower (min) selection bar
        timelineGroup
        .append("rect")
        .attr("id","selection_min")
        .attr("x", xTimeline(detailDomain.x.min))
        .attr("y", 0)
        .attr("transform", `translate(${-format.timeline.selectorWidth/2},0)`)
        .attr("height", format.timeline.height)
        .attr("width", format.timeline.selectorWidth)
        .attr("rx", format.timeline.selectorRadius)
        .attr("ry", format.timeline.selectorRadius)
        .attr("stroke-width", format.detail.circleHoverStroke)
        .classed("fill-secondary-500",true)
        .classed("hover:stroke-secondary-500",true)
        .call(d3.drag()
			.on("start", (e) => dragstarted(e,Side.min))
			.on("drag", (e) => dragged(e,Side.min))
			.on("end", (e) => dragended(e,Side.min))
        )
        .on("mouseover", () => showDate(Side.min, true))
        .on("mouseout", () => showDate(Side.min, false))

        timelineGroup
        .append("text")
        .attr("id","selectionLabel_min")
        .attr("x", xTimeline(detailDomain.x.min))
        .attr("y", format.timeline.height+format.timeline.axisFontSize)
        // .attr("dx", format.timeline.selectorWidth/2)
        .attr("text-anchor", "end")
        .attr("class","fill-dark")
        .attr("opacity", 0)
        .style("font-size", `${format.timeline.axisFontSize}px`)
        .text(d3.timeFormat("%m/%d/%y")(detailDomain.x.min));

        // Add the upper (max) selection bar
        timelineGroup
        .append("rect")
        .attr("id","selection_max")
        .attr("x", xTimeline(detailDomain.x.max))
        .attr("y", 0)
        .attr("transform", `translate(${-format.timeline.selectorWidth/2},0)`)
        .attr("height", format.timeline.height)
        .attr("width", format.timeline.selectorWidth)
        .attr("rx", format.timeline.selectorRadius)
        .attr("ry", format.timeline.selectorRadius)
        .attr("stroke-width", format.detail.circleHoverStroke)
        .classed("fill-secondary-500",true)
        .classed("hover:stroke-secondary-500",true)
        .call(d3.drag()
            .on("start", (e) => dragstarted(e,Side.max))
			.on("drag", (e) => dragged(e,Side.max))
			.on("end", (e) => dragended(e,Side.max))
        )
        .on("mouseover", () => showDate(Side.max, true))
        .on("mouseout", () => showDate(Side.max, false))

        timelineGroup
        .append("text")
        .attr("id","selectionLabel_max")
        .attr("x", xTimeline(detailDomain.x.max))
        .attr("y", format.timeline.height+format.timeline.axisFontSize)
        // .attr("dx", -format.timeline.selectorWidth/2)
        .attr("text-anchor", "start")
        .attr("class","fill-dark")
        .attr("opacity", 0)
        .style("font-size", `${format.timeline.axisFontSize}px`)
        .text(d3.timeFormat("%m/%d/%y")(detailDomain.x.max));
        
        // Draw the middle section
        drawMiddle(true);

        // Add the title
        titleGroup
        .append("text")
        .attr("x", 0)
        .attr("y", 0)
        .attr("dy", format.title.fontSize)
        .attr("text-anchor", "left")
        .attr("class","fill-dark")
        .style("font-size", `${format.title.fontSize}px`)
        .text(title);
        
    });
</script>

<div bind:this={wr} bind:clientWidth={imageWidth} bind:clientHeight={imageHeight} class="overflow-visible relative">
    <svg bind:this={el}></svg>
</div>