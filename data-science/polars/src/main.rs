use polars::prelude::CsvReader;
use polars::prelude::DataFrame;
use polars::prelude::NamedFrom;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader;
use polars::series::Series;

// Returns a DataFrame built on two Vec<Series> where the Series are of equal length
#[allow(dead_code)]
fn dataframe_from_vectors() -> PolarResult<DataFrame> {
    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    DataFrame::new(vec![s1, s2])
}

// Reads a CSV file and creates a DataFrame using Polar's CsvReader struct
fn dataframe_from_csv(path: &String) -> PolarResult<DataFrame> {
    CsvReader::from_path(path)?.has_header(true).finish()
}

// Prints some information about the dataframe
fn print_dataframe_info(df: &DataFrame) -> () {
    println!("Shape (rows, cols):\n{:?}", df.shape());
    println!("\n{:#?}", df.schema());
    println!("Size (W x H):\n{} x {}", df.width(), df.height());
}

// Prints the column names
fn print_dataframe_column_names(df: &DataFrame) -> () {
    let columns = df.get_columns();

    for (i, column) in columns.iter().enumerate() {
        // println!("{}, {}", column, column.name());
        println!("Col #{}: {}", i + 1, column.name());
    }
}

// Stacks (concats) one DataFrame with another
fn stack_dataframes(df: &DataFrame, path: &String) -> DataFrame {
    let df2 = dataframe_from_csv(&path).expect("Unable to read file");
    let df3 = df.vstack(&df2).unwrap();

    println!(
        "{}\n ({} rows, {} columns)",
        df3.head(Some(df3.height())),
        df3.height(),
        df3.width()
    );

    df3
}

fn get_column_by_name<'a>(df: &'a DataFrame, name: &str) -> &'a Series {
    let col = df.column(name).unwrap();
    println!(
        "Found column \'{}\' with a length of {}",
        col.name(),
        col.len()
    );
    col
}

fn main() {
    let path = "./data/iris_dataset.csv".to_string();

    let df = dataframe_from_csv(&path).expect("Unable to read file");
    // let df = dataframe_from_vectors().expect("Unable to create dataframe from vectors");

    println!("\n=== Dataframe Info:");
    print_dataframe_info(&df);

    println!("\n=== Columns:");
    print_dataframe_column_names(&df);

    println!("\n=== Stacking Dataframes:");
    let mut df3 = stack_dataframes(&df, &path);

    let name = "sepal.length";
    println!("\n=== Getting a column by name \"{}\":", name);
    get_column_by_name(&df3, name);

    println!("\n=== Dropping column \'{}\':", name);
    let dropped_column = df3.drop_in_place(name).expect("Unable to drop column");
    println!("{}", df3.head(Some(5)));

    println!("\n=== Inserting column into Dataframe:");
    let _df4 = df3.insert_at_idx(df3.width(), dropped_column).unwrap();
    println!("{}", _df4.head(Some(5)));

    // TODO: ADD TRANSFORM EXAMPLES
}
