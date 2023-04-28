use num_bigint::{BigInt};
use std::io;

fn main() {

    println!("Please enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u64 = input.trim().parse().expect("Invalid input");

    println!("Please enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let k: u64 = input.trim().parse().expect("Invalid input");

    let c = binomial_coefficients(n, k);

    println!("The binomial coefficient of {n} and {k} is {c}");

}

fn factorial(n: u64) -> BigInt {

    let n = n;
    let mut x = BigInt::from(1);

    for _i in 1..n {

        x *= &1 + &_i;

    }

    x
}

fn binomial_coefficients(n: u64, k: u64) -> BigInt {

    let n = n;
    let k = k;

    let c = factorial(n) / (factorial(k) * factorial(n-k));

    c
}