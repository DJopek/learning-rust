use num_bigint::{BigInt};
use std::io;

fn main() {

    // input n
    println!("Please enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u64 = input.trim().parse().expect("Invalid input");

    println!("Please enter a number:");

    // input k
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let k: u64 = input.trim().parse().expect("Invalid input");


    // calculating binomial coefficient
    let c = binomial_coefficients(n, k);

    println!("The binomial coefficient of {n} and {k} is {c}");


    // calculating how many commands you can create with n symbols without repetitive use of symbols
    let mut sum = BigInt::from(0);

    for _i in 1..n+1 {
        
        let c = binomial_coefficients(n, _i)*factorial(_i);

        sum += c;

    }
    
    println!("The sum of how many commands you can create with n symbols without repetitive use of symbols is: {sum}");

}

// factorial function
fn factorial(n: u64) -> BigInt {

    let n = n;
    let mut x = BigInt::from(1);

    for _i in 1..n {

        x *= &1 + &_i;

    }

    x
}

// binomial coefficient function
fn binomial_coefficients(n: u64, k: u64) -> BigInt {

    let n = n;
    let k = k;

    let c = factorial(n) / (factorial(k) * factorial(n-k));

    c
}