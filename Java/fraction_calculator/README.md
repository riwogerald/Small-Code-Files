# Fraction Calculator

## Overview

A simple Java GUI application that calculates the sum of two fractions using Swing components. This program demonstrates basic GUI programming concepts, input validation, and mathematical operations with fractions.

## Features

- **GUI Interface**: User-friendly dialog boxes for input and output
- **Fraction Input**: Enter numerator and denominator for two fractions
- **Input Validation**: Prevents division by zero errors
- **Automatic Calculation**: Computes and displays the sum of fractions
- **Decimal Output**: Shows result as decimal values
- **Error Handling**: Prompts user to re-enter zero denominators

## Technical Specifications

### Class Structure
- **FractionCalculator**: Main class containing the application logic
- Uses `JOptionPane` for GUI components
- Implements basic fraction arithmetic

### Key Features
- Input validation for zero denominators
- Automatic retry on invalid input
- Clear user feedback and error messages
- Decimal conversion for easy understanding

## Usage Instructions

### Running the Program
1. Compile: `javac fraction_calculator.java`
2. Run: `java FractionCalculator`

### Program Flow
1. Enter numerator for first fraction
2. Enter denominator for first fraction (cannot be zero)
3. Enter numerator for second fraction  
4. Enter denominator for second fraction (cannot be zero)
5. View the calculated sum

### Sample Interaction
```
Input: First fraction numerator = 3
Input: First fraction denominator = 4
Input: Second fraction numerator = 1
Input: Second fraction denominator = 2
Output: Sum of 0.75 + 0.5 is 1.25
```

## Code Structure

### Main Method
```java
public static void main(String[] args) {
    // Get first fraction
    // Get second fraction  
    // Calculate sum
    // Display result
}
```

### Input Validation
- Uses `do-while` loop for denominator validation
- Prevents division by zero errors
- Clear error messages for user guidance

## Educational Value

This program demonstrates:
- **GUI Programming**: Basic Swing components usage
- **Input Validation**: Preventing invalid user input
- **Loop Structures**: `do-while` loops for validation
- **Type Conversion**: String to integer parsing
- **Mathematical Operations**: Basic fraction arithmetic
- **Error Handling**: User-friendly error messages

## File Structure
```
fraction_calculator/
├── fraction_calculator.java    # Main application
└── README.md                  # This documentation
```

## Key Learning Points
- JOptionPane dialog usage
- Input validation techniques
- Fraction to decimal conversion
- Basic GUI application structure
- Error handling in Java applications

## Possible Enhancements
- Display result as proper fraction
- Support for subtraction, multiplication, division
- Fraction simplification
- Multiple fraction operations
- Better GUI with panels and buttons
