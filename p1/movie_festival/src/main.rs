#![allow(unused_parens)]
#![allow(unused_variables)]
use std::{io, fmt};

struct Movie {
    s: usize,
    e: usize,
}

impl fmt::Debug for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.s, self.e)
    }
}

fn main() {
    let data = read_input();
    println!("{}", process(data));
}

fn read_input() -> Vec<Movie> {
    let mut input = String::new();  
    // gather input from user    
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<i64>().unwrap();    
    let mut movies: Vec<Movie> = vec![];

    for _ in 0..n {
        input.clear();
        // parse input like "5 2" to a movie with start and end time
        io::stdin().read_line(&mut input).unwrap();
        let (s, e) = input
            .trim()
            .split_once(' ')
            .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
            .unwrap();

        movies.push(Movie{s, e});
    }
    movies
}

/*
 *  https://en.wikipedia.org/wiki/Interval_scheduling
 */
fn process(mut data: Vec<Movie>) -> usize {
    let mut selected = 1;

    data.sort_by(|a, b| a.e.partial_cmp(&b.e).unwrap());
    //println!("sorted data: {:?}", data);

    let mut current_end = data[0].e;

    for movie in data.iter().skip(1) {
        if movie.s >= current_end {
            selected += 1;
            current_end = movie.e;
        }
    }

    selected
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_process() {
        let test_data: Vec<Movie> = 
            vec![Movie{s:3, e:5}, Movie{s:4, e:9}, Movie{s:5, e:8}];
        assert_eq!(process(test_data), 2);
    }

    #[test]
    fn test_process2() {
        let test_data: Vec<Movie> =
        vec![
            Movie{s:6, e:7},
            Movie{s:4, e:5},
            Movie{s:8, e:9},
            Movie{s:2, e:3},
            Movie{s:10, e:11},
            Movie{s:1, e:2},
            Movie{s:9, e:10},
            Movie{s:3, e:4},
            Movie{s:5, e:6},
            Movie{s:7, e:8},
        ];
        assert_eq!(process(test_data), 10);
    }
}
