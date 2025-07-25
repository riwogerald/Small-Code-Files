# Advanced Password Generator

A comprehensive C++ console application for generating secure passwords with customizable options, strength analysis, and entropy calculations.

## Overview

This program implements a sophisticated password generation system using modern C++ features including random number generation, STL containers, and object-oriented design. It provides multiple password generation modes, strength analysis, and security-focused configuration options.

## Features

- **Secure Random Generation**: Uses `std::random_device` and Mersenne Twister for cryptographically secure randomness
- **Customizable Character Sets**: Support for lowercase, uppercase, digits, and symbols
- **Strength Analysis**: Real-time password strength calculation using entropy-based algorithms
- **Multiple Generation Modes**: Single password, bulk generation, and custom configuration
- **Ambiguous Character Filtering**: Option to exclude confusing characters (0, O, 1, l, I)
- **Visual Strength Indicators**: Graphical strength bars and percentage scores
- **Menu-Driven Interface**: User-friendly console interface with multiple options

## Password Strength Calculation

The program calculates password strength using entropy formula:
```
Entropy = Length × log₂(Character Set Size)
Strength = min(100, (Entropy / 128) × 100)
```

**Strength Categories:**
- Very Weak: < 25%
- Weak: 25-49%
- Good: 50-74%
- Strong: 75-89%
- Very Strong: ≥ 90%

## Technical Specifications

- **Language**: C++11 or later
- **Random Engine**: Mersenne Twister (std::mt19937)
- **Character Sets**: 
  - Lowercase: 26 characters
  - Uppercase: 26 characters
  - Digits: 10 characters
  - Symbols: 26 special characters
- **Password Length**: 4-128 characters
- **Entropy Range**: Up to 128 bits

## Prerequisites

- C++ compiler with C++11 support or later
- Standard libraries: `<iostream>`, `<string>`, `<vector>`, `<random>`, `<algorithm>`, `<cctype>`, `<iomanip>`

## Compilation

```bash
g++ -std=c++11 password_generator.cpp -o password_generator
```

For enhanced security and optimization:
```bash
g++ -std=c++11 -O2 -Wall -Wextra password_generator.cpp -o password_generator
```

## Usage

1. Compile and run the program:
   ```bash
   ./password_generator
   ```

2. Choose from the main menu options:
   - **Option 1**: Generate single password with default settings (12 characters, mixed case + digits)
   - **Option 2**: Generate multiple passwords at once
   - **Option 3**: Analyze existing password strength
   - **Option 4**: Custom password configuration
   - **Option 5**: Exit the program

## Sample Output

### Password Generation
```
Generated Password: Kp8mN2vQx9Wr
Length: 12 characters
Strength: 71.2% (Good)
Strength Bar: [==============      ]
```

### Multiple Password Generation
```
Generated 5 passwords:
--------------------------------
1. Mx9pQ2kL8vNr
2. Bp7nK5wZ3xCm
3. Rq6jH4yV2sGf
4. Nt8lP1uA6hEw
5. Zv3gM9kX7qBn
```

### Custom Configuration Options
- Password length (4-128 characters)
- Include/exclude character types
- Minimum number requirements
- Ambiguous character filtering
- No repeating characters option

## Class Structure

### PasswordGenerator Class
```cpp
class PasswordGenerator {
private:
    std::random_device rd;           // Hardware random seed
    std::mt19937 gen;               // Mersenne Twister generator
    
public:
    struct PasswordConfig { ... };   // Configuration structure
    
    std::string generatePassword(const PasswordConfig& config);
    std::vector<std::string> generateMultiple(const PasswordConfig& config, int count);
    double calculateStrength(const std::string& password);
    std::string getStrengthLabel(double strength);
};
```

## Key Methods

- **`generatePassword()`**: Creates a single password based on configuration
- **`generateMultiple()`**: Generates multiple passwords efficiently
- **`calculateStrength()`**: Analyzes password entropy and strength
- **`buildCharset()`**: Constructs character set based on options
- **`ensureMinimumRequirements()`**: Guarantees minimum character type requirements

## Security Features

### Cryptographically Secure Random Generation
- Uses hardware random number generator when available
- Mersenne Twister provides high-quality pseudorandom sequences
- Proper seeding prevents predictable patterns

### Character Set Management
- Separate character pools for different types
- Optional ambiguous character exclusion
- Configurable minimum requirements per character type

### Entropy-Based Strength Analysis
- Mathematical approach to password strength
- Considers both length and character set diversity
- Industry-standard entropy calculations

## Educational Value

This program demonstrates:

### Advanced C++ Concepts
- Modern random number generation
- STL containers and algorithms
- Exception handling
- Structure and class design
- Template usage with containers

### Cryptographic Principles
- Entropy and randomness concepts
- Password security best practices
- Character set diversity importance
- Strength measurement techniques

### Software Engineering
- Object-oriented design patterns
- User interface design
- Configuration management
- Error handling strategies

## Possible Extensions

### Enhanced Security Features
- Dictionary word checking
- Pattern detection algorithms
- Keyboard walk detection
- Repeated character analysis

### Additional Functionality
- Password policy compliance checking
- Export to file formats
- Password history tracking
- Batch processing capabilities

### User Interface Improvements
- Colorized output
- Progress bars for bulk generation
- Configuration file support
- Command-line argument parsing

## Learning Objectives

Perfect for understanding:
- Modern C++ random number generation
- Entropy and cryptographic concepts
- Object-oriented programming design
- STL container manipulation
- Menu-driven application architecture
- Security-focused software development

## Compilation Notes

**For Windows (Visual Studio):**
```cmd
cl /EHsc /std:c++11 password_generator.cpp
```

**For macOS:**
```bash
clang++ -std=c++11 -stdlib=libc++ password_generator.cpp -o password_generator
```

This tool provides a solid foundation for understanding both password security principles and advanced C++ programming techniques, making it ideal for cybersecurity education and C++ language learning.
