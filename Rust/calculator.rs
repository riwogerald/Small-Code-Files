use std::io;

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    End,
}

struct Calculator {
    tokens: Vec<Token>,
    position: usize,
}

impl Calculator {
    fn new(input: &str) -> Result<Self, String> {
        let tokens = Self::tokenize(input)?;
        Ok(Calculator { tokens, position: 0 })
    }

    fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' => {
                    chars.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '0'..='9' | '.' => {
                    let mut number_str = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            number_str.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let number: f64 = number_str.parse()
                        .map_err(|_| format!("Invalid number: {}", number_str))?;
                    tokens.push(Token::Number(number));
                }
                _ => return Err(format!("Unexpected character: {}", ch)),
            }
        }

        tokens.push(Token::End);
        Ok(tokens)
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::End)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut result = self.parse_term()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            match self.current_token() {
                Token::Plus => {
                    self.advance();
                    result += self.parse_term()?;
                }
                Token::Minus => {
                    self.advance();
                    result -= self.parse_term()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut result = self.parse_factor()?;

        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            match self.current_token() {
                Token::Multiply => {
                    self.advance();
                    result *= self.parse_factor()?;
                }
                Token::Divide => {
                    self.advance();
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.current_token() {
            Token::Number(n) => {
                let value = *n;
                self.advance();
                Ok(value)
            }
            Token::LeftParen => {
                self.advance();
                let result = self.parse_expression()?;
                if !matches!(self.current_token(), Token::RightParen) {
                    return Err("Expected closing parenthesis".to_string());
                }
                self.advance();
                Ok(result)
            }
            Token::Minus => {
                self.advance();
                Ok(-self.parse_factor()?)
            }
            _ => Err("Expected number or opening parenthesis".to_string()),
        }
    }

    fn evaluate(&mut self) -> Result<f64, String> {
        let result = self.parse_expression()?;
        if !matches!(self.current_token(), Token::End) {
            return Err("Unexpected token after expression".to_string());
        }
        Ok(result)
    }
}

fn main() {
    println!("Advanced Calculator");
    println!("Supports: +, -, *, /, parentheses, and decimal numbers");
    println!("Enter 'quit' to exit\n");

    loop {
        print!("calc> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "quit" {
            println!("Goodbye!");
            break;
        }

        match Calculator::new(input) {
            Ok(mut calc) => {
                match calc.evaluate() {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => println!("Parse Error: {}", e),
        }
    }
}