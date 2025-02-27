use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() -> PolarsResult<()> {
    // Create a DataFrame manually
//     let mut df: DataFrame = df![
//         "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
//         "age" => [25, 30, 35, 40],
//         "weight" => [57.9, 72.5, 53.6, 83.1],
//         "height" => [1.56, 1.77, 1.65, 1.75],
//     ]?;

//     println!("Original DataFrame:\n{:?}", df);

//     // Write DataFrame to CSV
//     let file = File::create("output.csv")?;
//     let mut writer = CsvWriter::new(BufWriter::new(file));
//     writer.include_header(true).finish(&mut df)?;  // ✅ Pass mutable reference

//     println!("\nDataFrame successfully written to output.csv");

//     let df_csv = CsvReadOptions::default()
//     .with_infer_schema_length(None)
//     .with_has_header(true)
//     .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
//     .try_into_reader_with_file_path(Some("output.csv".into()))?
//     .finish()?;
// println!("{}", df_csv);



// let result2 = df
//     .clone()
//     .lazy()
//     .select([
//         col("name"),
//         col("age").alias("age__"),
//         (col("weight") / col("height").pow(2)).alias("bmi"),
//     ])
//     .collect()?;
// println!("{}", result2);



// let result1 = df
//     .clone()
//     .lazy()
//     .select([
//         col("name"),
//         (cols(["weight", "height"]) * lit(0.95))
//             .name()
//             .suffix("-5%"),
//     ])
//     .collect()?;
// println!("{}", result1);


// let df = CsvReadOptions::default()
//     .try_into_reader_with_file_path(Some("output.csv".into()))
//     .unwrap()
//     .finish()
//     .unwrap();
// let mask = df.column("age")?.i64()?.gt(25);
// let df_small = df.filter(&mask)?;
// #[allow(deprecated)]
// let df_agg = df_small
//     .group_by(["class"])?
//     .select(["height"])
//     .mean()?;
// println!("{:?}", df_small);


let q = LazyCsvReader::new("output.csv")
    .with_has_header(true)
    .finish()?
    .filter(col("age").gt(lit(5)))
    .group_by(vec![col("class")])
    .agg([col("height").mean()]);
let df = q.collect()?;
println!("{}", df);
// println!("{}", q.explain(true)?);

    Ok(())
}
