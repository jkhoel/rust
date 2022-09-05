use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use smartcore::{
    linalg::naive::dense_matrix::DenseMatrix, linear::linear_regression::LinearRegression,
    metrics::mean_squared_error, model_selection::train_test_split,
};

// STRUCTURE DEFINING HOW EACH ROW IN THE DATASET WILL LOOK LIKE
#[derive(Debug)]
pub struct HousingDatasetRow {
    // to find feature reference: https://www.cs.toronto.edu/~delve/data/boston/bostonDetail.html
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    black: f64,
    lstat: f64,
    medv: f64,
}

impl HousingDatasetRow {
    pub fn new(v: Vec<&str>) -> HousingDatasetRow {
        // Looking at the data-set we only have numerical values. So we will just unwrap everything into a vector of f64s
        let unwrapped_row: Vec<f64> = v.iter().map(|r| r.parse().unwrap()).collect();

        HousingDatasetRow {
            crim: unwrapped_row[0],
            zn: unwrapped_row[1],
            indus: unwrapped_row[2],
            chas: unwrapped_row[3],
            nox: unwrapped_row[4],
            rm: unwrapped_row[5],
            age: unwrapped_row[6],
            dis: unwrapped_row[7],
            rad: unwrapped_row[8],
            tax: unwrapped_row[9],
            ptratio: unwrapped_row[10],
            black: unwrapped_row[11],
            lstat: unwrapped_row[12],
            medv: unwrapped_row[13],
        }
    }

    pub fn get_training_features(&self) -> Vec<f64> {
        // This function returns all the covariates that we will use to predict some outcome
        vec![
            self.crim,
            self.zn,
            self.indus,
            self.chas,
            self.nox,
            self.rm,
            self.age,
            self.dis,
            self.rad,
            self.tax,
            self.ptratio,
            self.black,
            self.lstat,
        ]
    }

    pub fn get_training_target(&self) -> f64 {
        // This function will return the target value in the data-set - aka the value we will be trying to predict
        // once we have the algorithm trained - namely the price
        self.medv
    }
}

// CONVERTS A ROW FROM THE CSV FILE INTO A HousingDatasetRow STRUCT
fn read_single_line(s: String) -> HousingDatasetRow {
    // Create a vector holding each value for the row in the dataset. Note that in this particular
    // case the CSV file is actually whitespace delimited.
    let row_values: Vec<&str> = s.split_whitespace().collect();

    // Make the vector into a struct
    HousingDatasetRow::new(row_values)
}

// READS THE HOUSING SET CSV FILE
pub fn read_housing_csv(path: impl AsRef<Path>) -> Vec<HousingDatasetRow> {
    let file = File::open(path).expect("Could not open file");

    // Check the documentation on BufReader as to why we are using it here
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .map(|(index, line)| line.expect(&format!("Impossible to read line number {}", index)))
        .map(|row| read_single_line(row))
        .collect()
}

// RUNS OUR LINEAR REGRESSION AND PROVIDES AN ESTIMATED PRICE
pub fn run_regression_no_polars(path: String) -> f64 {
    println!("\n => Reading csv file {}", path);
    let input_data = read_housing_csv(path);

    // Get total number of rows in our dataset
    let input_data_size = input_data.len();

    // Our observation (x) values corresponds to 13 columns in the CSV
    let n_cols = usize::try_from(13).unwrap();

    // Vector of our observations from each row in the CSV file:
    let x: Vec<f64> = input_data
        .iter()
        .flat_map(|row| row.get_training_features())
        .collect();

    // Vector of our target values from each row in the CSV file:
    let y: Vec<f64> = input_data
        .iter()
        .map(|row| row.get_training_target())
        .collect();

    // Create a DenseMatrix input_data_size long and n_cols wide, with our x values in it
    let x_matrix = DenseMatrix::from_array(input_data_size, n_cols, &x);

    // Split our dataset into a training set and a testing set. Here, 30% of the rows in the CSV will be used to test
    // our prediction algorithm, and the rest will be for training it
    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &y, 0.3, true);

    // Create a fitting model using our training set
    let lr = LinearRegression::fit(&x_train, &y_train, Default::default()).unwrap();

    // Feed our testing dataset into the prediction algorithm
    let predictions = lr.predict(&x_test).unwrap();
    println!(" => Created model and predictions. Comparing to find MSE...");

    // Since we know the _actual_ values for the test set, we can check the prediction against it to see the error
    let mse = mean_squared_error(&y_test, &predictions);

    // Finally, we print and return the mse
    println!(" => Mean Squared Error: {:?}\n\n", mse);
    mse
}
