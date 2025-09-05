# Rust Files

## Core Algorithm Implementations

### factorial.rs
**Purpose**: Calculates the factorial of a number using both recursive and iterative approaches.
- **Memory Safety**: Implements overflow protection with input validation (0-20 range)
- **Dual Implementation**: Both recursive and iterative methods with performance comparison
- **Pattern Matching**: Uses Rust's `match` expressions for elegant control flow
- **Error Handling**: Comprehensive input validation with descriptive error messages
- **Verification**: Automatically verifies both methods produce identical results

### fibonacci.rs
**Purpose**: Generates Fibonacci sequences using multiple efficient implementations.
- **Vector-Based Generation**: Complete sequence storage with pre-allocated capacity
- **Memory-Efficient Iteration**: Generates sequences without storing intermediate values
- **Nth Fibonacci Calculator**: Direct calculation of specific Fibonacci numbers
- **Ownership Demonstration**: Showcases Rust's ownership system and borrowing
- **Performance Optimization**: Uses `Vec::with_capacity()` for efficient memory allocation

### palindrome.rs
**Purpose**: Checks if strings are palindromes with comprehensive text processing.
- **Multiple Approaches**: Simple, optimized, and iterator-based implementations
- **Unicode Support**: Handles international characters and case conversion
- **Functional Programming**: Demonstrates Rust's iterator chains and functional style
- **Text Processing**: Advanced filtering for alphanumeric characters only
- **Performance Comparison**: Three different algorithms with identical results verification

### prime_number.rs
**Purpose**: Determines if numbers are prime using optimized algorithms.
- **6k±1 Optimization**: Efficient primality testing algorithm for large numbers
- **Sieve of Eratosthenes**: Generates all primes up to a given limit
- **Prime Factorization**: Decomposes composite numbers into prime factors
- **Performance Timing**: Uses `std::time::Instant` for algorithm benchmarking
- **Mathematical Verification**: Validates results with multiple test cases

### sort_list.rs
**Purpose**: Implements multiple sorting algorithms with performance comparison.
- **Algorithm Variety**: Bubble, quick, merge, and insertion sort implementations
- **In-Place Sorting**: Memory-efficient sorting without additional allocation
- **Performance Benchmarking**: Compares custom implementations against Rust's built-in sorting
- **Early Termination**: Optimized bubble sort with early exit when array is sorted
- **Generic Implementation**: Works with any comparable data type

### gcd_lcm.rs
**Purpose**: Calculates Greatest Common Divisor and Least Common Multiple.
- **Euclidean Algorithm**: Both recursive and iterative implementations
- **Extended Euclidean Algorithm**: Computes Bézout coefficients for linear combinations
- **Mathematical Properties**: Demonstrates GCD-LCM relationship verification
- **Fraction Operations**: Simplifies fractions using GCD calculations
- **Coprime Detection**: Identifies relatively prime number pairs

## Advanced Applications

### calculator.rs
**Purpose**: Advanced mathematical expression evaluator with recursive descent parsing.
- **Tokenization**: Lexical analysis breaking input into mathematical tokens
- **Recursive Descent Parser**: Implements proper operator precedence (PEMDAS/BODMAS)
- **Error Handling**: Comprehensive syntax error detection and reporting
- **Interactive REPL**: Read-Eval-Print Loop for continuous calculations
- **Expression Support**: Handles parentheses, decimal numbers, and unary operators
- **Grammar Implementation**: Formal grammar rules for mathematical expressions

### word_frequency.rs
**Purpose**: Analyzes text to determine word frequency and linguistic patterns.
- **Text Processing**: Advanced string cleaning and normalization
- **HashMap Usage**: Efficient word counting using Rust's standard collections
- **Statistical Analysis**: Calculates vocabulary richness and frequency distributions
- **Interactive Queries**: Real-time word lookup and frequency checking
- **Unicode Handling**: Proper handling of international text and special characters
- **Memory Efficiency**: Processes large texts without excessive memory usage

