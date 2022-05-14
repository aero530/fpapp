import * as d3 from "d3";
import type {AxisFormat, Point, PointArray, PointLimit} from "./chart.type";

export enum NumberFormat {
	Decimal = "~r",
	Scientific = "~e",
	DateQuarter = "Q%q/%y",
	SI = "~s"
};

export enum Side {
    x="x",
    y="y"
}

export function isDate(input: NumberFormat) {
    return (input === NumberFormat.DateQuarter)
}

export function updateTickFormat(axis, axisFormat: AxisFormat, side:Side) {
    let tickCount = 6;
    
    if (isDate(axisFormat[side])) {
        axis.tickFormat(d3.timeFormat(axisFormat[side]))
    } else {
        let ticks = axis.scale().ticks();
        if (((Math.abs(ticks[0]) > 0 && Math.abs(ticks[0]) < .01) || Math.abs(ticks[ticks.length-1])>10000)) {
            axisFormat[side] = NumberFormat.SI;
            tickCount = 4;
        }
        axis.tickFormat(d3.format(axisFormat[side])) // apply the new tick format
    }
    axis.ticks(tickCount);
}

export function toFormattedString(value: number | Date, format: NumberFormat) {
    return isDate(format) 
        ? d3.timeFormat(format)(value)
        : d3.format(format)(value)
}

export function maxStringLength<T,U>(data: PointArray<T,U>, format: AxisFormat) {
    let xMax = d3.max(data, (p:Point<T,U>) => 
        isDate(format.x) ? 
            d3.timeFormat(format.x)(p.x).length
            : d3.format(format.x)(p.x).length);

    let yMax = d3.max(data, (p:Point<T,U>) => 
        isDate(format.y) ? 
            d3.timeFormat(format.y)(p.y).length
            : d3.format(format.y)(p.y).length);

    return {"x": xMax, "y": yMax}
}

// Data and padding must be in 'data' units (not px)
export function getDataExtents(data: PointArray<Date|number, number>, format: AxisFormat, padding: PointLimit<number, number>) {
    let [xMin, xMax] = d3.extent(data, (d:Point<Date|number, number>) => d.x);
    let [yMin, yMax] = d3.extent(data, (d:Point<Date|number, number>) => d.y);

    if (isDate(format.x)) {
        xMin = new Date(xMin - padding.x.min);
        xMax = new Date(xMax + padding.x.max);
    } else {
        xMin -= padding.x.min;
        xMax += padding.x.max;
    }
    
    if (isDate(format.y)) {
        yMin = new Date(yMin - padding.y.min);
        yMax = new Date(yMax + padding.y.max);
    } else {
        yMin -= padding.y.min;
        yMax += padding.y.max;
    }

    let output : PointLimit<number, number> = {x:{min: xMin, max:xMax}, y:{min: yMin, max:yMax}};
    
    return output
}