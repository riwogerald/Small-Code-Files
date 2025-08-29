use std::io;

fn factorial_recursive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn factorial_iterative(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    println!("Factorial Calculator");
    println!("Enter a number (0-20 for safety): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let number: u64 = match input.trim().parse() {
        Ok(n) if n <= 20 => n,
        Ok(_) => {
            println!("Number too large! Please enter a number between 0-20.");
            return;
        }
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    let result_recursive = factorial_recursive(number);
    let result_iterative = factorial_iterative(number);

    println!("Factorial of {} (recursive): {}", number, result_recursive);
    println!("Factorial of {} (iterative): {}", number, result_iterative);
    
    // Verify both methods give same result
    assert_eq!(result_recursive, result_iterative);
    println!("Both methods produce the same result ✓");
}
