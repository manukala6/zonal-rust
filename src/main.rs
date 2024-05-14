use std::fs::File;

use chrono::prelude::*;
use polars::prelude::*;

fn main() {

    let occurences_df = CsvReader::from_path("data\\OBIS_anemone_occurrences_slim.tsv")
    .unwrap()
    .infer_schema(None)
    .has_header(true)
    .with_separator(b'\t')
    .finish()
    .unwrap();

    println!("Dataframe of OBIS occurence points: ");
    println!("{}", occurences_df);

    
}
