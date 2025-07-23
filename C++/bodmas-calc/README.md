# BODMAS Calculator

A recursive descent parser-based calculator that correctly implements BODMAS/PEMDAS order of operations in C.

## Overview

This calculator was developed as a fix for a broken recursive expression evaluator. The original code had infinite recursion issues and didn't properly handle mathematical precedence. This implementation uses recursive descent parsing to correctly evaluate mathematical expressions following standard order of operations.

## Features

### ✨ Core Functionality
- **BODMAS/PEMDAS Compliance**: Correctly handles order of operations
- **Four Basic Operations**: Addition (+), Subtraction (-), Multiplication (*), Division (/)
- **Parentheses Support**: Nested parentheses with proper grouping
- **Decimal Numbers**: Supports floating-point arithmetic
- **Negative Numbers**: Handles unary minus operator
- **Error Handling**: Division by zero detection and syntax error reporting

### 🎯 Order of Operations
The calculator follows standard mathematical precedence:
1. **Brackets/Parentheses**: `()` - Highest priority
2. **Multiplication/Division**: `*` `/` - Left-to-right evaluation
3. **Addition/Subtraction**: `+` `-` - Left-to-right evaluation

### 🔧 Advanced Features
- Interactive calculator mode
- Built-in test suite with verification
- Comprehensive error reporting
- Support for complex nested expressions

## Usage

### Compilation
```bash
gcc -o calculator calculator.c -lm
```

### Running the Program
```bash
./calculator
```

### Example Expressions

| Expression | Result | Explanation |
|------------|--------|-------------|
| `2 + 3 * 4` | 14.0 | Multiplication before addition |
| `(2 + 3) * 4` | 20.0 | Parentheses override precedence |
| `10 - 4 / 2` | 8.0 | Division before subtraction |
| `2 + 3 * (4 - 1)` | 11.0 | Nested operations |
| `-5 + 3 * 2` | 1.0 | Unary minus and precedence |
| `2.5 * 4 + 1.5` | 11.5 | Decimal arithmetic |

## Program Modes

### 1. Interactive Mode
Enter expressions one at a time and see immediate results:
```
=== BODMAS Calculator ===
Enter a mathematical expression (supports +, -, *, /, parentheses):
Example: 2 + 3 * (4 - 1) / 2
Expression: 5 + 3 * 2
Parsing: 5 + 3 * 2
Result: 11.000000
```

### 2. Test Mode
Runs a comprehensive test suite to verify BODMAS compliance:
```
=== Running BODMAS Tests ===
✓ 2 + 3 * 4 = 14.00
✓ 2 * 3 + 4 = 10.00
✓ 10 - 4 / 2 = 8.00
✓ (2 + 3) * 4 = 20.00
...
Tests passed: 11/11
```

## Technical Implementation

### Architecture
The calculator uses **recursive descent parsing** with three main parsing functions:

```
parse_expression()     // Handles + and - (lowest precedence)
    ↓
parse_term()          // Handles * and / (higher precedence)
    ↓
parse_factor()        // Handles numbers, parentheses, unary operators
```

### Key Functions

| Function | Purpose |
|----------|---------|
| `parse_expression()` | Main parser for addition/subtraction |
| `parse_term()` | Parser for multiplication/division |
| `parse_factor()` | Parser for atomic values and parentheses |
| `parse_number()` | Extracts numeric values (including decimals) |
| `get_expression_input()` | Handles user input |
| `run_tests()` | Executes verification test suite |

### Grammar Rules
The parser implements this context-free grammar:
```
Expression → Term (('+' | '-') Term)*
Term       → Factor (('*' | '/') Factor)*
Factor     → Number | '(' Expression ')' | ('-' | '+') Factor
Number     → Digit+ ('.' Digit+)?
```

## Error Handling

The calculator includes robust error handling for common issues:

- **Division by Zero**: `Error: Division by zero`
- **Mismatched Parentheses**: `Error: Missing closing parenthesis`
- **Invalid Characters**: `Error: Unexpected character 'x' at position 5`
- **Incomplete Parsing**: `Warning: Unexpected characters after expression`

## Comparison with Original Code

### Original Issues Fixed

| Problem | Original Code | Fixed Implementation |
|---------|---------------|---------------------|
| Infinite Recursion | `s_exp(sub_exp, op)` calls itself endlessly | Proper base cases and parameter progression |
| No BODMAS | All operations had equal precedence | Three-level parsing hierarchy |
| Limited Operators | Only + and - | Full arithmetic: +, -, *, / |
| Poor Input Handling | `scanf_s` issues | Robust string parsing |
| No Parentheses | Not supported | Full nested parentheses support |
| Stack Overflow | Guaranteed crash | Safe recursive parsing |

### Performance
- **Time Complexity**: O(n) where n is expression length
- **Space Complexity**: O(d) where d is maximum nesting depth
- **Memory Safe**: No buffer overflows, proper bounds checking

## Testing

The program includes a comprehensive test suite covering:
- Basic arithmetic operations
- Operator precedence verification
- Parentheses handling
- Decimal number support
- Negative number handling
- Complex nested expressions

Run tests with option `t` or `2` when starting the program.

## Limitations

- **Integer Overflow**: Very large numbers may cause precision issues
- **Expression Length**: Limited to 1000 characters
- **No Advanced Functions**: No support for sin, cos, log, etc.
- **No Variables**: Only numeric literals supported

## Future Enhancements

Potential improvements for future versions:
- Scientific functions (trigonometry, logarithms)
- Variable support
- Memory functions (store/recall)
- Expression history
- Graphing capabilities
- Multi-line expression support

## Contributing

When contributing to this project:
1. Maintain BODMAS compliance
2. Add test cases for new features
3. Follow existing code style
4. Update documentation
5. Ensure memory safety

## License

This project is provided as-is for educational purposes. Feel free to use, modify, and distribute according to your needs.

---

**Note**: This calculator demonstrates practical application of recursive descent parsing, compiler theory concepts, and proper C programming practices. It's an excellent example of how mathematical precedence can be naturally implemented through parser structure.