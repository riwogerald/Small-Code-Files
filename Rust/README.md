# Rust Files

## factorial.rs
**Purpose**: Calculates the factorial of a number using both recursive and iterative approaches.
- Implements memory-safe recursive factorial with overflow protection
- Provides iterative alternative for better performance
- Includes input validation and error handling for large numbers
- Demonstrates Rust's pattern matching with `match` expressions

## fibonacci.rs
**Purpose**: Generates Fibonacci sequences using multiple efficient implementations.
- Vector-based approach for storing complete sequences
- Memory-efficient iterative generation without vector indexing
- Optimized nth Fibonacci number calculator
- Showcases Rust's ownership system and vector capacity management

## palindrome.rs
**Purpose**: Checks if strings are palindromes with comprehensive text processing.
- Multiple implementation approaches (simple, optimized, iterator-based)
- Handles case-insensitive comparison and non-alphanumeric filtering
- Uses Rust's powerful iterator chains and functional programming
- Demonstrates string manipulation and Unicode handling

## prime_number.rs
**Purpose**: Determines if numbers are prime using optimized algorithms.
- Implements 6k±1 optimization for efficient primality testing
- Includes Sieve of Eratosthenes for generating prime ranges
- Shows prime factorization for composite numbers
- Demonstrates performance testing with `std::time::Instant`

## sort_list.rs
**Purpose**: Implements multiple sorting algorithms with performance comparison.
- Bubble sort with early termination optimization
- Quick sort with in-place partitioning
- Merge sort with efficient memory management
- Insertion sort for small arrays
- Compares custom implementations against Rust's built-in sorting

## gcd_lcm.rs
**Purpose**: Calculates Greatest Common Divisor and Least Common Multiple.
- Euclidean algorithm in both recursive and iterative forms
- Extended Euclidean algorithm with Bézout coefficients
- Fraction simplification and coprime number detection
- Mathematical verification of GCD-LCM relationship
- Demonstrates Rust's integer arithmetic and type safety
