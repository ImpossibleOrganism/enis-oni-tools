extern crate csv;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    // Output directory supplied by Cargo
    let out_dir = env::var("OUT_DIR").unwrap();

    // --- Create hash tables ---
    // Find the input file
    // TODO: The path to assets (../assets) should be an environment variable.
    let geysers_file_path = Path::new("../data/geysers.csv");
    let mut rdr = csv::Reader::from_path(geysers_file_path).expect("Failed to read CSV file");

    // Open the output file
    let dest_path = Path::new(&out_dir).join("gen_geyser_types.rs");
    let mut file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut phf_map = phf_codegen::Map::new();

    // pub struct GeyserType<'a> {
    //     name: &'a str,
    //     element: &'a str,
    //     temperature: f32,
    //     pmax: f32,
    //     yield_: f32,
    //     active: f32,
    // }

    // Deserialize
    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        let key = record
            .get(0)
            .expect("Failed to read key")
            .trim()
            .to_string();
        let value = format!(
            r#"GeyserType {{ name: {:?}, element: {:?}, temperature: {:?}, pmax: {:?}, yield_: {:?}, active: {:?} }}"#,
            key,
            record.get(1).expect("Could not read element").trim(),
            record
                .get(2)
                .expect("Could not read temperature")
                .trim()
                .parse::<f32>()
                .expect("Could not parse temperature"),
            record
                .get(3)
                .expect("Could not read pmax")
                .trim()
                .parse::<f32>()
                .expect("Could not parse pmax"),
            record
                .get(4)
                .expect("Could not read yield")
                .trim()
                .parse::<f32>()
                .expect("Could not parse yield"),
            record
                .get(5)
                .expect("Could not read active percent")
                .trim()
                .parse::<f32>()
                .expect("Could not parse active percent"),
        );
        phf_map.entry(key, &value);
    }

    // Write to file
    writeln!(&mut file, "{}", phf_map.build()).expect("Failed to write to file");
}
