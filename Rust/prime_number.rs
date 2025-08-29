use std::io;

fn is_prime_basic(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_prime_sqrt(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn generate_primes_sieve(limit: usize) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    (2..=limit)
        .filter(|&i| is_prime[i])
        .map(|i| i as u64)
        .collect()
}

fn main() {
    println!("Prime Number Checker");
    println!("Enter a number to check if it's prime:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let number: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    let is_prime_result = is_prime_basic(number);
    let is_prime_sqrt_result = is_prime_sqrt(number);

    // Verify both methods produce same result
    assert_eq!(is_prime_result, is_prime_sqrt_result);

    println!("\nNumber: {}", number);
    
    if is_prime_result {
        println!("✓ {} is a prime number!", number);
    } else {
        println!("✗ {} is not a prime number.", number);
    }

    // Show prime factorization for non-primes
    if !is_prime_result && number > 1 {
        print!("Prime factors: ");
        let mut factors = Vec::new();
        let mut n = number;
        let mut d = 2;
        
        while d * d <= n {
            while n % d == 0 {
                factors.push(d);
                n /= d;
            }
            d += 1;
        }
        if n > 1 {
            factors.push(n);
        }
        
        for (i, factor) in factors.iter().enumerate() {
            if i > 0 { print!(" × "); }
            print!("{}", factor);
        }
        println!();
    }

    // Demo: Show first few primes
    if number <= 100 {
        println!("\n--- Prime numbers up to 100 ---");
        let primes = generate_primes_sieve(100);
        for (i, prime) in primes.iter().enumerate() {
            if i > 0 && i % 10 == 0 {
                println!();
            }
            print!("{:3} ", prime);
        }
        println!("\n\nTotal primes up to 100: {}", primes.len());
    }
}
