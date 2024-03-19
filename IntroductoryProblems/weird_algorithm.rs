// use std::io::BufRead;
 
fn main() {
    // let input: u64 = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let input:u64 = std::io::read_to_string(std::io::stdin()).unwrap().trim().parse().unwrap();
    let mut n = input;
    while n!=1{
        print!("{} ", n);
        if n%2==0{
            n = n/2;
        }else{
            n = n*3+1;
        }
    }
    print!("1");
}
