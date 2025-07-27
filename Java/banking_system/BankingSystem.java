import java.util.*;
import java.io.*;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

// Transaction class to represent individual transactions
class Transaction {
    private String transactionId;
    private String type; // DEPOSIT, WITHDRAWAL, TRANSFER
    private double amount;
    private LocalDateTime timestamp;
    private String description;
    private double balanceAfter;
    
    public Transaction(String transactionId, String type, double amount, String description, double balanceAfter) {
        this.transactionId = transactionId;
        this.type = type;
        this.amount = amount;
        this.description = description;
        this.balanceAfter = balanceAfter;
        this.timestamp = LocalDateTime.now();
    }
    
    // Getters
    public String getTransactionId() { return transactionId; }
    public String getType() { return type; }
    public double getAmount() { return amount; }
    public LocalDateTime getTimestamp() { return timestamp; }
    public String getDescription() { return description; }
    public double getBalanceAfter() { return balanceAfter; }
    
    @Override
    public String toString() {
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
        return String.format("%s | %s | %s | $%.2f | $%.2f | %s", 
                transactionId, timestamp.format(formatter), type, amount, balanceAfter, description);
    }
}

// Account class to represent bank accounts
class Account {
    private String accountNumber;
    private String accountHolderName;
    private String accountType; // SAVINGS, CHECKING
    private double balance;
    private String pin;
    private boolean isActive;
    private LocalDateTime createdDate;
    private List<Transaction> transactionHistory;
    
    public Account(String accountNumber, String accountHolderName, String accountType, String pin) {
        this.accountNumber = accountNumber;
        this.accountHolderName = accountHolderName;
        this.accountType = accountType;
        this.pin = pin;
        this.balance = 0.0;
        this.isActive = true;
        this.createdDate = LocalDateTime.now();
        this.transactionHistory = new ArrayList<>();
    }
    
    // Getters and setters
    public String getAccountNumber() { return accountNumber; }
    public String getAccountHolderName() { return accountHolderName; }
    public void setAccountHolderName(String name) { this.accountHolderName = name; }
    public String getAccountType() { return accountType; }
    public double getBalance() { return balance; }
    public String getPin() { return pin; }
    public void setPin(String pin) { this.pin = pin; }
    public boolean isActive() { return isActive; }
    public void setActive(boolean active) { this.isActive = active; }
    public LocalDateTime getCreatedDate() { return createdDate; }
    public List<Transaction> getTransactionHistory() { return transactionHistory; }
    
    // Method to verify PIN
    public boolean verifyPin(String inputPin) {
        return this.pin.equals(inputPin);
    }
    
    // Method to deposit money
    public boolean deposit(double amount, String description) {
        if (amount <= 0) {
            return false;
        }
        
        this.balance += amount;
        String transactionId = generateTransactionId();
        Transaction transaction = new Transaction(transactionId, "DEPOSIT", amount, description, this.balance);
        transactionHistory.add(transaction);
        return true;
    }
    
    // Method to withdraw money
    public boolean withdraw(double amount, String description) {
        if (amount <= 0 || amount > this.balance) {
            return false;
        }
        
        this.balance -= amount;
        String transactionId = generateTransactionId();
        Transaction transaction = new Transaction(transactionId, "WITHDRAWAL", amount, description, this.balance);
        transactionHistory.add(transaction);
        return true;
    }
    
    // Method to transfer money (for sender)
    public boolean transferOut(double amount, String toAccountNumber) {
        if (amount <= 0 || amount > this.balance) {
            return false;
        }
        
        this.balance -= amount;
        String transactionId = generateTransactionId();
        String description = "Transfer to " + toAccountNumber;
        Transaction transaction = new Transaction(transactionId, "TRANSFER_OUT", amount, description, this.balance);
        transactionHistory.add(transaction);
        return true;
    }
    
    // Method to receive transfer (for receiver)
    public void transferIn(double amount, String fromAccountNumber) {
        this.balance += amount;
        String transactionId = generateTransactionId();
        String description = "Transfer from " + fromAccountNumber;
        Transaction transaction = new Transaction(transactionId, "TRANSFER_IN", amount, description, this.balance);
        transactionHistory.add(transaction);
    }
    
    // Method to generate unique transaction ID
    private String generateTransactionId() {
        return "TXN" + System.currentTimeMillis() + (int)(Math.random() * 1000);
    }
    
