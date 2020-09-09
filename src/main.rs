use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use::std::io;

// added num_bigint for bigger numbers
// without max seems to be 187


fn main() {

    println!("How many Fibonacci numbers to print?");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    let choice : usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Printing {} Fibonacci numbers..", choice);
    print_fibonacci_numbers(choice);

    fn print_fibonacci_numbers(choice : usize) {
        let mut counter: usize = 1;
        let mut f0: BigUint = Zero::zero();
        let mut f1: BigUint = One::one();
        for _ in 0..choice {
            let f2 = f0 + &f1;
            // This is a low cost way of swapping f0 with f1 and f1 with f2.
            f0 = replace(&mut f1, f2);
            println!("Fibonacci number {}: {}",counter, f0);
            counter += 1;
        }
    }
}