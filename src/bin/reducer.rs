use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::string::ToString;

// Remove leading `+`
// Should be deleted in Rust 1.4
fn remove_leading_plus(s: String) -> String {
    (&s[1..]).to_string()
}

fn main() {
    let reader = io::stdin();

    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    reader.lock().lines().map(|line| {
        let line = line.unwrap();
        let data: Vec<String> = line.split('\t').map(ToString::to_string).collect();

        (data[0].clone(), data[1].clone())
    }).map(|(year, temperature)| {
        let year = year.parse::<i32>().unwrap();
        let temperature = temperature.parse::<i32>().unwrap_or_else(|_| {
            remove_leading_plus(temperature).parse::<i32>().unwrap()
        });

        (year, temperature)
    }).scan(&mut hash_map, |hash_map, (year, temperature)| {
        hash_map
            .get_mut(&year)
            .map(|e| if temperature > *e { *e = temperature })
            .or_else(|| { hash_map.insert(year, temperature); Some(()) });

            Some((year, temperature))
    }).collect::<Vec<(i32, i32)>>();

    for (year, temperature) in hash_map.iter() {
        println!("{}\t{}", year, temperature);
    }
}
