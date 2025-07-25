# Student Fee Management System

A C++ console application demonstrating object-oriented programming principles through a student fee management system. The program uses classes, static members, and encapsulation to manage student information and fee tracking.

## Overview

This program implements a `Student` class with fee management capabilities and a `StudentApp` class for demonstration. It showcases fundamental OOP concepts including constructors, getters/setters, static members, and formatted output display.

## Features

- **Student Class**: Complete student information management
- **Fee Tracking**: Tracks total fees, payments made, and remaining balance
- **Static Fee Management**: Uses static member for uniform fee structure across all students
- **Encapsulation**: Private data members with public getter/setter methods
- **Payment System**: Method to process fee payments with validation
- **Formatted Display**: Professional-looking output with proper alignment and formatting

## Class Structure

### Student Class
```cpp
class Student {
private:
    string registrationNumber;    // Student's unique registration number
    string name;                 // Student's full name
    static float feetobepaid;    // Static fee amount (same for all students)
    float feesPaid;             // Amount of fees paid by student
    float balance;              // Remaining fee balance
    
public:
    // Constructor, getters, setters, and payment method
};
```

### StudentApp Class
```cpp
class StudentApp {
public:
    static void displayStudentInfo();  // Displays formatted student information
};
```

## Sample Output

```
========================================
          STUDENT INFORMATION          
========================================

STUDENT 1:
----------
Name: John Doe
Registration Number: REG001
Total Fee: $5000.00
Fees Paid: $2000.00
Fee Balance: $3000.00

STUDENT 2:
----------
Name: Jane Smith
Registration Number: REG002
Total Fee: $5000.00
Fees Paid: $3500.00
Fee Balance: $1500.00

========================================
```

## Technical Specifications

- **Language**: C++
- **Programming Paradigm**: Object-Oriented Programming
- **Default Fee Amount**: $5000.00
- **Precision**: 2 decimal places for monetary values
- **Memory Management**: Automatic (stack-based objects)

## Key OOP Concepts Demonstrated

### 1. Encapsulation
- Private data members
- Public getter and setter methods
- Controlled access to class data

### 2. Static Members
- Static `feetobepaid` variable shared across all instances
- Static getter and setter methods for fee management

### 3. Constructor
- Parameterized constructor accepting registration number and name
- Automatic initialization of fee-related members

### 4. Data Validation
- Payment validation in `makePayment()` method
- Prevents overpayment and negative payments

## Prerequisites

- C++ compiler with C++11 support or later
- Standard C++ library (iostream, string, iomanip)

## Compilation

```bash
g++ student.cpp -o student
```

With additional flags for better code quality:
```bash
g++ -Wall -std=c++11 student.cpp -o student
```

## Usage

1. Compile the program
2. Run the executable:
   ```bash
   ./student
   ```
3. View the displayed student information

## Key Methods

### Student Class Methods
- `Student(string, string)`: Constructor
- `getRegistrationNumber()` / `setRegistrationNumber()`: Registration number access
- `getName()` / `setName()`: Name access
- `getFeetobepaid()` / `setFeetobepaid()`: Static fee amount access
- `getFeesPaid()`: Returns amount paid
- `getBalance()`: Returns remaining balance
- `makePayment(float)`: Processes fee payment

### StudentApp Methods
- `displayStudentInfo()`: Demonstrates the student system with formatted output

## Educational Value

This program is excellent for learning:

### Object-Oriented Programming
- Class design and implementation
- Encapsulation principles
- Static vs instance members
- Constructor usage

### C++ Specific Features
- String manipulation
- Input/output formatting with iomanip
- Static member initialization
- Method overloading concepts

### Software Design
- Separation of concerns (Student vs StudentApp)
- Data validation techniques
- Professional output formatting

## Possible Extensions

You can extend this program by:

### Enhanced Features
- File I/O for persistent student data
- Multiple payment types and dates
- Student grade management
- Database integration

### Advanced OOP
- Inheritance (Graduate, Undergraduate students)
- Polymorphism with virtual methods
- Operator overloading for student comparison

### User Interface
- Interactive menu system
- Input validation for user entries
- Exception handling
- Command-line argument processing

## Learning Objectives

Perfect for understanding:
- Class design principles
- Static member variables and methods
- Encapsulation and data hiding
- Constructor implementation
- Professional code formatting
- Object lifecycle management
- Method design and validation

## Code Organization

1. **Class Definitions**: Student and StudentApp classes
2. **Static Member Initialization**: Required for static variables
3. **Demonstration Code**: StudentApp showing practical usage
4. **Main Function**: Simple entry point calling the demonstration

This program serves as an excellent foundation for more complex student management systems and demonstrates essential OOP principles in a practical, understandable context.
