#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

// Token types
typedef enum {
    TOKEN_NUMBER,
    TOKEN_PLUS,
    TOKEN_MULTIPLY,
    TOKEN_EOF,
    TOKEN_ERROR
} TokenType;

// Token structure
typedef struct {
    TokenType type;
    int value;
} Token;

// Global variables for parsing
char *input;
int position;
Token current_token;

// Function prototypes
void next_token();
int expression();
int term();
int factor();
void error(const char *message);

// Lexical analyzer - gets the next token from input
void next_token() {
    // Skip whitespace
    while (input[position] == ' ' || input[position] == '\t') {
        position++;
    }
    
    // Check for end of input
    if (input[position] == '\0' || input[position] == '\n') {
        current_token.type = TOKEN_EOF;
        return;
    }
    
    // Parse numbers
    if (isdigit(input[position])) {
        current_token.type = TOKEN_NUMBER;
        current_token.value = 0;
        
        while (isdigit(input[position])) {
            current_token.value = current_token.value * 10 + (input[position] - '0');
            position++;
        }
        return;
    }
    
    // Parse operators
    switch (input[position]) {
        case '+':
            current_token.type = TOKEN_PLUS;
            position++;
            break;
        case '*':
            current_token.type = TOKEN_MULTIPLY;
            position++;
            break;
        default:
            current_token.type = TOKEN_ERROR;
            position++;
            break;
    }
}

// Error handling
void error(const char *message) {
    printf("Error: %s\n", message);
    exit(1);
}

// Grammar: expression → term (('+') term)*
int expression() {
    int result = term();
    
    while (current_token.type == TOKEN_PLUS) {
        next_token(); // consume '+'
        result = result + term();
    }
    
    return result;
}

// Grammar: term → factor (('*') factor)*
int term() {
    int result = factor();
    
    while (current_token.type == TOKEN_MULTIPLY) {
        next_token(); // consume '*'
        result = result * factor();
    }
    
    return result;
}

// Grammar: factor → number
int factor() {
    int result;
    
    if (current_token.type == TOKEN_NUMBER) {
        result = current_token.value;
        next_token(); // consume the number
        return result;
    } else {
        error("Expected a number");
        return 0; // This won't be reached due to exit in error()
    }
}

// Parse and evaluate the expression
int parse_expression(char *expr) {
    input = expr;
    position = 0;
    
    // Get the first token
    next_token();
    
    // Parse the expression
    int result = expression();
    
    // Check if we've consumed all input
    if (current_token.type != TOKEN_EOF) {
        error("Unexpected token at end of expression");
    }
    
    return result;
}

// Main function
int main() {
    char input_buffer[256];
    
    printf("Simple Calculator (supports + and * operations)\n");
    printf("Enter expressions with integers, '+', and '*' operators\n");
    printf("Examples: 5, 3+4, 2*3+4, 2+3*4\n");
    printf("Enter 'quit' to exit\n\n");
    
    while (1) {
        printf("calc> ");
        
        // Read input
        if (fgets(input_buffer, sizeof(input_buffer), stdin) == NULL) {
            break;
        }
        
        // Remove newline if present
        input_buffer[strcspn(input_buffer, "\n")] = 0;
        
        // Check for quit command
        if (strcmp(input_buffer, "quit") == 0) {
            printf("Goodbye!\n");
            break;
        }
        
        // Skip empty input
        if (strlen(input_buffer) == 0) {
            continue;
        }
        
        // Parse and evaluate
        int result = parse_expression(input_buffer);
        printf("Result: %d\n\n", result);
    }
    
    return 0;
}