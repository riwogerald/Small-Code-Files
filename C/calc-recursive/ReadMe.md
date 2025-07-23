# Simple Recursive Descent Calculator

A basic calculator implementation in C that uses recursive descent parsing to evaluate mathematical expressions containing addition and multiplication operators.

## Features

- Supports integer arithmetic with `+` (addition) and `*` (multiplication) operations
- Implements proper operator precedence (multiplication before addition)
- Interactive command-line interface
- Built using recursive descent parsing technique
- Comprehensive error handling

## How It Works

### Architecture Overview

The calculator is built using three main components:

1. **Lexical Analyzer (Tokenizer)** - Breaks input into tokens
2. **Recursive Descent Parser** - Parses tokens according to grammar rules
3. **Evaluator** - Computes results during parsing

### Grammar Rules

The calculator implements this context-free grammar:

```
expression → term (('+') term)*
term       → factor (('*') factor)*  
factor     → number
```

This grammar ensures proper operator precedence where multiplication (`*`) has higher precedence than addition (`+`).

### Token Types

The lexical analyzer recognizes these token types:

- `TOKEN_NUMBER` - Integer values
- `TOKEN_PLUS` - Addition operator (`+`)
- `TOKEN_MULTIPLY` - Multiplication operator (`*`)
- `TOKEN_EOF` - End of input
- `TOKEN_ERROR` - Invalid characters

### Parsing Process

#### 1. Lexical Analysis (`next_token()`)
- Skips whitespace characters
- Identifies numbers by reading consecutive digits
- Recognizes operators (`+`, `*`)
- Handles end-of-input and error conditions

#### 2. Recursive Descent Parsing
The parser uses three mutually recursive functions:

**`expression()`** - Handles addition
- Calls `term()` to get the first operand
- While there are `+` operators, consumes them and adds the next term
- Example: `2+3+4` → evaluates left-to-right as `((2+3)+4)`

**`term()`** - Handles multiplication  
- Calls `factor()` to get the first operand
- While there are `*` operators, consumes them and multiplies by the next factor
- Example: `2*3*4` → evaluates left-to-right as `((2*3)*4)`

**`factor()`** - Handles numbers
- Expects a number token
- Returns the numeric value
- Raises error if number is not found

### Operator Precedence

The recursive structure automatically handles precedence:

- `2+3*4` is parsed as `2+(3*4) = 14` (not `(2+3)*4 = 20`)
- `2*3+4` is parsed as `(2*3)+4 = 10`

This works because `expression()` calls `term()`, ensuring multiplication is evaluated before addition.

## Usage

### Compilation

```bash
gcc -o calculator calc-recursive.c
```

### Running

```bash
./calculator
```

### Example Session

```
Simple Calculator (supports + and * operations)
Enter expressions with integers, '+', and '*' operators
Examples: 5, 3+4, 2*3+4, 2+3*4
Enter 'quit' to exit

calc> 5
Result: 5

calc> 3+4
Result: 7

calc> 2*3+4
Result: 10

calc> 2+3*4
Result: 14

calc> quit
Goodbye!
```

## Code Structure

### Key Functions

- `main()` - Interactive loop and input handling
- `parse_expression()` - Initializes parsing and returns final result
- `next_token()` - Lexical analyzer
- `expression()`, `term()`, `factor()` - Recursive descent parser functions
- `error()` - Error handling and program termination

### Global Variables

- `input` - Pointer to the input string being parsed
- `position` - Current position in the input string
- `current_token` - Currently processed token

## Limitations

- Only supports integers (no floating-point numbers)
- Limited to addition (`+`) and multiplication (`*`) operators
- No support for parentheses
- No support for subtraction or division
- Basic error handling (terminates on first error)

## Educational Value

This implementation demonstrates:

- **Recursive Descent Parsing** - A top-down parsing technique
- **Context-Free Grammars** - How to translate grammar rules into code
- **Operator Precedence** - Implementing mathematical precedence rules
- **Lexical Analysis** - Breaking input into meaningful tokens
- **Separation of Concerns** - Clean separation between lexing, parsing, and evaluation

## Potential Extensions

- Add parentheses support: `factor → number | '(' expression ')'`
- Include subtraction and division operators
- Support floating-point numbers
- Add better error recovery instead of immediate termination
- Implement unary operators (negative numbers)
- Add memory/variable storage functionality