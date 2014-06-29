use std::io;
use std::cmp::max;

fn main() {
    let mut max_temp = std::int::MIN;
    let mut last_year: &str = "";

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let data: ~[&str] = line.split('\t').collect();

        match data.as_slice() {
            [year, temp] => {
                let temp = from_str::<int>(temp.slice_to(temp.len() - 1)).unwrap();
                if last_year != "" && last_year != year {
                    println!("{}\t{}", last_year, max_temp);
                    max_temp = temp;
                } else {
                    max_temp = max(max_temp, temp);
                }

                last_year = year;
            },
            _ => {}
        }
    }

    if last_year != "" { println!("{}\t{}", last_year, max_temp); }
}
