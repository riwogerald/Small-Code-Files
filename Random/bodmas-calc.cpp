#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_EXPR_LEN 1000

// Global variables for expression parsing
char expression[MAX_EXPR_LEN];
int pos = 0;

// Function prototypes
float parse_expression();
float parse_term();
float parse_factor();
float parse_number();
void skip_whitespace();
char peek_char();
char get_char();
void get_expression_input();

// Get the entire expression as input
void get_expression_input() {
    printf("Enter a mathematical expression (supports +, -, *, /, parentheses):\n");
    printf("Example: 2 + 3 * (4 - 1) / 2\n");
    printf("Expression: ");
    
    if (fgets(expression, MAX_EXPR_LEN, stdin) != NULL) {
        // Remove newline if present
        int len = strlen(expression);
        if (len > 0 && expression[len-1] == '\n') {
            expression[len-1] = '\0';
        }
    }
    pos = 0; // Reset position for parsing
}

// Skip whitespace characters
void skip_whitespace() {
    while (pos < strlen(expression) && isspace(expression[pos])) {
        pos++;
    }
}

// Peek at current character without advancing
char peek_char() {
    skip_whitespace();
    if (pos >= strlen(expression)) {
        return '\0';
    }
    return expression[pos];
}

// Get current character and advance position
char get_char() {
    skip_whitespace();
    if (pos >= strlen(expression)) {
        return '\0';
    }
    return expression[pos++];
}

// Parse a number (including decimals)
float parse_number() {
    float result = 0;
    float decimal_place = 0;
    int has_decimal = 0;
    
    skip_whitespace();
    
    // Handle negative numbers
    int negative = 0;
    if (peek_char() == '-') {
        negative = 1;
        get_char(); // consume the '-'
    } else if (peek_char() == '+') {
        get_char(); // consume the '+'
    }
    
    // Parse integer part
    while (pos < strlen(expression) && isdigit(expression[pos])) {
        result = result * 10 + (expression[pos] - '0');
        pos++;
    }
    
    // Parse decimal part if present
    if (pos < strlen(expression) && expression[pos] == '.') {
        pos++; // skip decimal point
        decimal_place = 0.1;
        has_decimal = 1;
        
        while (pos < strlen(expression) && isdigit(expression[pos])) {
            result += (expression[pos] - '0') * decimal_place;
            decimal_place *= 0.1;
            pos++;
        }
    }
    
    return negative ? -result : result;
}

// Parse factors: numbers, parentheses, unary operators
// Handles: numbers, (expression), -factor, +factor
float parse_factor() {
    skip_whitespace();
    char ch = peek_char();
    
    if (ch == '(') {
        get_char(); // consume '('
        float result = parse_expression();
        
        if (peek_char() == ')') {
            get_char(); // consume ')'
        } else {
            printf("Error: Missing closing parenthesis\n");
        }
        return result;
    } else if (ch == '-') {
        get_char(); // consume '-'
        return -parse_factor();
    } else if (ch == '+') {
        get_char(); // consume '+'
        return parse_factor();
    } else if (isdigit(ch) || ch == '.') {
        return parse_number();
    } else {
        printf("Error: Unexpected character '%c' at position %d\n", ch, pos);
        return 0;
    }
}

// Parse terms: handles * and / (higher precedence)
// Recursive implementation for left-to-right evaluation
float parse_term() {
    float result = parse_factor();
    
    while (1) {
        char op = peek_char();
        if (op == '*') {
            get_char(); // consume '*'
            result *= parse_factor();
        } else if (op == '/') {
            get_char(); // consume '/'
            float divisor = parse_factor();
            if (divisor == 0) {
                printf("Error: Division by zero\n");
                return result;
            }
            result /= divisor;
        } else {
            break;
        }
    }
    
    return result;
}