    // Method to get account summary
    public String getAccountSummary() {
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd");
        return String.format("Account: %s | Holder: %s | Type: %s | Balance: $%.2f | Status: %s | Created: %s",
                accountNumber, accountHolderName, accountType, balance, 
                isActive ? "Active" : "Inactive", createdDate.format(formatter));
    }
    
    @Override
    public String toString() {
        return getAccountSummary();
    }
}

// Main BankingSystem class
public class BankingSystem {
    private Map<String, Account> accounts;
    private Scanner scanner;
    private final String ACCOUNTS_FILE = "accounts.txt";
    private final String TRANSACTIONS_FILE = "transactions.txt";
    
    public BankingSystem() {
        accounts = new HashMap<>();
        scanner = new Scanner(System.in);
        loadAccountsFromFile();
    }
    
    // Main menu
    public void displayMainMenu() {
        System.out.println("\n=== Welcome to Simple Banking System ===");
        System.out.println("1. Create New Account");
        System.out.println("2. Login to Account");
        System.out.println("3. Admin Panel");
        System.out.println("4. Exit");
        System.out.print("Choose an option (1-4): ");
    }
    
    // Account menu (after login)
    public void displayAccountMenu(String accountNumber) {
        Account account = accounts.get(accountNumber);
        System.out.println("\n=== Account Menu ===");
        System.out.println("Welcome, " + account.getAccountHolderName() + "!");
        System.out.println("Account: " + accountNumber + " | Balance: $" + String.format("%.2f", account.getBalance()));
        System.out.println("1. Check Balance");
        System.out.println("2. Deposit Money");
        System.out.println("3. Withdraw Money");
        System.out.println("4. Transfer Money");
        System.out.println("5. View Transaction History");
        System.out.println("6. Change PIN");
        System.out.println("7. Logout");
        System.out.print("Choose an option (1-7): ");
    }
    
    // Admin menu
    public void displayAdminMenu() {
        System.out.println("\n=== Admin Panel ===");
        System.out.println("1. View All Accounts");
        System.out.println("2. Search Account");
        System.out.println("3. Deactivate Account");
        System.out.println("4. Activate Account");
        System.out.println("5. Generate Bank Report");
        System.out.println("6. Back to Main Menu");
        System.out.print("Choose an option (1-6): ");
    }
    
    // Method to create new account
    public void createAccount() {
        System.out.println("\n=== Create New Account ===");
        
        System.out.print("Enter account holder name: ");
        scanner.nextLine(); // consume newline
        String name = scanner.nextLine();
        
        System.out.println("Select account type:");
        System.out.println("1. Savings");
        System.out.println("2. Checking");
        System.out.print("Choose (1-2): ");
        int typeChoice = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        String accountType = (typeChoice == 1) ? "SAVINGS" : "CHECKING";
        
        System.out.print("Create a 4-digit PIN: ");
        String pin = scanner.nextLine();
        
        if (pin.length() != 4 || !pin.matches("\\d{4}")) {
            System.out.println("Error: PIN must be exactly 4 digits!");
            return;
        }
        
        String accountNumber = generateAccountNumber();
        Account account = new Account(accountNumber, name, accountType, pin);
        accounts.put(accountNumber, account);
        
        System.out.println("Account created successfully!");
        System.out.println("Your account number is: " + accountNumber);
        System.out.println("Please keep your account number and PIN safe.");
        
        // Initial deposit
        System.out.print("Would you like to make an initial deposit? (y/n): ");
        String choice = scanner.nextLine();
        if (choice.toLowerCase().equals("y")) {
            System.out.print("Enter initial deposit amount: $");
            double amount = scanner.nextDouble();
            if (account.deposit(amount, "Initial deposit")) {
                System.out.println("Initial deposit of $" + String.format("%.2f", amount) + " successful!");
            }
        }
    }
    
    // Method to generate unique account number
    private String generateAccountNumber() {
        String accountNumber;
        do {
            accountNumber = String.format("%010d", (int)(Math.random() * 1000000000));
        } while (accounts.containsKey(accountNumber));
        return accountNumber;
    }
    
