extern crate collections;

use std::io;
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<int, int> = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let data: Vec<&str> = line.as_slice().split('\t').collect();

        match data.as_slice() {
            [year, temp] => {
                let year = from_str::<int>(year).unwrap();
                let temp = from_str::<int>(temp.trim_right()).unwrap();
                let cmp = |_: &int, t: &mut int| if temp > *t { *t = temp };

                map.insert_or_update_with(year, temp, cmp);
            },
            _ => {}
        }
    }

    for (y, t) in map.iter() { println!("{}\t{}", y, t); }
}
