import math  # To use square root and other mathematical functions
from decimal import Decimal, getcontext

# Set precision for Decimal calculations
getcontext().prec = 28  # You can adjust this value to set the level of precision

def describe_operator(operator):
    # Function to describe the operator
    descriptions = {
        '+': "Adding the next number(s).",
        '-': "Subtracting the next number(s).",
        '*': "Multiplying by the next number(s).",
        '/': "Dividing by the next number(s).",
        '^': "Raising to the power of the next number(s).",
        '%': "Finding the remainder (modulus) of division.",
        'sqrt': "Calculating the square root.",
        'C': "Resetting the calculator."
    }
    return descriptions.get(operator, "Unknown operation.")

def evaluate_expression():
    total = Decimal(input("Please input the first number: "))  # Get the first number, using Decimal
    while True:
        operator = input("Please input an operator (+, -, *, /, ^, %, sqrt, C, = to finish): ")  # Get operator
        print(describe_operator(operator))  # Describe the operator

        if operator == '=':  # Stop if the operator is '='
            break
        elif operator == 'C':  # Reset the calculator
            total = Decimal(input("Calculator reset. Please input the new first number: "))
            continue
        elif operator == 'sqrt':  # Square root function
            if total < 0:
                print("Error: Cannot compute square root of a negative number.")
            else:
                total = Decimal(math.sqrt(total))  # Compute square root with Decimal precision
            print(f"Intermediate result after square root: {total}")
            continue

        # Input multiple numbers separated by spaces
        numbers = input("Please input the next number(s), separated by spaces: ")
        numbers = [Decimal(num) for num in numbers.split()]

        # Process each number with the current operator
        for num in numbers:
            if operator == '+':
                total += num  # Add if operator is '+'
            elif operator == '-':
                total -= num  # Subtract if operator is '-'
            elif operator == '*':
                total *= num  # Multiply if operator is '*'
            elif operator == '/':
                if num == 0:  # Check for division by zero
                    print("Error: Division by zero is not allowed.")
                else:
                    total /= num  # Divide if operator is '/'
            elif operator == '^':
                total = total ** num  # Raise to power if operator is '^'
            elif operator == '%':
                total %= num  # Compute modulus if operator is '%'

    print(f"Final result: {total}")  # Output the result

# Run the function
evaluate_expression()
