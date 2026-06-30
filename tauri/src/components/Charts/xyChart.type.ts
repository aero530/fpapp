import type { Point, AxisFormat } from "./chart.type";

export type ChartFormat = {
    box: { // base size of the image.  The image can be scaled to any size but this gives a framework that all other size values are specified relative to.
        width: number;
        height: number;
    };
    title: {
        fontSize: number;
        paddingBottom: number;
        translate:Point<number, number>; // x/y translation of the title from top left corner
    };
    axis:{
        tick:{
            fontSize:number;
            padding: number;
            size: number; // length of tick lines
        };
        label: {
            fontSize:number;
        };
        strokeWidth: number;
        format: AxisFormat;
    };
    tooltip: {
        circleRadius : number;
        circleStrokeWidth : number;
        borderWidth : number;
        borderRadius : number;
        padding : number;
        format : AxisFormat
    };
    legend: {
        fontSize : number;
        translate : Point<number, number>; // x/y translation of the legend from top left corner of the plotting area
    };
    lineStrokeWidth: number;
    dataPointSize: number;
    fontAR: number; // aspect ratio of font (width of font is about 1/2 the height)
};