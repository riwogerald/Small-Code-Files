# Flex Lexical Analyzer - Lowercase to Uppercase Converter

## Overview

This is a simple lexical analyzer built using **Flex** (Fast Lexical Analyzer Generator) that converts lowercase letters to uppercase while preserving all other characters. The program demonstrates basic lexical analysis concepts and pattern matching.

## Files

- `Replace lex.l` - Source flex specification file
- `lex.yy.c` - Generated C code from flex (automatically created)

## How It Works

### The Flex Specification (`Replace lex.l`)

The flex file consists of three main sections:

```lex
%{
#include <stdio.h>
%}
%%
[a-z] {printf("%c",yytext[0]-32);}
\n    {printf("\n"); return 0;}
.     {printf("%c", yytext[0]);}
%%
int yywrap(void){
    return 1;
}

int main()
{
    printf("Enter the string: ");
    yylex();
    return 0;
}
```

#### 1. Definitions Section (`%{ ... %}`)
- Contains C code that will be copied directly to the generated file
- Includes necessary headers like `stdio.h`

#### 2. Rules Section (between `%%`)
This section defines three pattern-action pairs:

**Pattern: `[a-z]`**
- **Matches**: Any single lowercase letter from 'a' to 'z'
- **Action**: `{printf("%c",yytext[0]-32);}`
  - `yytext[0]` contains the matched character
  - Subtracting 32 from ASCII value converts lowercase to uppercase
  - Example: 'a' (ASCII 97) - 32 = 'A' (ASCII 65)

**Pattern: `\n`**
- **Matches**: Newline character
- **Action**: `{printf("\n"); return 0;}`
  - Prints a newline and terminates the program

**Pattern: `.`**
- **Matches**: Any character except newline (catch-all pattern)
- **Action**: `{printf("%c", yytext[0]);}`
  - Prints the character unchanged

#### 3. User Code Section
Contains helper functions:
- `yywrap()`: Returns 1 to indicate end of input
- `main()`: Entry point that prompts user and starts lexical analysis

### Pattern Matching Priority

Flex follows these rules for pattern matching:
1. **Longest match**: Matches the longest possible string
2. **First match**: If multiple patterns match the same length, the first one listed takes precedence

In this program:
- `[a-z]` has highest priority for lowercase letters
- `\n` specifically handles newlines
- `.` catches everything else

### ASCII Conversion Logic

The core conversion uses ASCII arithmetic:
```c
yytext[0] - 32
```

| Character | ASCII Value | After -32 | Result |
|-----------|-------------|-----------|---------|
| 'a'       | 97          | 65        | 'A'     |
| 'b'       | 98          | 66        | 'B'     |
| 'z'       | 122         | 90        | 'Z'     |

## Building and Running

### Prerequisites
- Flex lexical analyzer generator
- GCC compiler (or compatible C compiler)

### Build Steps

1. **Generate C code from flex specification:**
   ```bash
   flex Replace\ lex.l
   ```
   This creates `lex.yy.c`

2. **Compile the generated C code:**
   ```bash
   gcc lex.yy.c -o converter -lfl
   ```
   The `-lfl` flag links the flex library

3. **Run the program:**
   ```bash
   ./converter
   ```

### Example Usage

```
Enter the string: Hello World!
HELLO WORLD!
```

**Input**: `Hello World!`
**Output**: `HELLO WORLD!`

- 'H' → 'H' (already uppercase, matched by `.` pattern)
- 'e' → 'E' (lowercase, converted by `[a-z]` pattern)
- 'l' → 'L' (lowercase, converted)
- '!' → '!' (special character, unchanged)

## Key Flex Concepts Demonstrated

### Global Variables
- `yytext`: Pointer to the current token (matched text)
- `yyleng`: Length of current token
- `yyin`: Input file pointer (defaults to stdin)
- `yyout`: Output file pointer (defaults to stdout)

### Functions
- `yylex()`: Main lexical analysis function (generated)
- `yywrap()`: Called at EOF, return 1 to stop, 0 to continue
- `yyrestart()`: Restart scanner with new input

### Pattern Syntax
- `[a-z]`: Character class (any lowercase letter)
- `\n`: Escaped newline character
- `.`: Dot matches any character except newline
- `%%`: Section delimiter

## Limitations

1. **Single character processing**: Processes one character at a time
2. **No buffering**: Output appears immediately
3. **Simple termination**: Stops at first newline
4. **ASCII assumption**: Assumes ASCII character encoding

## Educational Value

This example demonstrates:
- Basic lexical analysis concepts
- Pattern matching with regular expressions
- Character encoding and ASCII manipulation
- Flex tool usage and C integration
- State machine-based text processing

## Possible Extensions

- Handle multiple lines of input
- Add support for Unicode characters
- Implement word-by-word processing
- Add command-line options for different conversions
- Create bidirectional conversion (upper to lower)

## Troubleshooting

**Common issues:**
- **Missing flex**: Install flex package (`sudo apt-get install flex` on Ubuntu)
- **Linking errors**: Ensure `-lfl` flag is used during compilation
- **Permission errors**: Make sure the executable has run permissions (`chmod +x converter`)

---

*This lexical analyzer serves as a foundation for understanding more complex language processing tools and compiler construction techniques.*