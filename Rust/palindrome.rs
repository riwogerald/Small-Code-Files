use std::io;

fn is_palindrome_simple(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn is_palindrome_optimized(s: &str) -> bool {
    let chars: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    let len = chars.len();
    for i in 0..len/2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn is_palindrome_iterator(s: &str) -> bool {
    let cleaned: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    cleaned.iter().eq(cleaned.iter().rev())
}

fn main() {
    println!("Palindrome Checker");
    println!("Enter a string to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let text = input.trim();
    
    if text.is_empty() {
        println!("Empty string entered!");
        return;
    }

    let result1 = is_palindrome_simple(text);
    let result2 = is_palindrome_optimized(text);
    let result3 = is_palindrome_iterator(text);

    // Verify all methods produce same result
    assert_eq!(result1, result2);
    assert_eq!(result2, result3);

    println!("\nInput: \"{}\"", text);
    
    if result1 {
        println!("✓ This is a palindrome!");
    } else {
        println!("✗ This is not a palindrome.");
    }

    // Demo with some example palindromes
    println!("\n--- Testing with example palindromes ---");
    let examples = vec![
        "racecar",
        "A man, a plan, a canal: Panama",
        "race a car",
        "hello",
        "Madam",
        "Was it a car or a cat I saw?",
        "No 'x' in Nixon",
    ];

    for example in examples {
        let is_pal = is_palindrome_simple(example);
        println!("\"{}\" -> {}", example, if is_pal { "✓ Palindrome" } else { "✗ Not palindrome" });
    }
}
