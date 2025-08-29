use std::io;

fn fibonacci_vector(n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return vec![0];
    }
    
    let mut fib = vec![0, 1];
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    fib
}

fn fibonacci_iterative(n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return vec![0];
    }
    
    let mut result = Vec::with_capacity(n);
    result.push(0);
    result.push(1);
    
    let (mut a, mut b) = (0, 1);
    for _ in 2..n {
        let next = a + b;
        result.push(next);
        a = b;
        b = next;
    }
    result
}

fn fibonacci_nth(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of Fibonacci numbers to generate: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let count: usize = match input.trim().parse() {
        Ok(n) if n <= 50 => n,
        Ok(_) => {
            println!("Number too large! Please enter a number between 0-50.");
            return;
        }
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    if count == 0 {
        println!("No Fibonacci numbers to display.");
        return;
    }

    let fib_sequence = fibonacci_vector(count);
    
    println!("\nFirst {} Fibonacci numbers:", count);
    for (i, &num) in fib_sequence.iter().enumerate() {
        if i == fib_sequence.len() - 1 {
            println!("{}", num);
        } else {
            print!("{}, ", num);
        }
    }

    // Demo: Show the nth Fibonacci number
    if count > 0 {
        let nth = fibonacci_nth((count - 1) as u32);
        println!("\nThe {}th Fibonacci number is: {}", count, nth);
    }
}
