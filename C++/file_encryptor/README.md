# Simple File Encryptor

A basic C++ tool to encrypt and decrypt files using the XOR cipher method.

## Overview

This program demonstrates a simplistic approach to file encryption and decryption using XOR (exclusive or) logic, a fundamental concept in cryptography, offering basic protection for files.

## Features
- **Encrypt & Decrypt**: Supports both encryption and decryption of files
- **Simplicity**: Simple XOR cipher for educational purposes
- **Low-Complexity**: Lightweight tool with minimal dependencies

## Technical Details
- **Cipher Method**: XOR encryption with a single character key
- **Key Used**: The letter 'K' as the XOR key
- **Logic**: Encrypts and decrypts by reading each byte, performing XOR with the key, and writing the result

## Prerequisites
- C++ compiler (g++, clang++, MSVC)
- Standard C++ libraries

## Compilation
```bash
g++ file_encryptor.cpp -o file_encryptor
```
Or with warning flags for better practice:
```bash
g++ -Wall -Wextra file_encryptor.cpp -o file_encryptor
```

## Usage
1. Compile the program
2. **Encrypt a file**:
   ```bash
   ./file_encryptor encrypt filename.txt encrypted.dat
   ```
3. **Decrypt a file**:
   ```bash
   ./file_encryptor decrypt encrypted.dat decrypted.txt
   ```

## Example Workflow
- Original File: `filename.txt`
- Encrypted File: `encrypted.dat`
- Decrypted File: `decrypted.txt`
- To encrypt, use:
  ```bash
  ./file_encryptor encrypt filename.txt encrypted.dat
  ```
- To decrypt, use:
  ```bash
  ./file_encryptor decrypt encrypted.dat decrypted.txt
  ```

## Limitations
- **Security**: XOR method is simplistic and insecure for real-world applications
- **Key Exposure**: Fixed key used, easily guessed; not suitable for sensitive files
- **No Error Encoding**: Does not handle malformed inputs gracefully

## Educational Value
This program serves an educational tool for understanding:
- Basic file I/O in C++
- XOR logic for cryptographic purposes
- Command-line arguments handling in C++

## Advanced Extensions
- Implement more secure encryption techniques (e.g., AES, DES)
- Add support for user-defined keys
- Implement error handling for edge cases
- Add capability to process large files efficiently
- Enhance with a GUI for ease of use
- Explore additional cryptographic concepts (e.g., hashing, digital signatures)

## Learning Goals
Provides insight into:
- Cryptography fundamentals
- XOR-based ciphers
- Practical usage examples for C++ I/O streams
- Structuring and compiling C++ command-line programs

This makes the program an ideal starting point for those new to file encryption concepts and seeking to understand the basic mechanics of cryptography through simple C++ examples.