    // Method to login to account
    public String loginToAccount() {
        System.out.println("\n=== Account Login ===");
        
        System.out.print("Enter account number: ");
        scanner.nextLine(); // consume newline
        String accountNumber = scanner.nextLine();
        
        Account account = accounts.get(accountNumber);
        if (account == null) {
            System.out.println("Account not found!");
            return null;
        }
        
        if (!account.isActive()) {
            System.out.println("Account is deactivated. Please contact admin.");
            return null;
        }
        
        System.out.print("Enter PIN: ");
        String pin = scanner.nextLine();
        
        if (!account.verifyPin(pin)) {
            System.out.println("Invalid PIN!");
            return null;
        }
        
        System.out.println("Login successful!");
        return accountNumber;
    }
    
    // Method to check balance
    public void checkBalance(String accountNumber) {
        Account account = accounts.get(accountNumber);
        System.out.println("\n=== Account Balance ===");
        System.out.println("Account Number: " + accountNumber);
        System.out.println("Account Holder: " + account.getAccountHolderName());
        System.out.println("Account Type: " + account.getAccountType());
        System.out.println("Current Balance: $" + String.format("%.2f", account.getBalance()));
    }
    
    // Method to deposit money
    public void depositMoney(String accountNumber) {
        Account account = accounts.get(accountNumber);
        
        System.out.println("\n=== Deposit Money ===");
        System.out.print("Enter deposit amount: $");
        double amount = scanner.nextDouble();
        scanner.nextLine(); // consume newline
        
        if (amount <= 0) {
            System.out.println("Invalid amount. Amount must be positive.");
            return;
        }
        
        System.out.print("Enter deposit description (optional): ");
        String description = scanner.nextLine();
        if (description.trim().isEmpty()) {
            description = "Cash deposit";
        }
        
        if (account.deposit(amount, description)) {
            System.out.println("Deposit successful!");
            System.out.println("Amount deposited: $" + String.format("%.2f", amount));
            System.out.println("New balance: $" + String.format("%.2f", account.getBalance()));
        } else {
            System.out.println("Deposit failed!");
        }
    }
    
    // Method to withdraw money
    public void withdrawMoney(String accountNumber) {
        Account account = accounts.get(accountNumber);
        
        System.out.println("\n=== Withdraw Money ===");
        System.out.println("Current balance: $" + String.format("%.2f", account.getBalance()));
        System.out.print("Enter withdrawal amount: $");
        double amount = scanner.nextDouble();
        scanner.nextLine(); // consume newline
        
        if (amount <= 0) {
            System.out.println("Invalid amount. Amount must be positive.");
            return;
        }
        
        if (amount > account.getBalance()) {
            System.out.println("Insufficient funds!");
            return;
        }
        
        System.out.print("Enter withdrawal description (optional): ");
        String description = scanner.nextLine();
        if (description.trim().isEmpty()) {
            description = "Cash withdrawal";
        }
        
        if (account.withdraw(amount, description)) {
            System.out.println("Withdrawal successful!");
            System.out.println("Amount withdrawn: $" + String.format("%.2f", amount));
            System.out.println("New balance: $" + String.format("%.2f", account.getBalance()));
        } else {
            System.out.println("Withdrawal failed!");
        }
    }
    
    // Method to transfer money
    public void transferMoney(String fromAccountNumber) {
        Account fromAccount = accounts.get(fromAccountNumber);
        
        System.out.println("\n=== Transfer Money ===");
        System.out.println("Current balance: $" + String.format("%.2f", fromAccount.getBalance()));
        
        System.out.print("Enter recipient account number: ");
        scanner.nextLine(); // consume newline
        String toAccountNumber = scanner.nextLine();
        
        Account toAccount = accounts.get(toAccountNumber);
        if (toAccount == null) {
            System.out.println("Recipient account not found!");
            return;
        }
        
        if (!toAccount.isActive()) {
            System.out.println("Recipient account is inactive!");
            return;
        }
        
        if (toAccountNumber.equals(fromAccountNumber)) {
            System.out.println("Cannot transfer to the same account!");
            return;
        }
        
        System.out.println("Transfer to: " + toAccount.getAccountHolderName());
        System.out.print("Enter transfer amount: $");
        double amount = scanner.nextDouble();
        
        if (amount <= 0) {
            System.out.println("Invalid amount. Amount must be positive.");
            return;
        }
        
        if (amount > fromAccount.getBalance()) {
            System.out.println("Insufficient funds!");
            return;
        }
        
        // Perform transfer
        if (fromAccount.transferOut(amount, toAccountNumber)) {
            toAccount.transferIn(amount, fromAccountNumber);
            
            System.out.println("Transfer successful!");
            System.out.println("Amount transferred: $" + String.format("%.2f", amount));
            System.out.println("To: " + toAccount.getAccountHolderName() + " (" + toAccountNumber + ")");
            System.out.println("Your new balance: $" + String.format("%.2f", fromAccount.getBalance()));
        } else {
            System.out.println("Transfer failed!");
        }
    }
    
