# Even Number Average Calculator

A simple C++ program that calculates the average of all even numbers within a specified range (3 to 111, inclusive).

## Overview

This program iterates through numbers from 3 to 111, identifies even numbers, and computes their sum, count, and average. The results are displayed with proper formatting for easy readability.

## How It Works

### Core Algorithm

1. **Initialization**: The program starts by initializing two variables:
   - `sum`: Accumulates the total of all even numbers found
   - `count`: Keeps track of how many even numbers are discovered

2. **Range Iteration**: A `for` loop iterates through each integer from 3 to 111 (inclusive)

3. **Even Number Detection**: For each number `i`, the program uses the modulo operator (`%`) to check if `i % 2 == 0`. This condition is true only for even numbers.

4. **Accumulation**: When an even number is found:
   - It's added to the running `sum`
   - The `count` is incremented by 1

5. **Average Calculation**: After processing all numbers, the average is computed using:
   ```cpp
   double average = static_cast<double>(sum) / count;
   ```
   The `static_cast<double>` ensures floating-point division for precise results.

### Output Format

The program displays:
- The range being analyzed (3 to 111)
- Total count of even numbers found
- Sum of all even numbers
- Average formatted to 2 decimal places

## Expected Results

When run, the program will find:
- **54 even numbers** in the range (4, 6, 8, 10, ..., 108, 110)
- **Sum**: 3,078
- **Average**: 57.00

Note: Although the range starts at 3, the first even number encountered is 4, and the last is 110.

## Compilation and Execution

### Prerequisites
- C++ compiler (g++, clang++, or Visual Studio)
- C++11 or later standard support

### Compile and Run
```bash
# Using g++
g++ -o evenAverage evenAverage.cpp

# Run the program
./evenAverage
```

### Alternative compilation
```bash
# With specific C++ standard
g++ -std=c++11 -o evenAverage evenAverage.cpp

# With optimization
g++ -O2 -o evenAverage evenAverage.cpp
```

## Sample Output

```
Computing average of even numbers from 3 to 111...

--- Results ---
Range: 3 to 111 (inclusive)
Even numbers found: 54
Sum of even numbers: 3078
Average: 57.00
```

## Key Features

- **Robust Logic**: Uses modulo arithmetic for reliable even number detection
- **Proper Type Casting**: Ensures accurate floating-point division
- **Formatted Output**: Results displayed with consistent precision and clear labeling
- **Efficient Algorithm**: Single-pass iteration with O(n) time complexity

## Technical Details

- **Headers Used**: 
  - `<iostream>`: For input/output operations
  - `<iomanip>`: For output formatting (`fixed`, `setprecision`)
- **Namespace**: Uses `std` namespace for convenience
- **Memory Usage**: Minimal - only uses a few integer and one double variable
- **Time Complexity**: O(n) where n is the range size (109 iterations)