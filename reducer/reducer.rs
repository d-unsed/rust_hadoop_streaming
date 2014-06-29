extern crate collections;

use std::io;
use collections::HashMap;

fn main() {
    let mut map: HashMap<int, int> = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let data: ~[&str] = line.split('\t').collect();

        match data.as_slice() {
            [year, temp_str] => {
                let year = from_str::<int>(year).unwrap();
                let temp = from_str::<int>(temp_str.trim_right()).unwrap();
                let cmp = |_: &int, t: &mut int| if temp > *t { *t = temp };

                map.insert_or_update_with(year, temp, cmp);
            },
            _ => {}
        }
    }

    for (y, t) in map.iter() { println!("{}\t{}", y, t); }
}
