use fast_float::parse;
use plotters::prelude::*;

pub fn read_csv(path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    // Build the CSV reader and iterate over each record in the CSV file:
    // let mut rdr = csv::Reader::from_path("./kc_house_data.csv")?;
    let mut rdr = csv::Reader::from_path(path)?;

    // Create two vectors to hold the data series we want to plot:
    let mut price: Vec<f64> = Vec::new();
    let mut sqft_living: Vec<f64> = Vec::new();

    // Iterate over each records in the CSV file and get the columns we are looking to plot:
    for result in rdr.records() {
        // The iterator returns a Result<StringRecord, Error> so we need to check for an error
        let record = result.expect("Unable to read file");

        // Pricing information is in the 3rd field / column:
        match record.get(2) {
            Some(i) => {
                let tmp: f64 = parse(i).unwrap();

                // If there is a price value in this record (or row), then we normalize it and put into our Vector
                price.push(tmp / 1000.0)
            }
            _ => (),
        }

        // Square-foot living is in the 6th field / column:
        match record.get(5) {
            Some(i) => sqft_living.push(i.parse::<f64>().unwrap()),
            _ => (),
        }
    }

    // Return a tuple of two vectors containing our data
    return Ok((price, sqft_living));
}

// Function for drawing the dataset as a Scatter Plot:
pub fn draw_scatter_plot(datset: &Vec<(f64, f64)>, path: &str) {
    // Create a canvas that we can plot to...
    let canvas = BitMapBackend::new(path, (600, 400)).into_drawing_area();

    // ...and give it a white background
    canvas.fill(&WHITE).unwrap();

    // Build the canvas context - essentially creating our chart:
    let mut ctx = ChartBuilder::on(&canvas)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("House Sales in King County", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..6000.0, 0.0..10000.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw - Scatter Plot (Using circles to mark each point)
    // See https://github.com/plotters-rs/plotters/blob/master/plotters/src/element/basic_shapes.rs for other shapes
    ctx.draw_series(
        datset
            .iter()
            // .map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
            .map(|point| Circle::new(*point, 2.0_f64, &BLUE.mix(0.2))),
    )
    .unwrap();
}

pub fn draw_area_plot(datset: &Vec<(f64, f64)>, path: &str) {
    let canvas = BitMapBackend::new(path, (600, 400)).into_drawing_area();
    canvas.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&canvas)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("House Sales in King County", ("sans-serif", 40.0))
        .build_cartesian_2d(0..9, 0..10000)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(datset[..10].iter().map(|x| x.1 as i32)),
            0,
            &RED.mix(0.2),
        )
        .border_style(&RED),
    )
    .unwrap();
}

// pub fn draw_bar_chart(datset: &Vec<(f64, f64)>, path: &str) {
//     let canvas = BitMapBackend::new(path, (600, 400)).into_drawing_area();
//     canvas.fill(&WHITE).unwrap();

//     let mut ctx = ChartBuilder::on(&canvas)
//         .set_label_area_size(LabelAreaPosition::Left, 40.0)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
//         .set_label_area_size(LabelAreaPosition::Right, 40.0)
//         .set_label_area_size(LabelAreaPosition::Top, 40.0)
//         .caption("House Sales in King County", ("sans-serif", 40.0))
//         .build_cartesian_2d((0..9).into_segmented(), 0..40)
//         .unwrap();

//     ctx.configure_mesh().draw().unwrap();

//     // Draw Bar Plot
//     ctx.draw_series((0..).zip(datset[..10].iter()).map(|(x, y)| {
//         let x0 = SegmentValue::Exact(x);
//         let x1 = SegmentValue::Exact(x + 1);
//         let mut bar = Rectangle::new([(x0, 0), (x1, *y)], BLUE.filled());
//         bar.set_margin(0, 0, 5, 5);
//         bar
//     }))
//     .unwrap();
// }

// pub fn draw_line_chart(datset: &Vec<(f64, f64)>, path: &str) {
//     let canvas = BitMapBackend::new(path, (600, 400)).into_drawing_area();
//     canvas.fill(&WHITE).unwrap();

//     let mut ctx = ChartBuilder::on(&canvas)
//         .set_label_area_size(LabelAreaPosition::Left, 40.0)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
//         .set_label_area_size(LabelAreaPosition::Right, 40.0)
//         .set_label_area_size(LabelAreaPosition::Top, 40.0)
//         .caption("House Sales in King County", ("sans-serif", 40.0))
//         .build_cartesian_2d((0..9).into_segmented(), 0..40)
//         .unwrap();

//     ctx.configure_mesh().draw().unwrap();

//     // Draw Line Chart
//     ctx.draw_series(LineSeries::new(
//         (0..)
//             .zip(datset[..10].iter())
//             .map(|(idx, y)| (idx as f64, *y)),
//         &BLUE,
//     ))
//     .unwrap();
// }
