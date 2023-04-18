fn main() {

    let p: i128 = 66176147051089903323586007;

    let mut a_n_1: Vec<i128> = vec![0, 0];
    let mut a_n: Vec<i128> = vec![1];

    for i in 0..125 {

        let x = (a_n[i] + a_n_1[i]) % p;

        a_n_1.push(x);
        a_n.push(x);
        
        let i_plus_1 = i + 1;

        println!("{i_plus_1}. člen Fibonacciho postupnosti: {x}");

    }
}