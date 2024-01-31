#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::let_and_return)]
#![allow(dead_code)]
use std::io::{self, BufWriter, Write};

/*
1 Sort the Stays by Departure Date: First, sort all customer stays by their 
  departure date.
2 Allocate Rooms: Iterate through the sorted list of stays. For each stay:
    - If there is a room that becomes free before the arrival date of this
      stay, allocate that room to this customer.
    - If there is no such room, allocate a new room.
3 Count the Rooms: The total number of rooms needed is the maximum number of 
  rooms in use at any point in time.
 */

fn main() {
    let mut input = String::new();    
    let mut dates: Vec<(i32, i32)> = Vec::new();  
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap_or_default();
    input.clear();

    for i in 0..n {
        let (a, b) = input
            .trim()
            .split_once(' ')
            .map(|(s, e)| (s.parse::<i32>().unwrap(), e.parse::<i32>().unwrap()))
            .unwrap();
        input.clear();
        dates.push((a,b));
    }

    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{}", -1).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_solve() {

    }
}