### password_generator.rs
**Purpose**: Generates cryptographically secure passwords with customizable options.
- **Cryptographic Security**: Uses `rand` crate for secure random number generation
- **Configurable Character Sets**: Customizable inclusion of letters, numbers, symbols
- **Strength Analysis**: Entropy-based password strength calculation and visualization
- **Ambiguous Character Filtering**: Option to exclude confusing characters (0, O, 1, l, I)
- **Batch Generation**: Create multiple passwords simultaneously
- **Interactive Configuration**: User-friendly setup for password requirements

### file_organizer.rs
**Purpose**: Automatically organizes files into categorized directories based on file types.
- **File System Operations**: Safe file manipulation using `std::fs`
- **Pattern Matching**: Extensive file extension to category mapping
- **Dry Run Mode**: Preview organization without making changes
- **Error Handling**: Robust error handling for file operations
- **Statistics Generation**: Analyzes directory contents and file distributions
- **Batch Processing**: Efficiently processes large numbers of files
- **Safety Features**: Confirmation prompts before making changes

### json_parser.rs
**Purpose**: Complete JSON parser and analyzer built from scratch.
- **Recursive Descent Parsing**: Hand-written parser for JSON grammar
- **AST Generation**: Creates abstract syntax tree for JSON structures
- **Pretty Printing**: Formats JSON with proper indentation and structure
- **Path Queries**: JSONPath-like querying system for nested data access
- **Error Recovery**: Detailed error messages with position information
- **Type Safety**: Strongly typed JSON value representation
- **Interactive Analysis**: Real-time JSON structure exploration

### binary_search_tree.rs
**Purpose**: Complete binary search tree implementation with full operations.
- **Generic Implementation**: Works with any ordered data type using traits
- **Tree Operations**: Insert, delete, search with optimal time complexity
- **Tree Traversals**: Inorder, preorder, and postorder traversal implementations
- **Tree Properties**: Height calculation, size tracking, min/max finding
- **Memory Management**: Safe memory handling using `Box<T>` smart pointers
- **Deletion Handling**: Complex node deletion with successor replacement
- **Visualization**: ASCII tree structure display for debugging

### matrix_operations.rs
**Purpose**: Comprehensive matrix mathematics library with linear algebra operations.
- **Matrix Arithmetic**: Addition, subtraction, multiplication with dimension validation
- **Advanced Operations**: Transpose, determinant calculation, trace computation
- **Linear Algebra**: Identity matrix generation, symmetry checking
- **Operator Overloading**: Natural mathematical syntax using Rust traits
- **Error Handling**: Comprehensive validation for matrix operations
- **Memory Efficiency**: Optimized memory layout and operations
- **Mathematical Accuracy**: Proper handling of floating-point precision

### hash_table.rs
**Purpose**: Custom hash table implementation with collision resolution.
- **Open Addressing**: Linear probing for collision resolution
- **Dynamic Resizing**: Automatic capacity expansion based on load factor
- **Deletion Handling**: Lazy deletion with tombstone marking
- **Generic Implementation**: Type-safe storage for any hashable key-value pairs
- **Performance Monitoring**: Load factor tracking and resize notifications
- **Internal Visualization**: Debug view of hash table internal structure
- **Collision Analysis**: Demonstrates hash collision handling strategies

### text_adventure.rs
**Purpose**: Interactive text-based adventure game with object-oriented design.
- **Game State Management**: Complex state tracking for rooms, items, and player
- **Command Parsing**: Natural language command interpretation
- **Inventory System**: Item collection, usage, and management
- **Room Navigation**: Connected room system with directional movement
- **Game Logic**: Win/lose conditions, scoring system, health management
- **Interactive Storytelling**: Branching narrative with player choices
- **Modular Design**: Extensible architecture for adding new content

