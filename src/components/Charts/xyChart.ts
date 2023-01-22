

import type { ChartFormat } from "./xyChart.type";
import type { Point, PointArray, PointLimit } from "./chart.type";

import {NumberFormat, Side, isDate, updateTickFormat, maxStringLength, getDataExtents} from "./chart"

import * as d3 from "d3";

type xType = Date | number;
type yType = number;

export class XyChart {
    
    // Label for the x axis
    public xLabel: string;
    // Label for the y axis
    public yLabel: string;
    // Title for the chart
    public title: string="";
    // Data that will be displayed on this chart.  This base chart does not plot the data but this is needed to properly scale the axis
    public data: PointArray<xType, yType> = [];

    // // Override default chart domain. This is in the same units as the data itself.
    // public domain : PointLimit;

    // The element that holds the chart
    public el;
    // The chart format parameters
    public format: ChartFormat = {
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
    
    #svg;
    #titleGroup;
    #chartGroup;
    #axisGroup;
    #legendGroup;
    #x;
    #y;
    #xAxis;
    #yAxis;
    #chartXaxis;
    #chartYaxis
    #width;
    #height;
    #margin;
    #numberFormat;
    #domain;
    #legend;
    
    constructor(xLabel: string, yLabel: string, title: string, format: ChartFormat, data: PointArray<xType, yType>, domain: PointLimit<xType, yType>, element) {        
        this.xLabel = xLabel;
        this.yLabel = yLabel;
        this.title = title;
        this.format = format;
        this.data = data;
        this.#domain = domain;
        this.el = element;
        
        this.applyDomain(domain);

        this.init();
    }
    
    get svg() {return this.#svg;}
    get titleGroup() {return this.#titleGroup;}
    get chartGroup() {return this.#chartGroup;}
    get axisGroup() {return this.#axisGroup;}
    get legendGroup() {return this.#legendGroup;}
    get width() {return this.#width;}
    get height() {return this.#height;}
    get margin() {return this.#margin;}
    get domain() {return this.#domain;}
    get x() {return this.#x};
    get y() {return this.#y};
    get xAxis() {return this.#xAxis};
    get yAxis() {return this.#yAxis};
    get chartXaxis() {return this.#chartXaxis};
    get chartYaxis() {return this.#chartYaxis};
    get legend() {return this.#legend};
    set domain(domain : PointLimit<xType, yType>) {
        this.applyDomain(domain)
    }

    applyDomain(domain:PointLimit<xType, yType>) {
        // if domain is input as null then set it to the data limits
        let dataDomain = getDataExtents(this.data, this.format.axis.format, {x:{min:0,max:0},y:{min:0,max:0}});
        this.#domain.x.min = (domain.x.min !== null) ? domain.x.min : dataDomain.x.min;
        this.#domain.x.max = (domain.x.max !== null) ? domain.x.max : dataDomain.x.max;
        this.#domain.y.min = (domain.y.min !== null) ? domain.y.min : dataDomain.y.min;
        this.#domain.y.max = (domain.y.max !== null) ? domain.y.max : dataDomain.y.max;
    }
    

    public init() {
        // number format to actually use.  Gets updated when running 'updateTickFormat'
        this.#numberFormat = {
            "x": this.format.axis.format.x,
            "y": this.format.axis.format.y
        };

        // Calculated chart dimensions
        const stringLength = maxStringLength(this.data, this.#numberFormat);

        const hasTitle = this.title.length > 0;
        this.#margin = {
            top: hasTitle ? 
                (this.format.title.fontSize + this.format.title.translate.y + this.format.title.paddingBottom)
            : this.format.title.fontSize/2,
            right: (stringLength.x/2)*this.format.axis.tick.fontSize*this.format.fontAR,
            bottom: this.format.axis.label.fontSize*2 + this.format.axis.tick.fontSize + this.format.axis.tick.padding + this.format.axis.tick.size,
            left: this.format.axis.label.fontSize*2 + stringLength.y*this.format.axis.tick.fontSize*this.format.fontAR + this.format.axis.tick.padding + this.format.axis.tick.size
        }

        // Compute chart data region size
        this.#width = this.format.box.width - this.#margin.left - this.#margin.right;
        this.#height = this.format.box.height - this.#margin.top - this.#margin.bottom;

        const chartTranslate:Point<number, number> = {x:this.#margin.left, y:this.#margin.top};

        // append the svg object to the div called "chart"
        this.#svg = d3.select(this.el)
            .attr("preserveAspectRatio", "xMidYMid meet")
            .attr("viewBox", "0 0 "+this.format.box.width+" "+this.format.box.height)
            .classed("svg-content", true);
        this.#titleGroup = this.svg.append("g")
            .attr("transform", `translate(${this.format.title.translate.x},${this.format.title.translate.y})`);
        this.#chartGroup = this.svg.append("g")
            .attr("transform", `translate(${chartTranslate.x},${chartTranslate.y})`);
        this.#axisGroup = this.svg.append("g")
            .attr("transform", `translate(${chartTranslate.x},${chartTranslate.y})`);
        this.#legendGroup = this.#chartGroup.append("g")
            .attr("transform", `translate(${this.format.legend.translate.x},${this.format.legend.translate.x})`);

    }

    public addLegend() {
        this.#legend = this.#legendGroup.append("g");
    }

    public addAxis() {

        // Define axis scales
        this.#x = isDate(this.#numberFormat.x)  ? 
        d3.scaleTime()
            .domain([this.#domain.x.min,this.#domain.x.max])
            .range([0, this.#width])
        : d3.scaleLinear()
            .domain([this.#domain.x.min,this.#domain.x.max])
            .range([0, this.#width]);

        this.#y = isDate(this.#numberFormat.y)  ? 
            d3.scaleTime()
            .domain([this.#domain.y.min,this.#domain.y.max])
            .range([this.#height, 0])
        : d3.scaleLinear()
            .domain([this.#domain.y.min,this.#domain.y.max])
            .range([this.#height, 0]);

        // Define axis
        this.#xAxis = d3.axisBottom(this.#x)
        .tickSize(this.format.axis.tick.size)
        .tickPadding(this.format.axis.tick.padding);

        this.#yAxis = d3.axisLeft(this.#y)
        .tickSize(this.format.axis.tick.size)
        .tickPadding(this.format.axis.tick.padding);

        updateTickFormat(this.#xAxis, this.#numberFormat, Side.x);
        updateTickFormat(this.#yAxis, this.#numberFormat, Side.y);

        // Add x-axis to chart
        this.#chartXaxis = this.#axisGroup.append("g")
        .attr("transform", "translate(0," + this.#height + ")")
        .attr("stroke-width", this.format.axis.strokeWidth)
        .style("font-size", `${this.format.axis.tick.fontSize}px`)
        .attr("class","text-dark dark:text-light")
        .call(this.#xAxis);

        // Add x-axis label
        this.#chartXaxis
        .append("text")
        .attr("x", this.#width/2)
        .attr("y", this.format.axis.tick.size+this.format.axis.tick.padding+this.format.axis.tick.fontSize)
        .attr("dy", this.format.axis.label.fontSize)
        .attr("text-anchor", "middle")
        .attr("class","fill-dark dark:fill-light")
        .style("font-size", `${this.format.axis.label.fontSize}px`)
        .text(this.xLabel);

        // Add y-axis to chart
        this.#chartYaxis = this.#axisGroup.append("g")
        .attr("stroke-width", this.format.axis.strokeWidth)
        .style("font-size", `${this.format.axis.tick.fontSize}px`)
        .attr("class","text-dark dark:text-light")
        .call(this.#yAxis);

        // Add y-axis label
        this.#chartYaxis
        .append("text")
        .attr("x", -this.#height/2)
        .attr("y", -this.#margin.left)
        .attr("dy", this.format.axis.label.fontSize)
        .attr("text-anchor", "middle")
        .attr("transform", "rotate(-90)")
        .attr("class","fill-dark dark:fill-light")
        .style("font-size", `${this.format.axis.label.fontSize}px`)
        .text(this.yLabel);

        // Add title
        this.#titleGroup
        .append("text")
        .attr("x", 0)
        .attr("y", 0)
        .attr("dy", this.format.title.fontSize)
        .attr("text-anchor", "left")
        .attr("class","fill-dark dark:fill-light")
        .style("font-size", `${this.format.title.fontSize}px`)
        .text(this.title);
    }
    
}