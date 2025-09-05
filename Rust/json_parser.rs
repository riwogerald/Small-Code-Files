use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

struct JsonParser {
    input: Vec<char>,
    position: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        JsonParser {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.current_char() != Some('"') {
            return Err("Expected opening quote".to_string());
        }
        self.advance(); // Skip opening quote

        let mut result = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                match self.current_char() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('/') => result.push('/'),
                    Some(c) => result.push(c),
                    None => return Err("Unexpected end of input in string".to_string()),
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string".to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let mut number_str = String::new();
        
        // Handle negative numbers
        if self.current_char() == Some('-') {
            number_str.push('-');
            self.advance();
        }

        // Parse digits before decimal point
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Parse decimal point and digits after
        if self.current_char() == Some('.') {
            number_str.push('.');
            self.advance();
            
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    number_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Parse exponent
        if matches!(self.current_char(), Some('e') | Some('E')) {
            number_str.push('e');
            self.advance();
            
            if matches!(self.current_char(), Some('+') | Some('-')) {
                number_str.push(self.current_char().unwrap());
                self.advance();
            }
            
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    number_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        number_str.parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", number_str))
    }

    fn parse_array(&mut self) -> Result<Vec<JsonValue>, String> {
        if self.current_char() != Some('[') {
            return Err("Expected opening bracket".to_string());
        }
        self.advance(); // Skip '['
        self.skip_whitespace();

        let mut array = Vec::new();

        // Handle empty array
        if self.current_char() == Some(']') {
            self.advance();
            return Ok(array);
        }

        loop {
            let value = self.parse_value()?;
            array.push(value);
            
            self.skip_whitespace();
            
            match self.current_char() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                }
                Some(']') => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }

        Ok(array)
    }

    fn parse_object(&mut self) -> Result<HashMap<String, JsonValue>, String> {
        if self.current_char() != Some('{') {
            return Err("Expected opening brace".to_string());
        }
        self.advance(); // Skip '{'
        self.skip_whitespace();

        let mut object = HashMap::new();

        // Handle empty object
        if self.current_char() == Some('}') {
            self.advance();
            return Ok(object);
        }

        loop {
            // Parse key
            let key = self.parse_string()?;
            self.skip_whitespace();

            // Expect colon
            if self.current_char() != Some(':') {
                return Err("Expected ':' after object key".to_string());
            }
            self.advance();
            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            
            self.skip_whitespace();
            
            match self.current_char() {
                Some(',') => {
                    self.advance();
                    self.skip_whitespace();
                }
                Some('}') => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }

        Ok(object)
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        
        match self.current_char() {
            Some('"') => Ok(JsonValue::String(self.parse_string()?)),
            Some('[') => Ok(JsonValue::Array(self.parse_array()?)),
            Some('{') => Ok(JsonValue::Object(self.parse_object()?)),
            Some('t') => {
                if self.input[self.position..].starts_with(&['t', 'r', 'u', 'e']) {
                    self.position += 4;
                    Ok(JsonValue::Bool(true))
                } else {
                    Err("Invalid literal".to_string())
                }
            }
            Some('f') => {
                if self.input[self.position..].starts_with(&['f', 'a', 'l', 's', 'e']) {
                    self.position += 5;
                    Ok(JsonValue::Bool(false))
                } else {
                    Err("Invalid literal".to_string())
                }
            }
            Some('n') => {
                if self.input[self.position..].starts_with(&['n', 'u', 'l', 'l']) {
                    self.position += 4;
                    Ok(JsonValue::Null)
                } else {
                    Err("Invalid literal".to_string())
                }
            }
            Some(ch) if ch.is_ascii_digit() || ch == '-' => {
                Ok(JsonValue::Number(self.parse_number()?))
            }
            Some(ch) => Err(format!("Unexpected character: {}", ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse(&mut self) -> Result<JsonValue, String> {
        let value = self.parse_value()?;
        self.skip_whitespace();
        
        if self.position < self.input.len() {
            return Err("Unexpected characters after JSON value".to_string());
        }
        
        Ok(value)
    }
}

fn pretty_print_json(value: &JsonValue, indent: usize) {
    let indent_str = "  ".repeat(indent);
    
    match value {
        JsonValue::Null => print!("null"),
        JsonValue::Bool(b) => print!("{}", b),
        JsonValue::Number(n) => {
            if n.fract() == 0.0 {
                print!("{}", *n as i64);
            } else {
                print!("{}", n);
            }
        }
        JsonValue::String(s) => print!("\"{}\"", s),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                print!("[]");
            } else {
                println!("[");
                for (i, item) in arr.iter().enumerate() {
                    print!("{}", "  ".repeat(indent + 1));
                    pretty_print_json(item, indent + 1);
                    if i < arr.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                print!("{}]", indent_str);
            }
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                print!("{{}}");
            } else {
                println!("{{");
                let mut entries: Vec<_> = obj.iter().collect();
                entries.sort_by_key(|(k, _)| *k);
                
                for (i, (key, value)) in entries.iter().enumerate() {
                    print!("{}\"{}\": ", "  ".repeat(indent + 1), key);
                    pretty_print_json(value, indent + 1);
                    if i < entries.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                print!("{}}}", indent_str);
            }
        }
    }
}

fn analyze_json(value: &JsonValue, path: String) {
    match value {
        JsonValue::Null => println!("{}: null", path),
        JsonValue::Bool(b) => println!("{}: boolean ({})", path, b),
        JsonValue::Number(n) => println!("{}: number ({})", path, n),
        JsonValue::String(s) => println!("{}: string (\"{}\", length: {})", path, s, s.len()),
        JsonValue::Array(arr) => {
            println!("{}: array (length: {})", path, arr.len());
            for (i, item) in arr.iter().enumerate() {
                analyze_json(item, format!("{}[{}]", path, i));
            }
        }
        JsonValue::Object(obj) => {
            println!("{}: object ({} properties)", path, obj.len());
            for (key, value) in obj {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                analyze_json(value, new_path);
            }
        }
    }
}

fn main() {
    println!("🔍 JSON Parser and Analyzer");
    println!("===========================");
    println!("Enter JSON data (type 'END' on a new line to finish):");

    let mut json_input = String::new();
    
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");
        
        let line = line.trim();
        if line == "END" {
            break;
        }
        
        json_input.push_str(line);
        json_input.push('\n');
    }

    if json_input.trim().is_empty() {
        println!("No JSON data entered!");
        return;
    }

    let mut parser = JsonParser::new(&json_input);
    
    match parser.parse() {
        Ok(json_value) => {
            println!("\n✅ JSON parsed successfully!");
            
            println!("\n=== Pretty Printed JSON ===");
            pretty_print_json(&json_value, 0);
            println!("\n");
            
            println!("\n=== JSON Structure Analysis ===");
            analyze_json(&json_value, String::new());
            
            // Interactive query system
            println!("\n=== Interactive Query ===");
            println!("Enter JSON paths to query (e.g., 'name', 'users[0].email', 'quit' to exit):");
            
            loop {
                print!("query> ");
                io::Write::flush(&mut io::stdout()).unwrap();
                
                let mut query = String::new();
                io::stdin().read_line(&mut query).expect("Failed to read input");
                let query = query.trim();
                
                if query == "quit" {
                    break;
                }
                
                if query.is_empty() {
                    continue;
                }
                
                match query_json_path(&json_value, query) {
                    Some(value) => println!("Result: {}", value),
                    None => println!("Path not found: {}", query),
                }
            }
        }
        Err(e) => {
            println!("\n❌ JSON parsing failed: {}", e);
            println!("Please check your JSON syntax and try again.");
        }
    }
}

fn query_json_path(value: &JsonValue, path: &str) -> Option<&JsonValue> {
    if path.is_empty() {
        return Some(value);
    }

    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;

    for part in parts {
        if part.contains('[') && part.contains(']') {
            // Handle array indexing like "users[0]"
            let bracket_pos = part.find('[').unwrap();
            let key = &part[..bracket_pos];
            let index_part = &part[bracket_pos + 1..part.len() - 1];
            
            if let Ok(index) = index_part.parse::<usize>() {
                // First navigate to the key
                if !key.is_empty() {
                    if let JsonValue::Object(obj) = current {
                        current = obj.get(key)?;
                    } else {
                        return None;
                    }
                }
                
                // Then index into the array
                if let JsonValue::Array(arr) = current {
                    current = arr.get(index)?;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            // Regular object property access
            if let JsonValue::Object(obj) = current {
                current = obj.get(part)?;
            } else {
                return None;
            }
        }
    }

    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_object() {
        let json = r#"{"name": "John", "age": 30, "active": true}"#;
        let mut parser = JsonParser::new(json);
        let result = parser.parse().unwrap();
        
        if let JsonValue::Object(obj) = result {
            assert_eq!(obj.get("name"), Some(&JsonValue::String("John".to_string())));
            assert_eq!(obj.get("age"), Some(&JsonValue::Number(30.0)));
            assert_eq!(obj.get("active"), Some(&JsonValue::Bool(true)));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3, "hello", null, true]"#;
        let mut parser = JsonParser::new(json);
        let result = parser.parse().unwrap();
        
        if let JsonValue::Array(arr) = result {
            assert_eq!(arr.len(), 6);
            assert_eq!(arr[0], JsonValue::Number(1.0));
            assert_eq!(arr[3], JsonValue::String("hello".to_string()));
            assert_eq!(arr[4], JsonValue::Null);
            assert_eq!(arr[5], JsonValue::Bool(true));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_nested_structure() {
        let json = r#"{"users": [{"name": "Alice", "scores": [95, 87]}]}"#;
        let mut parser = JsonParser::new(json);
        let result = parser.parse().unwrap();
        
        let alice_score = query_json_path(&result, "users[0].scores[0]");
        assert_eq!(alice_score, Some(&JsonValue::Number(95.0)));
    }
}