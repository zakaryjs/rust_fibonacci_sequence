use std::io;

fn main() {
    loop {

        println!("Input the number you would like to see the Fibonacci Sequence for:");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Error.");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        for num in 0..number + 1 {
            println!("fibonacci - {num}: {}", fibonacci_sequence(num));
        }
    }
}

fn fibonacci_sequence(num: u32) -> u32 {

    if num == 0 {
        return 0
    }
    if num == 1 {
        return 1
    }
    if num == 2 {
        return num
    }
    else {
        return fibonacci_sequence(num - 1) + fibonacci_sequence(num - 2);
    }
}