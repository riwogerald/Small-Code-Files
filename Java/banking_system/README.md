# Simple Banking System

## Overview

A comprehensive console-based banking system built in Java that simulates real-world banking operations. This system provides secure account management, transaction processing, and administrative functions with complete data persistence and audit trails.

## Features

### Account Management
- **Account Creation**: Create new savings or checking accounts with unique 10-digit account numbers
- **Account Types**: Support for both Savings and Checking accounts
- **PIN Security**: 4-digit PIN authentication for all account access
- **Account Status**: Active/Inactive account management
- **Initial Deposits**: Optional initial deposit during account creation

### Banking Operations
- **Balance Inquiry**: Check current account balance and account details
- **Deposits**: Add money to accounts with transaction descriptions
- **Withdrawals**: Withdraw money with insufficient funds protection
- **Money Transfers**: Transfer funds between accounts with validation
- **PIN Management**: Change account PIN with current PIN verification

### Transaction System
- **Transaction History**: Complete audit trail of all transactions
- **Transaction Types**: DEPOSIT, WITHDRAWAL, TRANSFER_IN, TRANSFER_OUT
- **Unique Transaction IDs**: Each transaction gets a unique identifier
- **Timestamp Tracking**: All transactions timestamped with date and time
- **Balance Tracking**: Running balance recorded with each transaction
- **Transaction Descriptions**: Customizable descriptions for each transaction

### Administrative Functions
- **Admin Panel**: Comprehensive administrative tools
- **Account Search**: Find accounts by account number
- **Account Management**: Activate/deactivate accounts
- **Bank Reports**: Generate comprehensive banking statistics
- **Account Listing**: View all accounts with details

### Security Features
- **PIN Authentication**: Secure 4-digit PIN system
- **Account Validation**: Verify account existence and status
- **Transaction Validation**: Prevent invalid transactions
- **Insufficient Funds Protection**: Block overdrafts
- **Self-transfer Prevention**: Cannot transfer to same account

## Technical Specifications

### Classes

#### Transaction Class
```java
Transaction {
    - String transactionId (unique identifier)
    - String type (DEPOSIT, WITHDRAWAL, TRANSFER_IN, TRANSFER_OUT)
    - double amount
    - LocalDateTime timestamp
    - String description
    - double balanceAfter
}
```

#### Account Class
```java
Account {
    - String accountNumber (10-digit unique)
    - String accountHolderName
    - String accountType (SAVINGS, CHECKING)
    - double balance
    - String pin (4-digit)
    - boolean isActive
    - LocalDateTime createdDate
    - List<Transaction> transactionHistory
}
```

#### BankingSystem Class
- Main application controller
- Handles all banking operations
- Manages data persistence
- Provides user interface

### Key Methods
- `createAccount()`: Create new bank accounts
- `loginToAccount()`: Secure account login with PIN
- `checkBalance()`: Display account balance and details
- `depositMoney()`: Process deposit transactions
- `withdrawMoney()`: Process withdrawal transactions
- `transferMoney()`: Handle inter-account transfers
- `viewTransactionHistory()`: Display transaction history
- `changePin()`: Update account PIN
- `generateBankReport()`: Create comprehensive reports

## Usage Instructions

### Running the Program
1. Compile: `javac BankingSystem.java`
2. Run: `java BankingSystem`

### Main Menu Options
1. **Create New Account** - Register a new bank account
2. **Login to Account** - Access existing account
3. **Admin Panel** - Administrative functions
4. **Exit** - Save data and quit

### Account Operations (After Login)
1. **Check Balance** - View current balance and account info
2. **Deposit Money** - Add funds to account
3. **Withdraw Money** - Remove funds from account
4. **Transfer Money** - Send money to another account
5. **View Transaction History** - See last 20 transactions
6. **Change PIN** - Update account PIN
7. **Logout** - Return to main menu

### Admin Panel Operations
1. **View All Accounts** - List all bank accounts
2. **Search Account** - Find specific account
3. **Deactivate Account** - Disable account access
4. **Activate Account** - Re-enable account access
5. **Generate Bank Report** - View banking statistics

## Data Persistence

### File Storage
- **accounts.txt**: Account information storage
- **Automatic Save**: Data saved on program exit
- **Automatic Load**: Previous data loaded on program start
- **CSV Format**: Human-readable comma-separated values

### Data Format
```
AccountNumber,HolderName,AccountType,Balance,PIN,IsActive
1234567890,John Doe,SAVINGS,1500.00,1234,true
```

## Sample Usage Scenarios

### Creating an Account
```
=== Create New Account ===
Enter account holder name: John Doe
Select account type:
1. Savings
2. Checking
Choose (1-2): 1
Create a 4-digit PIN: 1234
Account created successfully!
Your account number is: 1234567890
```

### Making a Deposit
```
=== Deposit Money ===
Enter deposit amount: $500.00
Enter deposit description: Salary deposit
Deposit successful!
Amount deposited: $500.00
New balance: $1500.00
```

### Transferring Money
```
=== Transfer Money ===
Current balance: $1500.00
Enter recipient account number: 0987654321
Transfer to: Jane Smith
Enter transfer amount: $250.00
Transfer successful!
Amount transferred: $250.00
Your new balance: $1250.00
```

## Transaction History Sample
```
=== Transaction History ===
TXN1674123456789 | 2024-01-15 14:30:22 | DEPOSIT | $500.00 | $1500.00 | Salary deposit
TXN1674123987654 | 2024-01-15 15:45:10 | TRANSFER_OUT | $250.00 | $1250.00 | Transfer to 0987654321
TXN1674124567890 | 2024-01-15 16:20:05 | WITHDRAWAL | $100.00 | $1150.00 | ATM withdrawal
```

## Bank Report Sample
```
=== Bank Report ===
Total Accounts: 150
Active Accounts: 145
Inactive Accounts: 5
Savings Accounts: 95
Checking Accounts: 55
Total Bank Balance: $2,450,000.00
Total Transactions: 1,250
Average Balance per Account: $16,333.33
```

## Security Considerations

### PIN Security
- 4-digit PIN requirement
- PIN validation on every login
- Secure PIN change process
- PIN confirmation for sensitive operations

### Account Protection
- Account existence verification
- Active status checking
- Insufficient funds prevention
- Self-transfer blocking

### Data Integrity
- Unique account number generation
- Transaction ID uniqueness
- Balance consistency checking
- Atomic transaction processing

## File Structure
```
banking_system/
├── BankingSystem.java          # Main application
├── accounts.txt                # Account data (auto-generated)
└── README.md                  # This documentation
```

## Educational Value

This banking system demonstrates:
- **Object-Oriented Design**: Multiple classes with clear responsibilities
- **Data Encapsulation**: Private fields with public methods
- **Collections Framework**: Lists and Maps for data management
- **Date/Time Handling**: LocalDateTime for timestamps
- **File I/O Operations**: Reading and writing data files
- **Input Validation**: Robust error handling and validation
- **Security Concepts**: Authentication and authorization
- **Business Logic**: Real-world banking operations
- **Menu-Driven Interface**: User-friendly console navigation
- **Data Persistence**: Maintaining data between sessions

## Key Benefits
- **Realistic Banking Operations**: Simulates actual banking processes
- **Complete Audit Trail**: Full transaction history tracking
- **Secure Access**: PIN-based authentication system
- **Administrative Tools**: Comprehensive admin panel
- **Data Persistence**: No data loss between sessions
- **Error Handling**: Robust input validation and error management
- **Scalable Design**: Can handle multiple accounts and transactions
- **User-Friendly Interface**: Intuitive menu system
