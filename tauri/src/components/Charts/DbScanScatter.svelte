<script lang="ts">
    
    import type { Point, PointArray, PointLimit } from "./chart.type";
    import type { ChartFormat } from "./xyChart.type";
    import type { DbScan, Normalization } from "../../userApi.type";
    import type { DataInput } from "./scatter.type";

    import * as d3 from "d3";

    import { getDataExtents, NumberFormat } from "./chart";
    import Scatter from "./Scatter.svelte";

    export let fingerprint: DbScan;
    export let dataInput: DataInput | null;
    export let title: string="";
    export let showBounds: boolean;
    export let showGroups: boolean;

    // Flag to enable showing points in the chart if there is data in addition to the bounds
    let showPoints = (dataInput === null) ? false : true;

    let format: ChartFormat = {
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
        lineStrokeWidth : 0.6,
        dataPointSize : .3,
        fontAR : 0.5,
    };


    function denormalize(point:Array<number>, sensors:Array<string>, norm:Normalization) {      
        return point.map((value,index) => {
            let number = value;
            let inMin = 0; // dbScan data is always normalized to 0 to 1
            let inMax = 1;
            let outMin = norm[sensors[index]].min;
            let outMax = norm[sensors[index]].max;
            return (number - inMin) * (outMax - outMin) / (inMax - inMin) + outMin
        })
    }

    // Pull input values from data
    const boundsPoints = fingerprint.dbScan.data.knn_algorithm.cover_tree.data.map(
        (point) => {
            let output = denormalize(
                point, 
                fingerprint.dbScan.params.sensors, 
                fingerprint.dbScan.normalization
            );
            return {"x":output[0],"y":output[1]}
        }
    )

    const boundsGroupLabels: number[] = fingerprint.dbScan.data.cluster_labels;
    
    const xLabel = fingerprint.dbScan.params.sensors[0];
    const yLabel = fingerprint.dbScan.params.sensors[1];
    
    // Compute domain extents of the fingerprint bounds points
    let [xMin, xMax] = d3.extent(boundsPoints, (d: Point<number, number>) => d.x);
    let [yMin, yMax] = d3.extent(boundsPoints, (d: Point<number, number>) => d.y);

    // Scale espilon up by the extent of the data in each axis
    const epsilon = {
        x: fingerprint.dbScan.params.epsilon * (xMax-xMin),
        y: fingerprint.dbScan.params.epsilon * (yMax-yMin)
    };

    // Combine data sets to facilitate domain computation
    let fullDataset: PointArray<number, number> = [];
    
    if (showPoints && dataInput.points) {
        fullDataset = [...fullDataset, ...dataInput.points]
    };
    if (boundsPoints) {
        fullDataset = [...fullDataset, ...boundsPoints]
    };

    // Compute padding to add onto the bounds data to account for epsilon
    let padding: PointLimit<number, number> = {
        x: {min:epsilon.x,max:epsilon.x},
        y: {min:epsilon.y,max:epsilon.y},
    }

    // Calculate the domain on the chart based on the full data set and epsilon padding
    let domain = getDataExtents(fullDataset, format.axis.format, padding);

</script>

<Scatter 
    data={
        showPoints ? 
            {
                "points": dataInput.points,
                "groups": showGroups ? dataInput.groups : null,
            }
        :
            null
    }
    {xLabel}
    {yLabel}
    bounds={
        showBounds ?
            {
                "points": boundsPoints,
                "groups": showGroups ? boundsGroupLabels : null,
                "radius": epsilon,
            } 
        :
            null
    }
    {format}
    {title}
    {domain}
/>