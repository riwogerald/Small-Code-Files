use std::io;

fn gcd_recursive(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_recursive(b, a % b)
    }
}

fn gcd_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd_recursive(a, b)
    }
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = gcd_extended(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

fn are_coprime(a: u64, b: u64) -> bool {
    gcd_recursive(a, b) == 1
}

fn simplify_fraction(numerator: u64, denominator: u64) -> (u64, u64) {
    let common_divisor = gcd_recursive(numerator, denominator);
    (numerator / common_divisor, denominator / common_divisor)
}

fn main() {
    println!("GCD and LCM Calculator");
    println!("Enter two positive integers separated by a space:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    
    if numbers.len() != 2 {
        println!("Please enter exactly two numbers separated by a space.");
        return;
    }

    let a: u64 = match numbers[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number! Please enter a valid positive integer.");
            return;
        }
    };

    let b: u64 = match numbers[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number! Please enter a valid positive integer.");
            return;
        }
    };

    if a == 0 || b == 0 {
        println!("Please enter positive integers (not zero).");
        return;
    }

    let gcd_result_recursive = gcd_recursive(a, b);
    let gcd_result_iterative = gcd_iterative(a, b);
    let lcm_result = lcm(a, b);

    // Verify both GCD methods produce same result
    assert_eq!(gcd_result_recursive, gcd_result_iterative);

    println!("\nNumbers: {} and {}", a, b);
    println!("GCD (Greatest Common Divisor): {}", gcd_result_recursive);
    println!("LCM (Least Common Multiple): {}", lcm_result);

    // Additional properties
    if are_coprime(a, b) {
        println!("✓ {} and {} are coprime (relatively prime)", a, b);
    } else {
        println!("✗ {} and {} are not coprime", a, b);
    }

    // Verify LCM property: a * b = gcd * lcm
    assert_eq!(a * b, gcd_result_recursive * lcm_result);
    println!("✓ Verification: {} × {} = {} × {} = {}", a, b, gcd_result_recursive, lcm_result, a * b);

    // Show fraction simplification
    let (simplified_num, simplified_den) = simplify_fraction(a, b);
    println!("Fraction {}/{} simplifies to {}/{}", a, b, simplified_num, simplified_den);

    // Extended Euclidean Algorithm demo
    let (gcd_ext, x, y) = gcd_extended(a as i64, b as i64);
    println!("\nExtended Euclidean Algorithm:");
    println!("GCD({}, {}) = {}", a, b, gcd_ext);
    println!("Bézout coefficients: {} × {} + {} × {} = {}", a, x, b, y, gcd_ext);
    
    // Verify Bézout identity
    let verification = (a as i64) * x + (b as i64) * y;
    assert_eq!(verification, gcd_ext);
    println!("✓ Bézout identity verified!");
}
