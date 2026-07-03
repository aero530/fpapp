//! Functions to plot data

use log::error;
use plotters::coord::Shift;
use plotters::prelude::*;
use image::{ImageBuffer, Rgba, RgbImage, DynamicImage};

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
/// return.1 is the maximum value.  NaN when every table is empty.
pub fn range(input: Vec<&Table<u32>>) -> (f64, f64) {
    let y_min = input
        .iter()
        .map(|table| table.range().0)
        .fold(f64::NAN, |m, v| v.min(m));
    let y_max = input
        .iter()
        .map(|table| table.range().1)
        .fold(f64::NAN, |m, v| v.max(m));
    (y_min, y_max)
}

/// Return the overall span of year values for a group of tables,
/// or None when every table is empty.
fn domain(input: Vec<&Table<u32>>) -> Option<(u32, u32)> {
    let domains: Vec<(u32, u32)> = input.iter().filter_map(|table| table.domain()).collect();
    let x_min = domains.iter().map(|d| d.0).min()?;
    let x_max = domains.iter().map(|d| d.1).max()?;
    Some((x_min, x_max))
}

/// Draw the chart onto the provided drawing area (shared between the file and
/// buffer backends)
fn draw_chart<DB: DrawingBackend>(
    root: &DrawingArea<DB, Shift>,
    data: &[(String, &Table<u32>)],
    title: &str,
) where
    DB::ErrorType: 'static,
{
    let tables: Vec<&Table<u32>> = data.iter().map(|(_table_name, table)| *table).collect();
    let Some(domain) = domain(tables.clone()) else {
        error!("No data to plot for '{}'", title);
        return;
    };
    let range = range(tables);
    if range.0.is_nan() || range.1.is_nan() {
        error!("No data to plot for '{}'", title);
        return;
    }

    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(root)
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
        .bold_line_style(BLACK.mix(0.8))
        .light_line_style(BLACK.mix(0.1))
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
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .legend_area_size(40)
        .label_font(("sans-serif", 20).into_font())
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}

/// Generate a scatter plot and save it to a file
pub fn scatter_plot_file(
    filepath: String,
    data: Vec<(String, &Table<u32>)>,
    title: String,
    width: u32,
    height: u32,
) {
    let root = BitMapBackend::new(&filepath, (width, height)).into_drawing_area();
    draw_chart(&root, &data, &title);
}

/// Generate a scatter plot and return it as an image buffer
pub fn scatter_plot_buf(
    data: Vec<(String, &Table<u32>)>,
    title: String,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Define a buffer to store in plot image's pixel values
    let mut buf: Vec<u8> = vec![0; width as usize * height as usize * 3];

    {
        let root = BitMapBackend::with_buffer(&mut buf, (width, height)).into_drawing_area();
        draw_chart(&root, &data, &title);
    }

    let img: RgbImage = ImageBuffer::from_raw(width, height, buf).unwrap();
    DynamicImage::ImageRgb8(img).into_rgba8()
}
