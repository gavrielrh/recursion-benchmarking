#![feature(test)]

extern crate test;
extern crate rand;

use std::error::Error;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

// Gets sorting time of algorithm in ms
fn get_sort_time(mut v: Vec<i32>, f: fn(&mut Vec<i32>)) -> u64 {
    let start = Instant::now();
    f(&mut v);
    start.elapsed().as_secs() * 1000 + start.elapsed().subsec_nanos() as u64 / 1_000_000
}

// Gets fibonacci time of algorithm in ms
fn get_fibo_time(n: u64, f: fn(u64) -> u64) -> u64 {
    let start = Instant::now();
    f(n);
    start.elapsed().as_secs() * 1000 + start.elapsed().subsec_nanos() as u64 / 1_000_000
}

// Regular recursive implementation of fibonacci
pub fn fibo_rec(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    fibo_rec(n - 1) + fibo_rec(n - 2)
}

// Wrapper for memoized fibonacci function. Uses staticly sized memoization buffer
pub fn fibo_rec_memoized(n: usize) -> u64 {
    let mut memo = vec![0; n + 1];
    real_fibo_rec_memoized(n, &mut memo)
}

// My memoized fibonacci function
pub fn real_fibo_rec_memoized(n: usize, mut memo: &mut Vec<u64>) -> u64 {
    if n <= 2 {
        return 1;
    }

    if memo[n] != 0 {
        return memo[n];
    }

    memo[n] = real_fibo_rec_memoized(n - 1, &mut memo) + real_fibo_rec_memoized(n - 2, &mut memo);
    memo[n]
}

// Iterative implementation of fibonacci
pub fn fibo_iter(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;
    for i in 3..(n+1) {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

// Helper function for quicksort
pub fn partition(a: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let mut i = low - 1;
    let pivot = a[high as usize];

    for j in low..high {
        if a[j as usize] <= pivot {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i+1) as usize, high as usize);

    i+1
}

// Wrapper for recursive quicksort function
pub fn quicksort_rec(mut a: &mut Vec<i32>) {
    let n = a.len() as i32;
    real_quicksort_rec(&mut a, 0, n - 1);
}

// Recursive implementation of quicksort
pub fn real_quicksort_rec(a: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let p = partition(a, low, high);

        real_quicksort_rec(a, low, p - 1);
        real_quicksort_rec(a, p+1, high);
    }
}

// Iterative implementation of quicksort
pub fn quicksort_iter(mut a: &mut Vec<i32>) {
    let mut stack = Vec::new();
    let mut start = 0;
    let mut end = a.len() as i32 - 1;

    stack.push((start, end));

    while !stack.is_empty() {
        let x = stack.pop().unwrap();
        start = x.0;
        end = x.1;

        let pivot = partition(&mut a, start, end);

        if pivot - 1 > start {
            stack.push((start, pivot - 1));
        }

        if pivot + 1 < end {
            stack.push((pivot + 1, end));
        }
    }
}

fn main() {
    //make_test_vec_files(10, 10000);
    // let mut v = read_vector_from_file("vec10.txt");
    // println!("{:?}", v);
    // quicksort_iter(&mut v);
    // println!("{:?}", v);
    println!("{}", fibo_rec_memoized(50));
}

// Generates vectors of size start, multiplied by 10, until reaching size end, and saves them to files
fn make_test_vec_files(start: i32, end: i32) {
    let mut n = start;

    let mut v: Vec<i32> = Vec::new();

    while n <= end {
        let v = get_rand_vector(n as usize);
        save_vec_to_file(v);
        n *= 10;
    }
}

/*
Taken from: http://www.programming-idioms.org/idiom/15/pick-uniformly-a-random-integer-in-a-b/501/rust
Because Rust is weird about its docs and rand is apparently internal stuff :(
*/
fn pick(a: i32, b: i32) -> i32 {
    let between = Range::new(a, b);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

// Generates random vector of size len with i32s between -99 and 99 inclusive
fn get_rand_vector(len: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        v.push(pick(-99, 100));
    }
    v
}

// Saves vector (of length #) to file with name: vec#.txt
fn save_vec_to_file(v: Vec<i32>) {
    let n = v.len();
    let path_str = format!("vec{}.txt", n);
    let path = Path::new(&path_str);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the vector string to `file`, returns `io::Result<()>`
    match file.write_all(format!("{:?}",v).as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

// Loads vector from file at path p
fn read_vector_from_file(p: &str) -> Vec<i32> {
    let path = Path::new(p);
    let display = path.display();
    let mut data: Vec<i32> = Vec::new();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, 
                                            why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                            why.description()),
        Ok(_) => {
                data = (&s[1..s.len()-1]).to_string().split(", ").map(|s| i32::from_str(s).unwrap()).collect();
            },
    }
    data
}

// 'cargo bench' to run these
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_quicksort_rec(b: &mut Bencher) {
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            let mut a = read_vector_from_file("src/vec10000.txt").clone();
            let n = a.len() as i32;
            test::black_box(|| quicksort_rec(&mut a));
        });
    }

    #[bench]
    fn bench_quicksort_iter(b: &mut Bencher) {
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            let mut a = read_vector_from_file("src/vec10000.txt").clone();
            let n = a.len() as i32;
            test::black_box(|| quicksort_iter(&mut a));
        });
    }

    #[bench]
    fn bench_fibonacci_iter(b: &mut Bencher) {
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            let n = 90;
            test::black_box(|| fibo_iter(n));
        });
    }

    #[bench]
    fn bench_fibonacci_rec(b: &mut Bencher) {
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            let n = 90;
            test::black_box(|| fibo_iter(n));
        });
    }

    #[bench]
    fn bench_fibonacci_rec_memoized(b: &mut Bencher) {
        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from disregarding
            // unused values
            let n = 90;
            test::black_box(|| fibo_rec_memoized(n));
        });
    }
}