use medium_linear_regression::*;

fn main() {
    let path = "./data/boston_house.csv";

    run_regression_no_polars(path.to_string());
}
