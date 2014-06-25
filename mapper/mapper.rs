use std::io;

fn main() {
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let year = line.slice(15, 19);
        let temperature = line.slice(87, 92);

        if temperature != "+9999" {
            println!("{}\t{}", year, temperature);
        }
    }
}
