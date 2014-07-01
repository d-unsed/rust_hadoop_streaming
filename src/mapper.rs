use std::io;

fn main() {
    let quality = ['0', '1', '4', '5', '9'];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let year = line.slice(15, 19);
        let temperature = line.slice(87, 92);
        let q = line[92];

        if temperature != "+9999" && quality.iter().any(|e| q == *e as u8) {
            println!("{}\t{}", year, temperature);
        }
    }
}
