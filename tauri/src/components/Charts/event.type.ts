import type {PointArray, Limit} from "./chart.type";

export type ChartFormat = {
    box: { // base size of the image.  The image can be scaled to any size but this gives a framework that all other size values are specified relative to.
        width: number;
        height: number;
    };
    detail: {
        height: number;
        circleMinDiameter: number;
        circleHoverStroke: number;
        padding: number;
        strokeWidth: number;
        initialDayRange: number;
        tooltipFontSize: number;
        tickSize: number;
        tickPadding: number;
        tickCount: number;
        tickStrokeWidth: number; 
        axisFontSize: number;
    };
    timeline: {
        height: number;
        outlineStrokeWidth: number;
        selectionHeight: number;
        binCount: number;
        selectorWidth: number;
        selectorRadius: number;
        horizInset: number;
        dayPadding: number;
        minDateRange: number;
        axisFontSize: number;
        tickSize: number;
        tickPadding: number;
    };
    title: {
        height: number;
        fontSize: number;
    };
};

// The domainType can be a number (epoch time) or Date object
export type domainType = Date | number;

export type EventData<domainType> = {
    data: PointArray<domainType, number>,
    domain: Limit<domainType>
}
