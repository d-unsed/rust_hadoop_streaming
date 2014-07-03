use std::io;

fn main() {
    let quality = ['0', '1', '4', '5', '9'];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let l = line.as_slice();

        let year = l.slice(15, 19);
        let temperature = l.slice(87, 92);
        let q = l[92];

        if temperature != "+9999" && quality.iter().any(|e| q == *e as u8) {
            println!("{}\t{}", year, temperature);
        }
    }
}