    // Method to view transaction history
    public void viewTransactionHistory(String accountNumber) {
        Account account = accounts.get(accountNumber);
        
        System.out.println("\n=== Transaction History ===");
        System.out.println("Account: " + accountNumber + " (" + account.getAccountHolderName() + ")");
        
        List<Transaction> history = account.getTransactionHistory();
        if (history.isEmpty()) {
            System.out.println("No transactions found.");
            return;
        }
        
        System.out.println("Transaction ID | Date & Time | Type | Amount | Balance After | Description");
        System.out.println("--------------------------------------------------------------------------------");
        
        // Show last 20 transactions
        int startIndex = Math.max(0, history.size() - 20);
        for (int i = history.size() - 1; i >= startIndex; i--) {
            System.out.println(history.get(i));
        }
        
        System.out.println("Total transactions: " + history.size());
    }
    
    // Method to change PIN
    public void changePin(String accountNumber) {
        Account account = accounts.get(accountNumber);
        
        System.out.println("\n=== Change PIN ===");
        System.out.print("Enter current PIN: ");
        scanner.nextLine(); // consume newline
        String currentPin = scanner.nextLine();
        
        if (!account.verifyPin(currentPin)) {
            System.out.println("Invalid current PIN!");
            return;
        }
        
        System.out.print("Enter new 4-digit PIN: ");
        String newPin = scanner.nextLine();
        
        if (newPin.length() != 4 || !newPin.matches("\\d{4}")) {
            System.out.println("Error: PIN must be exactly 4 digits!");
            return;
        }
        
        System.out.print("Confirm new PIN: ");
        String confirmPin = scanner.nextLine();
        
        if (!newPin.equals(confirmPin)) {
            System.out.println("PINs do not match!");
            return;
        }
        
        account.setPin(newPin);
        System.out.println("PIN changed successfully!");
    }
    
    // Admin methods
    public void viewAllAccounts() {
        System.out.println("\n=== All Accounts ===");
        if (accounts.isEmpty()) {
            System.out.println("No accounts found.");
            return;
        }
        
        System.out.println("Total accounts: " + accounts.size());
        System.out.println("Account Number | Holder Name | Type | Balance | Status | Created Date");
        System.out.println("------------------------------------------------------------------------------");
        
        for (Account account : accounts.values()) {
            System.out.println(account);
        }
    }
    
    public void searchAccount() {
        System.out.print("\nEnter account number to search: ");
        scanner.nextLine(); // consume newline
        String accountNumber = scanner.nextLine();
        
        Account account = accounts.get(accountNumber);
        if (account == null) {
            System.out.println("Account not found!");
            return;
        }
        
        System.out.println("\n=== Account Details ===");
        System.out.println(account);
        System.out.println("Total transactions: " + account.getTransactionHistory().size());
    }
    
    public void deactivateAccount() {
        System.out.print("\nEnter account number to deactivate: ");
        scanner.nextLine(); // consume newline
        String accountNumber = scanner.nextLine();
        
        Account account = accounts.get(accountNumber);
        if (account == null) {
            System.out.println("Account not found!");
            return;
        }
        
        if (!account.isActive()) {
            System.out.println("Account is already inactive!");
            return;
        }
        
        account.setActive(false);
        System.out.println("Account deactivated successfully!");
    }
    
    public void activateAccount() {
        System.out.print("\nEnter account number to activate: ");
        scanner.nextLine(); // consume newline
        String accountNumber = scanner.nextLine();
        
        Account account = accounts.get(accountNumber);
        if (account == null) {
            System.out.println("Account not found!");
            return;
        }
        
        if (account.isActive()) {
            System.out.println("Account is already active!");
            return;
        }
        
        account.setActive(true);
        System.out.println("Account activated successfully!");
    }
    
