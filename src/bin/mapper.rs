extern crate itertools;

use std::io;
use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let reader = io::stdin();

    let valid_quality = vec!['0', '1', '4', '5', '9'];

    reader.lock().lines().map(|line| {
        // Read input lines and slice data
        let line = line.unwrap().to_string();

        let year = (&line[15..19]).to_string();
        let temperature = (&line[87..92]).to_string();
        let quality = line.chars().nth(92).unwrap();

        (year, temperature, quality)
    }).filter(|&(_, ref temperature, ref quality)| {
        // Filter records with proper quality
        temperature != "+9999" && valid_quality.contains(&quality)
    }).foreach(|(year, temperature, _)| {
        // Print results
        println!("{}\t{}", year, temperature);
    });
}
