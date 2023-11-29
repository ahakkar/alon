#![allow(unused_parens)]
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| eprintln!("Error reading input: {}", e))
        .unwrap_or_default();

    let mut n = input.trim().parse::<i64>().unwrap_or_default();

    print!("{} ", n);

    while (n != 1) {
        if (n % 2 == 0) {
            n = n/2;
            print!("{} ", n);
            continue;
        }
        
        n = n*3 +1;
        print!("{} ", n);
    }
    print!("\n");
    //println!("result is: {}", n);
}
