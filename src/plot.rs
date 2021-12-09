//! Functions to plot data

use plotters::prelude::*;

use crate::simulation::Table;

/// Colors used to generate plots
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

/// Return the overall span of dollar values for a group of tables.
///
/// The returned value is a tuple where return.0 is the minimum and
/// return.1 is the maximum value.
pub fn range(input: Vec<&Table<u32>>) -> (f64, f64) {
    let y_min = input
        .iter()
        .map(|table| table.range().0)
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0 / 0.0, |m, v| v.min(m));
    let y_max = input
        .iter()
        .map(|table| table.range().1)
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0 / 0.0, |m, v| v.max(m));
    (y_min, y_max)
}

/// Return the overall span of year values for a group of tables.
///
/// The returned value is a tuple where return.0 is the minimum and
/// return.1 is the maximum value.
fn domain(input: Vec<&Table<u32>>) -> (u32, u32) {
    let x_min = *input
        .iter()
        .map(|table| table.domain().0)
        .collect::<Vec<u32>>()
        .iter()
        .min()
        .unwrap();
    let x_max = *input
        .iter()
        .map(|table| table.domain().1)
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap();
    (x_min, x_max)
}

/// Generate a scatter plot
pub fn scatter_plot(filepath: String, data: Vec<(String, &Table<u32>)>, title: String) {
    let domain = domain(data.iter().map(|(_table_name, table)| *table).collect());
    let range = range(data.iter().map(|(_table_name, table)| *table).collect());

    let root = BitMapBackend::new(&filepath, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 60).into_font())
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(100)
        .build_cartesian_2d(domain.0..domain.1, range.0..range.1)
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
