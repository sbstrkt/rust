use std::io;

// https://dev.to/kelvinkirima014/
// https://jeremybytes.blogspot.com/loop
// https://github.com/rust-learn

fn main() {
    loop {
            
    // input number    

    println!("Please input a positive number:");
   
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u64 = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            println!("Input was not a positive integer!");
            continue;
        }
    };

    // show ascending nth and Fibonacci
    let _i: u64; 
        for i in 0..=input {
            println!("Fibonacci ({}) => {}", i, fib(i));
        }
    }
}

// Fibonacci logic
fn fib (n: u64) -> u64 {
   if n <= 0 {
       return 0;
   } else if n == 1 {
       return 1;
   }   fib(n - 1) + (fib(n - 2))
}

// TODO tests


