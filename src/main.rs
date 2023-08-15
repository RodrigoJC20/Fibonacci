use std::io;

fn main() {
    println!("Choose a number to print the Fibonacci Sequence");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the number");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please, enter a valid number")
    };

    generate_fubonacci(n);
}

fn generate_fubonacci(n: u32) {
    let mut a = 0;
    let mut b = 1;
    let mut count = n;
    while count > 0 {
        print!("{} ", b);
        let c = b;
        b = b + a;
        a = c;
        count = count - 1;
    }
}
