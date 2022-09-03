use plotters::prelude::*;
use rust_plotter::*;

// Source: https://towardsdatascience.com/how-to-create-plot-in-rust-fdc6c024461c

fn main() {
    let file_path = "./kc_house_data.csv".to_string();

    // Read the CSV file containing the data we want to plot and assign them to two local vectors:
    let (price, sqft_living) = match read_csv(&file_path) {
        Ok(t) => t,
        _ => (Vec::new(), Vec::new()),
    };

    // Perform a zip (merge) function so that we stitch the two vectors into a single vector of tuples, and make the
    // resulting iterator into a collection:
    let price_sqft_living: Vec<(f64, f64)> = price
        .iter()
        .cloned()
        .zip(sqft_living.iter().cloned())
        .collect();

    // Draw some shiny plots!
    draw_scatter_plot(&price_sqft_living, "scatter_plot.png");
    draw_area_plot(&price_sqft_living, "area_plot.png");

    // These do not currently work:
    draw_bar_chart(&price_sqft_living, "bar_chart.png");
    // draw_line_chart(&price_sqft_living, "line_chart.png");
}