    public void generateBankReport() {
        System.out.println("\n=== Bank Report ===");
        int totalAccounts = accounts.size();
        int activeAccounts = 0;
        int savingsAccounts = 0;
        int checkingAccounts = 0;
        double totalBalance = 0;
        int totalTransactions = 0;
        
        for (Account account : accounts.values()) {
            if (account.isActive()) activeAccounts++;
            if (account.getAccountType().equals("SAVINGS")) savingsAccounts++;
            else checkingAccounts++;
            totalBalance += account.getBalance();
            totalTransactions += account.getTransactionHistory().size();
        }
        
        System.out.println("Total Accounts: " + totalAccounts);
        System.out.println("Active Accounts: " + activeAccounts);
        System.out.println("Inactive Accounts: " + (totalAccounts - activeAccounts));
        System.out.println("Savings Accounts: " + savingsAccounts);
        System.out.println("Checking Accounts: " + checkingAccounts);
        System.out.println("Total Bank Balance: $" + String.format("%.2f", totalBalance));
        System.out.println("Total Transactions: " + totalTransactions);
        System.out.println("Average Balance per Account: $" + String.format("%.2f", 
                totalAccounts > 0 ? totalBalance / totalAccounts : 0));
    }
    
    // File operations
    public void saveAccountsToFile() {
        // Implementation for saving accounts to file
        try (PrintWriter writer = new PrintWriter(new FileWriter(ACCOUNTS_FILE))) {
            for (Account account : accounts.values()) {
                writer.println(account.getAccountNumber() + "," + 
                             account.getAccountHolderName() + "," + 
                             account.getAccountType() + "," + 
                             account.getBalance() + "," + 
                             account.getPin() + "," + 
                             account.isActive());
            }
        } catch (IOException e) {
            System.out.println("Error saving accounts: " + e.getMessage());
        }
    }
    
    public void loadAccountsFromFile() {
        // Implementation for loading accounts from file
        try (BufferedReader reader = new BufferedReader(new FileReader(ACCOUNTS_FILE))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] parts = line.split(",");
                if (parts.length >= 6) {
                    String accountNumber = parts[0];
                    String name = parts[1];
                    String type = parts[2];
                    double balance = Double.parseDouble(parts[3]);
                    String pin = parts[4];
                    boolean isActive = Boolean.parseBoolean(parts[5]);
                    
                    Account account = new Account(accountNumber, name, type, pin);
                    account.deposit(balance, "Initial balance");
                    account.setActive(isActive);
                    accounts.put(accountNumber, account);
                }
            }
        } catch (FileNotFoundException e) {
            // File doesn't exist, start with empty bank
        } catch (IOException e) {
            System.out.println("Error loading accounts: " + e.getMessage());
        }
    }
    
    // Main execution method
    public void run() {
        System.out.println("=== SIMPLE BANKING SYSTEM ===");
        System.out.println("Welcome to the Bank!");
        
        while (true) {
            displayMainMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1:
                        createAccount();
                        break;
                    case 2:
                        String loggedInAccount = loginToAccount();
                        if (loggedInAccount != null) {
                            handleAccountOperations(loggedInAccount);
                        }
                        break;
                    case 3:
                        handleAdminOperations();
                        break;
                    case 4:
                        saveAccountsToFile();
                        System.out.println("Thank you for using Simple Banking System!");
                        return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine(); // consume invalid input
            }
        }
    }
    
    // Handle account operations after login
    private void handleAccountOperations(String accountNumber) {
        while (true) {
            displayAccountMenu(accountNumber);
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: checkBalance(accountNumber); break;
                    case 2: depositMoney(accountNumber); break;
                    case 3: withdrawMoney(accountNumber); break;
                    case 4: transferMoney(accountNumber); break;
                    case 5: viewTransactionHistory(accountNumber); break;
                    case 6: changePin(accountNumber); break;
                    case 7: 
                        System.out.println("Logged out successfully!");
                        return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine(); // consume invalid input
            }
        }
    }
    
    // Handle admin operations
    private void handleAdminOperations() {
        while (true) {
            displayAdminMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: viewAllAccounts(); break;
                    case 2: searchAccount(); break;
                    case 3: deactivateAccount(); break;
                    case 4: activateAccount(); break;
                    case 5: generateBankReport(); break;
                    case 6: return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine(); // consume invalid input
            }
        }
    }
    
    // Main method
    public static void main(String[] args) {
        BankingSystem bank = new BankingSystem();
        bank.run();
    }
}