### network_scanner.rs
**Purpose**: Network port scanner for security analysis and network discovery.
- **Concurrent Scanning**: Multi-threaded port scanning for performance
- **Service Detection**: Identifies common services running on open ports
- **Network Range Scanning**: Scans entire IP address ranges efficiently
- **Timeout Management**: Configurable connection timeouts for reliability
- **Result Analysis**: Comprehensive reporting with response times and statistics
- **Safety Features**: Educational warnings about responsible usage
- **Performance Metrics**: Detailed timing and throughput analysis

## Key Rust Features Demonstrated

### Memory Safety
- **Ownership System**: Demonstrates Rust's unique ownership model
- **Borrowing**: Safe references without memory leaks or dangling pointers
- **Smart Pointers**: `Box<T>`, `Arc<T>`, and `Mutex<T>` for safe memory management
- **Zero-Cost Abstractions**: High-level code with no runtime overhead

### Concurrency
- **Thread Safety**: Safe concurrent programming with compile-time guarantees
- **Message Passing**: Communication between threads using channels
- **Shared State**: Mutex and Arc for safe shared memory access
- **Parallel Processing**: Multi-threaded algorithms for performance

### Type System
- **Generic Programming**: Type-safe generic functions and data structures
- **Trait System**: Powerful interface system for code reuse
- **Pattern Matching**: Exhaustive pattern matching with `match` expressions
- **Error Handling**: `Result<T, E>` and `Option<T>` for safe error management

### Functional Programming
- **Iterator Chains**: Powerful iterator combinators for data processing
- **Closures**: Anonymous functions with environment capture
- **Higher-Order Functions**: Functions that operate on other functions
- **Immutability**: Preference for immutable data structures

### Performance
- **Zero-Cost Abstractions**: High-level features with no runtime cost
- **Compile-Time Optimization**: Aggressive compiler optimizations
- **Memory Efficiency**: Minimal memory overhead and optimal layouts
- **Benchmarking**: Built-in tools for performance measurement

## Educational Value

These Rust implementations demonstrate:
- **Systems Programming**: Low-level control with high-level safety
- **Modern Language Features**: Cutting-edge programming language design
- **Performance Engineering**: Writing fast code without sacrificing safety
- **Concurrent Programming**: Safe parallelism and thread management
- **Mathematical Computing**: Numerical algorithms with precision guarantees
- **Text Processing**: Advanced string manipulation and parsing
- **Data Structures**: Efficient implementations of fundamental structures
- **Network Programming**: Safe network operations and protocol handling
- **Game Development**: Interactive applications with state management
- **Security Tools**: Responsible security analysis and testing tools

## Compilation and Execution

### Prerequisites
- Rust toolchain (rustc + cargo) version 1.70 or later
- For some programs: external crates (`rand` for password generator)

### Basic Compilation
```bash
# Single file compilation
rustc filename.rs

# Run the executable
./filename

# Or compile and run in one step
rust-script filename.rs
```

### With External Dependencies
For programs requiring external crates, create a `Cargo.toml`:
```toml
[package]
name = "program_name"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

Then use:
```bash
cargo run
```

### Performance Optimization
```bash
# Release build with optimizations
rustc -O filename.rs

# Or with Cargo
cargo build --release
```

## Learning Path

1. **Start with Core Algorithms**: `factorial.rs`, `fibonacci.rs`, `palindrome.rs`
2. **Explore Data Structures**: `binary_search_tree.rs`, `hash_table.rs`, `matrix_operations.rs`
3. **Advanced Applications**: `calculator.rs`, `json_parser.rs`, `word_frequency.rs`
4. **Practical Tools**: `password_generator.rs`, `file_organizer.rs`, `network_scanner.rs`
5. **Interactive Programs**: `text_adventure.rs`

Each program builds upon concepts from previous ones, creating a comprehensive learning experience in Rust programming, from basic syntax to advanced systems programming concepts.