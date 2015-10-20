extern crate itertools;

use std::io;
use std::io::BufRead;
use std::string::ToString;

use itertools::Itertools;

// Remove leading `+`
// Should be deleted in Rust 1.4
fn remove_leading_plus(s: String) -> String {
    (&s[1..]).to_string()
}

fn main() {
    let reader = io::stdin();

    reader.lock().lines().map(|line| {
        // Read tab-separated lines
        let line = line.unwrap();
        let data: Vec<String> = line.split('\t').map(ToString::to_string).collect();

        (data[0].clone(), data[1].clone())
    }).map(|(year, temperature)| {
        // Parse strings to i32
        let year = year.parse::<i32>().unwrap();
        let temperature = temperature.parse::<i32>().unwrap_or_else(|_| {
            remove_leading_plus(temperature).parse::<i32>().unwrap()
        });

        (year, temperature)
    }).group_by(|&(year, _)| {
        // Group by year
        year
    }).map(|(year, groups)| {
      // Map maximum temperature of each group to years
      let max_temperature = groups.iter().map(|&(_, temperature)| temperature).max().unwrap();

      (year, max_temperature)
    }).foreach(|(year, temperature)| {
        // Print results
        println!("{}\t{}", year, temperature);
    });
}
