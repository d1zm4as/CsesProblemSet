use std::io::BufRead;
 
fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let n: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let linhas: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .sum();
    println!("{}", (n * (n + 1)) / 2 - linhas);
 
}