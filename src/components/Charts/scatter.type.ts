import type {Point, PointArray} from "./chart.type";

// Base data for a scatter plot
export type DataInput = {
    points: PointArray<number, number>, // data points to add to the chart
    groups: Array<number> | null, // array of id's for each point in the points array that defines which group each point is a member of
}

// Bounds data for a scatter plot.  This is rendered aa filled in circle behind each data point.
// The circle is scaled in the x&y dimensions so often will look elliptical
export type BoundInput = {
    points: PointArray<number, number>, // data points of the centers of the array of ellipses that make up the bounds
    groups: Array<number> | null, // array of id's for each point in the points array that defines which group each point is a member of
    radius: Point<number, number>, // radius of the circle / ellipse to draw around each point
}