// Parse expressions: handles + and - (lower precedence)
// This is the main recursive function
float parse_expression() {
    float result = parse_term();
    
    while (1) {
        char op = peek_char();
        if (op == '+') {
            get_char(); // consume '+'
            result += parse_term();
        } else if (op == '-') {
            get_char(); // consume '-'
            result -= parse_term();
        } else {
            break;
        }
    }
    
    return result;
}

// Interactive calculator mode
void interactive_mode() {
    char continue_calc;
    
    do {
        printf("\n=== BODMAS Calculator ===\n");
        get_expression_input();
        
        if (strlen(expression) == 0) {
            printf("Empty expression entered.\n");
            continue;
        }
        
        printf("Parsing: %s\n", expression);
        
        pos = 0; // Reset parser position
        float result = parse_expression();
        
        // Check if we parsed the entire expression
        skip_whitespace();
        if (pos < strlen(expression)) {
            printf("Warning: Unexpected characters after expression: '%s'\n", 
                   &expression[pos]);
        }
        
        printf("Result: %.6f\n", result);
        
        printf("\nDo another calculation? (y/n): ");
        scanf(" %c", &continue_calc);
        
        // Clear input buffer
        int c;
        while ((c = getchar()) != '\n' && c != EOF);
        
    } while (continue_calc == 'y' || continue_calc == 'Y');
}

// Test function with predefined expressions
void run_tests() {
    printf("\n=== Running BODMAS Tests ===\n");
    
    // Test cases with expected results
    struct {
        char* expr;
        float expected;
    } tests[] = {
        {"2 + 3 * 4", 14.0},
        {"2 * 3 + 4", 10.0},
        {"10 - 4 / 2", 8.0},
        {"(2 + 3) * 4", 20.0},
        {"2 + 3 * (4 - 1)", 11.0},
        {"10 / 2 - 3", 2.0},
        {"2.5 * 4 + 1.5", 11.5},
        {"-5 + 3 * 2", 1.0},
        {"(-5 + 3) * 2", -4.0},
        {"2 * 3 * 4 + 1", 25.0},
        {"20 / 4 / 2", 2.5}
    };
    
    int num_tests = sizeof(tests) / sizeof(tests[0]);
    int passed = 0;
    
    for (int i = 0; i < num_tests; i++) {
        strcpy(expression, tests[i].expr);
        pos = 0;
        
        float result = parse_expression();
        float diff = result - tests[i].expected;
        if (diff < 0) diff = -diff; // absolute difference
        
        if (diff < 0.0001) { // floating point comparison
            printf("✓ %s = %.2f\n", tests[i].expr, result);
            passed++;
        } else {
            printf("✗ %s = %.2f (expected %.2f)\n", 
                   tests[i].expr, result, tests[i].expected);
        }
    }
    
    printf("\nTests passed: %d/%d\n", passed, num_tests);
}

int main(void) {
    printf("BODMAS/PEMDAS Calculator with Recursive Parsing\n");
    printf("Supports: +, -, *, /, parentheses, decimal numbers\n");
    printf("Order of operations: Parentheses → Multiplication/Division → Addition/Subtraction\n\n");
    
    char choice;
    printf("Choose mode:\n");
    printf("1. Interactive calculator (i)\n");
    printf("2. Run tests (t)\n");
    printf("3. Both (b)\n");
    printf("Enter choice: ");
    scanf(" %c", &choice);
    
    // Clear input buffer
    int c;
    while ((c = getchar()) != '\n' && c != EOF);
    
    switch (choice) {
        case 'i':
        case 'I':
        case '1':
            interactive_mode();
            break;
        case 't':
        case 'T':
        case '2':
            run_tests();
            break;
        case 'b':
        case 'B':
        case '3':
            run_tests();
            interactive_mode();
            break;
        default:
            printf("Invalid choice, starting interactive mode...\n");
            interactive_mode();
            break;
    }
    
    printf("\nThank you for using the BODMAS Calculator!\n");
    return 0;
}