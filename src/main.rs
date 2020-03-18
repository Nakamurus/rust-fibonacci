use std::io;

fn main() {
    println!("Which term of Fibonacci sequence do you want to know?");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().unwrap();

    fibonacci(number)
}

fn fibonacci(n: i32) {
    let five_root = (5 as f64).sqrt();

    let fibonacci_num: f64 =
        (1.0 / five_root) * (((1.0 + five_root) / 2.0).powi(n) - ((1.0 - five_root) / 2.0).powi(n));
    println!(
        "The number of {} term of Fibonacci is: {}",
        n, fibonacci_num
    );
}
