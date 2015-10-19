use std::io;
use std::io::BufRead;

fn main() {
    let reader = io::stdin();

    let valid_quality = vec!['0', '1', '4', '5', '9'];

    let data: Vec<(String, String, char)> = reader.lock().lines().map(|line| {
        let line = line.unwrap().to_string();

        ((&line[15..19]).to_string(), (&line[87..92]).to_string(), line.chars().nth(92).unwrap())
    }).filter(|&(_, ref temperature, ref quality)| {
        temperature != "+9999" && valid_quality.contains(&quality)
    }).collect();

    for (year, temperature, _) in data {
        println!("{}\t{}", year, temperature);
    }
}
