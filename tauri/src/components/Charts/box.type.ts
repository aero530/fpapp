import type {Point} from "./chart.type";

export type ChartFormat = {
    box: { // base size of the image.  The image can be scaled to any size but this gives a framework that all other size values are specified relative to.
        width: number;
        height: number;
    };
    lineStrokeWidth: number;
    limitOffset: Point<number, number>;
};