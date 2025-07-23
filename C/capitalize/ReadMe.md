# Capitalize - Lex/Flex Case Conversion Tool

A simple lexical analyzer built with Lex/Flex that converts the case of alphabetic characters in text input. Lowercase letters are converted to uppercase, and uppercase letters are converted to lowercase.

## Table of Contents
- [How It Works](#how-it-works)
- [File Structure](#file-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Code Explanation](#code-explanation)
- [Example](#example)
- [Troubleshooting](#troubleshooting)

## How It Works

This program demonstrates the basics of lexical analysis using Flex (Fast Lexical Analyzer). It reads text input and applies pattern matching rules to:
- Convert lowercase letters (a-z) to uppercase (A-Z)
- Convert uppercase letters (A-Z) to lowercase (a-z)
- Ignore whitespace (tabs and spaces)
- Pass through all other characters unchanged

## File Structure

```
project/
├── capitalize.l      # Flex source file with lexical rules
├── lex.yy.c         # Generated C code (created by Flex)
├── capitalize       # Compiled executable
└── README.md        # This documentation
```

## Prerequisites

- **Flex** (Fast Lexical Analyzer Generator)
- **GCC** (GNU Compiler Collection) or any C compiler
- **Linux/Unix environment** (or Windows with appropriate tools)

### Installing Prerequisites

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install flex gcc
```

**CentOS/RHEL:**
```bash
sudo yum install flex gcc
```

**macOS:**
```bash
brew install flex gcc
```

## Installation

1. **Clone or download the source code**
2. **Generate the lexer from the Flex file:**
   ```bash
   flex capitalize.l
   ```
   This creates `lex.yy.c`

3. **Compile the generated C code:**
   ```bash
   gcc lex.yy.c -o capitalize
   ```

## Usage

### Interactive Mode
Run the program and type text directly:
```bash
./capitalize
```
Type your text and press Enter. Use Ctrl+D (Linux/Mac) or Ctrl+Z (Windows) to exit.

### File Input
Process a text file:
```bash
./capitalize < input.txt
```

### Pipe Input
Use with other commands:
```bash
echo "Hello World!" | ./capitalize
```

## Code Explanation

### Flex File Structure (`capitalize.l`)

#### 1. Definition Section
```c
%{
#include<stdio.h>
#include<string.h>
%}
```
- Contains C code that will be copied to the generated file
- Includes necessary header files

#### 2. Rules Section
```c
%%
[\t ]+           /* ignore whitespace */ ;
[a-z]           { printf("%c",yytext[0]-32); }
[A-Z]           { printf("%c",yytext[0]+32); }
.|\n            { ECHO; }
%%
```

**Rule Breakdown:**
- `[\t ]+` - Matches one or more tabs or spaces, ignores them
- `[a-z]` - Matches lowercase letters, converts to uppercase by subtracting 32
- `[A-Z]` - Matches uppercase letters, converts to lowercase by adding 32
- `.|\n` - Matches any other character or newline, prints unchanged

**ASCII Conversion Logic:**
- ASCII value of 'A' = 65, 'a' = 97
- Difference = 97 - 65 = 32
- To convert 'a' to 'A': 97 - 32 = 65
- To convert 'A' to 'a': 65 + 32 = 97

#### 3. User Code Section
```c
int yywrap(void) { return 1; }

int main() {
    yylex();
    return 0;
}
```
- `yywrap()`: Required function, returns 1 to indicate end of input
- `main()`: Entry point, calls `yylex()` to start lexical analysis

### Key Variables and Functions

- **`yytext`**: Points to the current token (matched text)
- **`yylex()`**: Main lexical analyzer function
- **`ECHO`**: Macro that prints the current token unchanged
- **Pattern matching**: Uses regular expressions to define token patterns

## Example

**Input:**
```
Hello World! This is a TEST.
Numbers: 123, Symbols: @#$%
```

**Output:**
```
hELLOwORLD!tHISISAtest.
nUMBERS:123,sYMBOLS:@#$%
```

**Explanation:**
- 'H' → 'h', 'e' → 'E', 'l' → 'L', etc.
- Spaces are ignored (removed from output)
- Numbers and symbols remain unchanged
- Newlines and other characters are preserved

## Troubleshooting

### Common Issues

1. **Flex not found:**
   ```bash
   flex: command not found
   ```
   **Solution:** Install Flex using your package manager

2. **Compilation errors:**
   ```bash
   undefined reference to yywrap
   ```
   **Solution:** Ensure `yywrap()` function is implemented and returns 1

3. **Wrong case conversion:**
   ```bash
   # Wrong output characters
   ```
   **Solution:** Check ASCII arithmetic (should be ±32, not other values)

4. **Program hangs:**
   **Solution:** The program waits for input. Type text and press Ctrl+D to exit

### Debugging Tips

- **Verbose compilation:** `gcc -v lex.yy.c -o capitalize`
- **Check generated code:** Examine `lex.yy.c` for pattern matching logic
- **Test with simple input:** Start with single characters
- **Use print statements:** Add `printf()` statements for debugging

## Learning Objectives

This example demonstrates:
- **Lexical analysis concepts**
- **Regular expression pattern matching**
- **ASCII character manipulation**
- **Flex/Lex tool usage**
- **C programming integration**

## Further Reading

- [Flex Manual](https://westes.github.io/flex/manual/)
- [Lex & Yacc Tutorial](http://dinosaur.compilertools.net/)
- [ASCII Table Reference](https://www.asciitable.com/)