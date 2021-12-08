use plotters::prelude::*;

use super::tables::Table;

pub const COLORS: [RGBColor; 9] = [
    RGBColor(24, 171, 221),
    RGBColor(176, 75, 207),
    RGBColor(29, 229, 188),
    RGBColor(234, 115, 105),
    RGBColor(220, 75, 179),
    RGBColor(223, 84, 44),
    RGBColor(234, 189, 60),
    RGBColor(110, 240, 210),
    RGBColor(239, 166, 143),
];

pub fn range(input: Vec<&Table<u32>>) -> (u32, u32, f64, f64) {
    let x_min = *input
        .iter()
        .map(|table| {
            *table
                .0
                .keys()
                .copied()
                .collect::<Vec<u32>>()
                .iter()
                .min()
                .unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .min()
        .unwrap();
    let x_max = *input
        .iter()
        .map(|table| {
            *table
                .0
                .keys()
                .copied()
                .collect::<Vec<u32>>()
                .iter()
                .max()
                .unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap();
    let y_min = input
        .iter()
        .map(|table| {
            table
                .0
                .values()
                .copied()
                .collect::<Vec<f64>>()
                .iter()
                .fold(0.0 / 0.0, |m, v| v.min(m))
        })
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0 / 0.0, |m, v| v.min(m));
    let y_max = input
        .iter()
        .map(|table| {
            table
                .0
                .values()
                .copied()
                .collect::<Vec<f64>>()
                .iter()
                .fold(0.0 / 0.0, |m, v| v.max(m))
        })
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0 / 0.0, |m, v| v.max(m));
    (x_min, x_max, y_min, y_max)
}

pub fn scatter_plot(filepath: String, data: Vec<(String, &Table<u32>)>, title: String) {
    let (x_min, x_max, y_min, y_max) =
        range(data.iter().map(|(_table_name, table)| *table).collect());

    let root = BitMapBackend::new(&filepath, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 60).into_font())
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(100)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_label_style(("sans-serif", 25).into_font())
        .y_label_style(("sans-serif", 25).into_font())
        .bold_line_style(&BLACK.mix(0.8))
        .light_line_style(&BLACK.mix(0.1))
        .y_label_formatter(&|v| format!("${}", v))
        .draw()
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_y_axis()
        .x_label_style(("sans-serif", 40).into_font())
        .x_desc("Year")
        .draw()
        .unwrap();

    data.iter()
        .enumerate()
        .for_each(|(idx, (table_name, table))| {
            chart
                .draw_series(LineSeries::new(
                    table.0.clone().into_iter(),
                    COLORS[idx % COLORS.len()].stroke_width(4),
                ))
                .unwrap()
                .label(table_name)
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 30, y)],
                        COLORS[idx % COLORS.len()].stroke_width(4),
                    )
                });
        });
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .legend_area_size(40)
        .label_font(("sans-serif", 20).into_font())
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}
