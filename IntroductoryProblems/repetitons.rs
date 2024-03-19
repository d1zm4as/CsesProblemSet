use std::io::BufRead;
 
fn main() {
    let input: String = std::io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", repetitions(input));
}
 
fn repetitions(input: String) -> u64 {
    let mut ant: char = input.chars().next().unwrap();
     let mut cont: u64 = 0;
     let mut maior: u64 = 1;
     for x in input.chars() {
         if x== ant {
             cont += 1;
             maior = std::cmp::max(maior, cont);
             continue;
         }
         cont = 1;
         ant = x;
     }
    maior
}
