import type {NumberFormat} from "./chart";

export type Dispersion = {
	"q0": number,
	"q1": number,
	"q2": number,
	"q3": number,
	"q4": number,
}

export type Bin = {
	min: number,
	max: number,
	center: number,
	count: number,
}

export type Histogram = {
	bins: Array<Bin>
}

export type Limit<V> = {
	min: V,
	max: V
};

export type Point<T,U> = {x:T, y:U};
export type PointArray<T,U> = Array<Point<T,U>>;
export type PointLimit<T,U> = {x:Limit<T>, y:Limit<U>};

export type AxisFormat = {x:NumberFormat, y:NumberFormat};