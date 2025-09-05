use std::io;
use rand::Rng;

struct PasswordConfig {
    length: usize,
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    exclude_ambiguous: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        PasswordConfig {
            length: 12,
            include_lowercase: true,
            include_uppercase: true,
            include_numbers: true,
            include_symbols: false,
            exclude_ambiguous: false,
        }
    }
}

struct PasswordGenerator {
    lowercase: String,
    uppercase: String,
    numbers: String,
    symbols: String,
    ambiguous: String,
}

impl PasswordGenerator {
    fn new() -> Self {
        PasswordGenerator {
            lowercase: "abcdefghijklmnopqrstuvwxyz".to_string(),
            uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            numbers: "0123456789".to_string(),
            symbols: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            ambiguous: "0O1lI".to_string(),
        }
    }

    fn build_charset(&self, config: &PasswordConfig) -> String {
        let mut charset = String::new();

        if config.include_lowercase {
            charset.push_str(&self.lowercase);
        }
        if config.include_uppercase {
            charset.push_str(&self.uppercase);
        }
        if config.include_numbers {
            charset.push_str(&self.numbers);
        }
        if config.include_symbols {
            charset.push_str(&self.symbols);
        }

        if config.exclude_ambiguous {
            charset = charset
                .chars()
                .filter(|c| !self.ambiguous.contains(*c))
                .collect();
        }

        charset
    }

    fn generate_password(&self, config: &PasswordConfig) -> Result<String, String> {
        let charset = self.build_charset(config);
        
        if charset.is_empty() {
            return Err("No character types selected!".to_string());
        }

        let mut rng = rand::thread_rng();
        let charset_chars: Vec<char> = charset.chars().collect();
        
        let password: String = (0..config.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset_chars.len());
                charset_chars[idx]
            })
            .collect();

        Ok(password)
    }

    fn calculate_strength(&self, password: &str) -> f64 {
        let mut charset_size = 0;
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_symbol = false;

        for ch in password.chars() {
            if ch.is_ascii_lowercase() && !has_lower {
                has_lower = true;
                charset_size += 26;
            } else if ch.is_ascii_uppercase() && !has_upper {
                has_upper = true;
                charset_size += 26;
            } else if ch.is_ascii_digit() && !has_digit {
                has_digit = true;
                charset_size += 10;
            } else if !ch.is_alphanumeric() && !has_symbol {
                has_symbol = true;
                charset_size += 32;
            }
        }

        if charset_size == 0 {
            return 0.0;
        }

        // Calculate entropy: log2(charset_size^length)
        let entropy = password.len() as f64 * (charset_size as f64).log2();
        
        // Convert to strength score (0-100)
        (entropy / 128.0 * 100.0).min(100.0)
    }

    fn get_strength_label(&self, strength: f64) -> &str {
        match strength {
            s if s < 25.0 => "Very Weak",
            s if s < 50.0 => "Weak",
            s if s < 75.0 => "Good",
            s if s < 90.0 => "Strong",
            _ => "Very Strong",
        }
    }

    fn display_strength_bar(&self, strength: f64) {
        let bars = (strength / 5.0) as usize;
        print!("Strength Bar: [");
        for i in 0..20 {
            if i < bars {
                print!("=");
            } else {
                print!(" ");
            }
        }
        println!("]");
    }
}

fn get_user_config() -> PasswordConfig {
    let mut config = PasswordConfig::default();

    println!("\n=== Password Configuration ===");
    
    print!("Password length (4-128) [default: 12]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(length) = input.trim().parse::<usize>() {
        if (4..=128).contains(&length) {
            config.length = length;
        }
    }

    print!("Include lowercase letters? (y/n) [default: y]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    config.include_lowercase = !input.trim().eq_ignore_ascii_case("n");

    print!("Include uppercase letters? (y/n) [default: y]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    config.include_uppercase = !input.trim().eq_ignore_ascii_case("n");

    print!("Include numbers? (y/n) [default: y]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    config.include_numbers = !input.trim().eq_ignore_ascii_case("n");

    print!("Include symbols? (y/n) [default: n]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    config.include_symbols = input.trim().eq_ignore_ascii_case("y");

    print!("Exclude ambiguous characters (0,O,1,l,I)? (y/n) [default: n]: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    config.exclude_ambiguous = input.trim().eq_ignore_ascii_case("y");

    config
}

fn main() {
    println!("🔐 Secure Password Generator");
    println!("============================");

    let generator = PasswordGenerator::new();

    loop {
        println!("\nOptions:");
        println!("1. Generate password with default settings");
        println!("2. Generate password with custom settings");
        println!("3. Check password strength");
        println!("4. Generate multiple passwords");
        println!("5. Exit");
        print!("Choose an option (1-5): ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                let config = PasswordConfig::default();
                match generator.generate_password(&config) {
                    Ok(password) => {
                        let strength = generator.calculate_strength(&password);
                        println!("\nGenerated Password: {}", password);
                        println!("Length: {} characters", password.len());
                        println!("Strength: {:.1}% ({})", strength, generator.get_strength_label(strength));
                        generator.display_strength_bar(strength);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                let config = get_user_config();
                match generator.generate_password(&config) {
                    Ok(password) => {
                        let strength = generator.calculate_strength(&password);
                        println!("\nGenerated Password: {}", password);
                        println!("Length: {} characters", password.len());
                        println!("Strength: {:.1}% ({})", strength, generator.get_strength_label(strength));
                        generator.display_strength_bar(strength);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                print!("Enter password to analyze: ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();
                
                if !password.is_empty() {
                    let strength = generator.calculate_strength(password);
                    println!("\nPassword Analysis:");
                    println!("Password: {}", password);
                    println!("Length: {} characters", password.len());
                    println!("Strength: {:.1}% ({})", strength, generator.get_strength_label(strength));
                    generator.display_strength_bar(strength);
                }
            }
            "4" => {
                print!("How many passwords to generate? ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut count_input = String::new();
                io::stdin().read_line(&mut count_input).unwrap();
                
                if let Ok(count) = count_input.trim().parse::<usize>() {
                    if count > 0 && count <= 20 {
                        let config = PasswordConfig::default();
                        println!("\nGenerated {} passwords:", count);
                        println!("------------------------");
                        
                        for i in 1..=count {
                            match generator.generate_password(&config) {
                                Ok(password) => println!("{}. {}", i, password),
                                Err(e) => println!("{}. Error: {}", i, e),
                            }
                        }
                    } else {
                        println!("Please enter a number between 1 and 20.");
                    }
                } else {
                    println!("Invalid number!");
                }
            }
            "5" => {
                println!("Thank you for using the Password Generator!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-5.");
            }
        }
    }
}