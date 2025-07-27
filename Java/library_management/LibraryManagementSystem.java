import java.util.*;
import java.io.*;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.temporal.ChronoUnit;

// Book class to represent library books
class Book {
    private String isbn;
    private String title;
    private String author;
    private String genre;
    private int publicationYear;
    private boolean isAvailable;
    private LocalDate dueDate;
    private String borrowedBy;
    
    public Book(String isbn, String title, String author, String genre, int publicationYear) {
        this.isbn = isbn;
        this.title = title;
        this.author = author;
        this.genre = genre;
        this.publicationYear = publicationYear;
        this.isAvailable = true;
        this.dueDate = null;
        this.borrowedBy = null;
    }
    
    // Getters and setters
    public String getIsbn() { return isbn; }
    public String getTitle() { return title; }
    public void setTitle(String title) { this.title = title; }
    public String getAuthor() { return author; }
    public void setAuthor(String author) { this.author = author; }
    public String getGenre() { return genre; }
    public void setGenre(String genre) { this.genre = genre; }
    public int getPublicationYear() { return publicationYear; }
    public void setPublicationYear(int year) { this.publicationYear = year; }
    public boolean isAvailable() { return isAvailable; }
    public LocalDate getDueDate() { return dueDate; }
    public String getBorrowedBy() { return borrowedBy; }
    
    // Method to borrow book
    public boolean borrowBook(String memberId, int loanDays) {
        if (!isAvailable) {
            return false;
        }
        
        this.isAvailable = false;
        this.borrowedBy = memberId;
        this.dueDate = LocalDate.now().plusDays(loanDays);
        return true;
    }
    
    // Method to return book
    public void returnBook() {
        this.isAvailable = true;
        this.borrowedBy = null;
        this.dueDate = null;
    }
    
    // Method to check if book is overdue
    public boolean isOverdue() {
        return !isAvailable && dueDate != null && LocalDate.now().isAfter(dueDate);
    }
    
    // Method to get days overdue
    public long getDaysOverdue() {
        if (!isOverdue()) {
            return 0;
        }
        return ChronoUnit.DAYS.between(dueDate, LocalDate.now());
    }
    
    // Method to extend loan
    public boolean extendLoan(int additionalDays) {
        if (isAvailable) {
            return false;
        }
        
        this.dueDate = this.dueDate.plusDays(additionalDays);
        return true;
    }
    
    @Override
    public String toString() {
        String status = isAvailable ? "Available" : "Borrowed by " + borrowedBy;
        String dueDateStr = (dueDate != null) ? " (Due: " + dueDate.format(DateTimeFormatter.ofPattern("yyyy-MM-dd")) + ")" : "";
        return String.format("ISBN: %s | %s by %s | Genre: %s | Year: %d | Status: %s%s",
                isbn, title, author, genre, publicationYear, status, dueDateStr);
    }
}

// Member class to represent library members
class Member {
    private String memberId;
    private String name;
    private String email;
    private String phone;
    private LocalDate membershipDate;
    private List<String> borrowedBooks;
    private double fineAmount;
    private boolean isActive;
    
    public Member(String memberId, String name, String email, String phone) {
        this.memberId = memberId;
        this.name = name;
        this.email = email;
        this.phone = phone;
        this.membershipDate = LocalDate.now();
        this.borrowedBooks = new ArrayList<>();
        this.fineAmount = 0.0;
        this.isActive = true;
    }
    
    // Getters and setters
    public String getMemberId() { return memberId; }
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }
    public String getPhone() { return phone; }
    public void setPhone(String phone) { this.phone = phone; }
    public LocalDate getMembershipDate() { return membershipDate; }
    public List<String> getBorrowedBooks() { return borrowedBooks; }
    public double getFineAmount() { return fineAmount; }
    public void setFineAmount(double fineAmount) { this.fineAmount = fineAmount; }
    public boolean isActive() { return isActive; }
    public void setActive(boolean active) { this.isActive = active; }
    
    // Method to add borrowed book
    public void addBorrowedBook(String isbn) {
        if (!borrowedBooks.contains(isbn)) {
            borrowedBooks.add(isbn);
        }
    }
    
    // Method to remove borrowed book
    public boolean removeBorrowedBook(String isbn) {
        return borrowedBooks.remove(isbn);
    }
    
    // Method to add fine
    public void addFine(double amount) {
        this.fineAmount += amount;
    }
    
    // Method to pay fine
    public boolean payFine(double amount) {
        if (amount <= 0 || amount > fineAmount) {
            return false;
        }
        
        this.fineAmount -= amount;
        return true;
    }
    
    // Method to check if member can borrow more books
    public boolean canBorrow() {
        return isActive && borrowedBooks.size() < 5; // Max 5 books per member
    }
    
    @Override
    public String toString() {
        return String.format("ID: %s | %s | Email: %s | Phone: %s | Books: %d | Fine: $%.2f | Status: %s | Member since: %s",
                memberId, name, email, phone, borrowedBooks.size(), fineAmount, 
                isActive ? "Active" : "Inactive", membershipDate.format(DateTimeFormatter.ofPattern("yyyy-MM-dd")));
    }
}

// Main LibraryManagementSystem class
public class LibraryManagementSystem {
    private Map<String, Book> books;
    private Map<String, Member> members;
    private Scanner scanner;
    private final String BOOKS_FILE = "books.txt";
    private final String MEMBERS_FILE = "members.txt";
    private final double FINE_PER_DAY = 1.0; // $1 per day overdue
    
    public LibraryManagementSystem() {
        books = new HashMap<>();
        members = new HashMap<>();
        scanner = new Scanner(System.in);
        loadDataFromFiles();
    }
    
    // Main menu
    public void displayMainMenu() {
        System.out.println("\n=== Library Management System ===");
        System.out.println("1. Book Management");
        System.out.println("2. Member Management");
        System.out.println("3. Borrowing & Returning");
        System.out.println("4. Search & Reports");
        System.out.println("5. Fine Management");
        System.out.println("6. Save & Exit");
        System.out.print("Choose an option (1-6): ");
    }
    
    // Book management menu
    public void displayBookMenu() {
        System.out.println("\n=== Book Management ===");
        System.out.println("1. Add New Book");
        System.out.println("2. View All Books");
        System.out.println("3. Search Books");
        System.out.println("4. Update Book Information");
        System.out.println("5. Remove Book");
        System.out.println("6. Back to Main Menu");
        System.out.print("Choose an option (1-6): ");
    }
    
    // Member management menu
    public void displayMemberMenu() {
        System.out.println("\n=== Member Management ===");
        System.out.println("1. Add New Member");
        System.out.println("2. View All Members");
        System.out.println("3. Search Members");
        System.out.println("4. Update Member Information");
        System.out.println("5. Deactivate/Activate Member");
        System.out.println("6. Back to Main Menu");
        System.out.print("Choose an option (1-6): ");
    }
    
    // Borrowing menu
    public void displayBorrowingMenu() {
        System.out.println("\n=== Borrowing & Returning ===");
        System.out.println("1. Borrow Book");
        System.out.println("2. Return Book");
        System.out.println("3. Extend Loan");
        System.out.println("4. View Member's Borrowed Books");
        System.out.println("5. View Overdue Books");
        System.out.println("6. Back to Main Menu");
        System.out.print("Choose an option (1-6): ");
    }
    
    // Book management methods
    public void addBook() {
        System.out.println("\n=== Add New Book ===");
        
        System.out.print("Enter ISBN: ");
        scanner.nextLine(); // consume newline
        String isbn = scanner.nextLine();
        
        if (books.containsKey(isbn)) {
            System.out.println("Error: Book with this ISBN already exists!");
            return;
        }
        
        System.out.print("Enter Title: ");
        String title = scanner.nextLine();
        
        System.out.print("Enter Author: ");
        String author = scanner.nextLine();
        
        System.out.print("Enter Genre: ");
        String genre = scanner.nextLine();
        
        System.out.print("Enter Publication Year: ");
        int year = scanner.nextInt();
        
        Book book = new Book(isbn, title, author, genre, year);
        books.put(isbn, book);
        
        System.out.println("Book added successfully!");
        System.out.println(book);
    }
    
    public void viewAllBooks() {
        System.out.println("\n=== All Books ===");
        if (books.isEmpty()) {
            System.out.println("No books found in the library.");
            return;
        }
        
        System.out.println("Total books: " + books.size());
        System.out.println();
        
        // Separate available and borrowed books
        List<Book> availableBooks = new ArrayList<>();
        List<Book> borrowedBooks = new ArrayList<>();
        
        for (Book book : books.values()) {
            if (book.isAvailable()) {
                availableBooks.add(book);
            } else {
                borrowedBooks.add(book);
            }
        }
        
        System.out.println("=== Available Books (" + availableBooks.size() + ") ===");
        for (Book book : availableBooks) {
            System.out.println(book);
        }
        
        System.out.println("\n=== Borrowed Books (" + borrowedBooks.size() + ") ===");
        for (Book book : borrowedBooks) {
            System.out.println(book);
            if (book.isOverdue()) {
                System.out.println("  *** OVERDUE by " + book.getDaysOverdue() + " days ***");
            }
        }
    }
    
    public void searchBooks() {
        System.out.println("\n=== Search Books ===");
        System.out.println("Search by:");
        System.out.println("1. Title");
        System.out.println("2. Author");
        System.out.println("3. Genre");
        System.out.println("4. ISBN");
        System.out.print("Choose search type (1-4): ");
        
        int searchType = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        System.out.print("Enter search term: ");
        String searchTerm = scanner.nextLine().toLowerCase();
        
        List<Book> results = new ArrayList<>();
        
        for (Book book : books.values()) {
            boolean matches = false;
            
            switch (searchType) {
                case 1: // Title
                    matches = book.getTitle().toLowerCase().contains(searchTerm);
                    break;
                case 2: // Author
                    matches = book.getAuthor().toLowerCase().contains(searchTerm);
                    break;
                case 3: // Genre
                    matches = book.getGenre().toLowerCase().contains(searchTerm);
                    break;
                case 4: // ISBN
                    matches = book.getIsbn().toLowerCase().contains(searchTerm);
                    break;
            }
            
            if (matches) {
                results.add(book);
            }
        }
        
        if (results.isEmpty()) {
            System.out.println("No books found matching your search.");
        } else {
            System.out.println("\nSearch Results (" + results.size() + " found):");
            for (Book book : results) {
                System.out.println(book);
            }
        }
    }
    
    // Member management methods
    public void addMember() {
        System.out.println("\n=== Add New Member ===");
        
        System.out.print("Enter Member ID: ");
        scanner.nextLine(); // consume newline
        String memberId = scanner.nextLine();
        
        if (members.containsKey(memberId)) {
            System.out.println("Error: Member with this ID already exists!");
            return;
        }
        
        System.out.print("Enter Name: ");
        String name = scanner.nextLine();
        
        System.out.print("Enter Email: ");
        String email = scanner.nextLine();
        
        System.out.print("Enter Phone: ");
        String phone = scanner.nextLine();
        
        Member member = new Member(memberId, name, email, phone);
        members.put(memberId, member);
        
        System.out.println("Member added successfully!");
        System.out.println(member);
    }
    
    public void viewAllMembers() {
        System.out.println("\n=== All Members ===");
        if (members.isEmpty()) {
            System.out.println("No members found.");
            return;
        }
        
        System.out.println("Total members: " + members.size());
        System.out.println();
        
        for (Member member : members.values()) {
            System.out.println(member);
        }
    }
    
    // Borrowing methods
    public void borrowBook() {
        System.out.println("\n=== Borrow Book ===");
        
        System.out.print("Enter Member ID: ");
        scanner.nextLine(); // consume newline
        String memberId = scanner.nextLine();
        
        Member member = members.get(memberId);
        if (member == null) {
            System.out.println("Member not found!");
            return;
        }
        
        if (!member.isActive()) {
            System.out.println("Member account is inactive!");
            return;
        }
        
        if (!member.canBorrow()) {
            System.out.println("Member has reached borrowing limit or has unpaid fines!");
            return;
        }
        
        System.out.print("Enter Book ISBN: ");
        String isbn = scanner.nextLine();
        
        Book book = books.get(isbn);
        if (book == null) {
            System.out.println("Book not found!");
            return;
        }
        
        if (!book.isAvailable()) {
            System.out.println("Book is already borrowed!");
            System.out.println("Current borrower: " + book.getBorrowedBy());
            System.out.println("Due date: " + book.getDueDate());
            return;
        }
        
        System.out.print("Enter loan period (days) [default: 14]: ");
        String loanInput = scanner.nextLine();
        int loanDays = loanInput.isEmpty() ? 14 : Integer.parseInt(loanInput);
        
        if (book.borrowBook(memberId, loanDays)) {
            member.addBorrowedBook(isbn);
            System.out.println("Book borrowed successfully!");
            System.out.println("Due date: " + book.getDueDate());
        } else {
            System.out.println("Failed to borrow book!");
        }
    }
    
    public void returnBook() {
        System.out.println("\n=== Return Book ===");
        
        System.out.print("Enter Book ISBN: ");
        scanner.nextLine(); // consume newline
        String isbn = scanner.nextLine();
        
        Book book = books.get(isbn);
        if (book == null) {
            System.out.println("Book not found!");
            return;
        }
        
        if (book.isAvailable()) {
            System.out.println("Book is not currently borrowed!");
            return;
        }
        
        String borrowerId = book.getBorrowedBy();
        Member member = members.get(borrowerId);
        
        // Calculate fine if overdue
        double fine = 0.0;
        if (book.isOverdue()) {
            long daysOverdue = book.getDaysOverdue();
            fine = daysOverdue * FINE_PER_DAY;
            member.addFine(fine);
            System.out.println("Book is overdue by " + daysOverdue + " days.");
            System.out.println("Fine of $" + String.format("%.2f", fine) + " has been added to member's account.");
        }
        
        book.returnBook();
        member.removeBorrowedBook(isbn);
        
        System.out.println("Book returned successfully!");
        if (fine > 0) {
            System.out.println("Member " + borrowerId + " now owes $" + String.format("%.2f", member.getFineAmount()) + " in fines.");
        }
    }
    
    public void viewOverdueBooks() {
        System.out.println("\n=== Overdue Books ===");
        
        List<Book> overdueBooks = new ArrayList<>();
        for (Book book : books.values()) {
            if (book.isOverdue()) {
                overdueBooks.add(book);
            }
        }
        
        if (overdueBooks.isEmpty()) {
            System.out.println("No overdue books found!");
            return;
        }
        
        System.out.println("Total overdue books: " + overdueBooks.size());
        System.out.println();
        
        for (Book book : overdueBooks) {
            Member member = members.get(book.getBorrowedBy());
            long daysOverdue = book.getDaysOverdue();
            double potentialFine = daysOverdue * FINE_PER_DAY;
            
            System.out.println(book);
            System.out.println("  Borrower: " + member.getName() + " (" + member.getMemberId() + ")");
            System.out.println("  Days overdue: " + daysOverdue);
            System.out.println("  Potential fine: $" + String.format("%.2f", potentialFine));
            System.out.println();
        }
    }
    
    // Fine management methods
    public void manageFines() {
        System.out.println("\n=== Fine Management ===");
        System.out.println("1. View All Fines");
        System.out.println("2. Pay Fine");
        System.out.println("3. Waive Fine");
        System.out.println("4. Back to Main Menu");
        System.out.print("Choose an option (1-4): ");
        
        int choice = scanner.nextInt();
        
        switch (choice) {
            case 1: viewAllFines(); break;
            case 2: payFine(); break;
            case 3: waiveFine(); break;
            case 4: return;
        }
    }
    
    public void viewAllFines() {
        System.out.println("\n=== All Outstanding Fines ===");
        
        List<Member> membersWithFines = new ArrayList<>();
        double totalFines = 0.0;
        
        for (Member member : members.values()) {
            if (member.getFineAmount() > 0) {
                membersWithFines.add(member);
                totalFines += member.getFineAmount();
            }
        }
        
        if (membersWithFines.isEmpty()) {
            System.out.println("No outstanding fines!");
            return;
        }
        
        System.out.println("Members with outstanding fines: " + membersWithFines.size());
        System.out.println("Total fine amount: $" + String.format("%.2f", totalFines));
        System.out.println();
        
        for (Member member : membersWithFines) {
            System.out.println(member.getMemberId() + " - " + member.getName() + 
                             ": $" + String.format("%.2f", member.getFineAmount()));
        }
    }
    
    public void payFine() {
        System.out.print("\nEnter Member ID: ");
        scanner.nextLine(); // consume newline
        String memberId = scanner.nextLine();
        
        Member member = members.get(memberId);
        if (member == null) {
            System.out.println("Member not found!");
            return;
        }
        
        if (member.getFineAmount() <= 0) {
            System.out.println("Member has no outstanding fines!");
            return;
        }
        
        System.out.println("Current fine amount: $" + String.format("%.2f", member.getFineAmount()));
        System.out.print("Enter payment amount: $");
        double paymentAmount = scanner.nextDouble();
        
        if (member.payFine(paymentAmount)) {
            System.out.println("Payment of $" + String.format("%.2f", paymentAmount) + " processed successfully!");
            System.out.println("Remaining balance: $" + String.format("%.2f", member.getFineAmount()));
        } else {
            System.out.println("Invalid payment amount!");
        }
    }
    
    // File operations
    public void saveDataToFiles() {
        // Save books
        try (PrintWriter writer = new PrintWriter(new FileWriter(BOOKS_FILE))) {
            for (Book book : books.values()) {
                writer.println(book.getIsbn() + "," + 
                             book.getTitle() + "," + 
                             book.getAuthor() + "," + 
                             book.getGenre() + "," + 
                             book.getPublicationYear() + "," + 
                             book.isAvailable() + "," + 
                             (book.getBorrowedBy() != null ? book.getBorrowedBy() : "") + "," + 
                             (book.getDueDate() != null ? book.getDueDate().toString() : ""));
            }
        } catch (IOException e) {
            System.out.println("Error saving books: " + e.getMessage());
        }
        
        // Save members
        try (PrintWriter writer = new PrintWriter(new FileWriter(MEMBERS_FILE))) {
            for (Member member : members.values()) {
                writer.println(member.getMemberId() + "," + 
                             member.getName() + "," + 
                             member.getEmail() + "," + 
                             member.getPhone() + "," + 
                             member.getMembershipDate().toString() + "," + 
                             member.getFineAmount() + "," + 
                             member.isActive() + "," + 
                             String.join(";", member.getBorrowedBooks()));
            }
        } catch (IOException e) {
            System.out.println("Error saving members: " + e.getMessage());
        }
    }
    
    public void loadDataFromFiles() {
        // Load books
        try (BufferedReader reader = new BufferedReader(new FileReader(BOOKS_FILE))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] parts = line.split(",");
                if (parts.length >= 5) {
                    String isbn = parts[0];
                    String title = parts[1];
                    String author = parts[2];
                    String genre = parts[3];
                    int year = Integer.parseInt(parts[4]);
                    boolean isAvailable = Boolean.parseBoolean(parts[5]);
                    
                    Book book = new Book(isbn, title, author, genre, year);
                    
                    if (!isAvailable && parts.length >= 8) {
                        String borrowedBy = parts[6];
                        if (!borrowedBy.isEmpty()) {
                            LocalDate dueDate = LocalDate.parse(parts[7]);
                            book.borrowBook(borrowedBy, 0); // Temporary borrow
                        }
                    }
                    
                    books.put(isbn, book);
                }
            }
        } catch (FileNotFoundException e) {
            // File doesn't exist, start with empty library
        } catch (IOException e) {
            System.out.println("Error loading books: " + e.getMessage());
        }
        
        // Load members
        try (BufferedReader reader = new BufferedReader(new FileReader(MEMBERS_FILE))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] parts = line.split(",");
                if (parts.length >= 7) {
                    String memberId = parts[0];
                    String name = parts[1];
                    String email = parts[2];
                    String phone = parts[3];
                    double fineAmount = Double.parseDouble(parts[5]);
                    boolean isActive = Boolean.parseBoolean(parts[6]);
                    
                    Member member = new Member(memberId, name, email, phone);
                    member.setFineAmount(fineAmount);
                    member.setActive(isActive);
                    
                    // Load borrowed books
                    if (parts.length > 7 && !parts[7].isEmpty()) {
                        String[] borrowedBooks = parts[7].split(";");
                        for (String isbn : borrowedBooks) {
                            member.addBorrowedBook(isbn);
                        }
                    }
                    
                    members.put(memberId, member);
                }
            }
        } catch (FileNotFoundException e) {
            // File doesn't exist, start with empty members
        } catch (IOException e) {
            System.out.println("Error loading members: " + e.getMessage());
        }
    }
    
    // Generate reports
    public void generateReports() {
        System.out.println("\n=== Library Report ===");
        
        // Basic statistics
        int totalBooks = books.size();
        int availableBooks = 0;
        int borrowedBooks = 0;
        int overdueBooks = 0;
        
        for (Book book : books.values()) {
            if (book.isAvailable()) {
                availableBooks++;
            } else {
                borrowedBooks++;
                if (book.isOverdue()) {
                    overdueBooks++;
                }
            }
        }
        
        int totalMembers = members.size();
        int activeMembers = 0;
        double totalFines = 0.0;
        
        for (Member member : members.values()) {
            if (member.isActive()) {
                activeMembers++;
            }
            totalFines += member.getFineAmount();
        }
        
        System.out.println("=== Book Statistics ===");
        System.out.println("Total Books: " + totalBooks);
        System.out.println("Available Books: " + availableBooks);
        System.out.println("Borrowed Books: " + borrowedBooks);
        System.out.println("Overdue Books: " + overdueBooks);
        
        System.out.println("\n=== Member Statistics ===");
        System.out.println("Total Members: " + totalMembers);
        System.out.println("Active Members: " + activeMembers);
        System.out.println("Inactive Members: " + (totalMembers - activeMembers));
        
        System.out.println("\n=== Financial Statistics ===");
        System.out.println("Total Outstanding Fines: $" + String.format("%.2f", totalFines));
        
        // Genre distribution
        Map<String, Integer> genreCount = new HashMap<>();
        for (Book book : books.values()) {
            genreCount.merge(book.getGenre(), 1, Integer::sum);
        }
        
        System.out.println("\n=== Genre Distribution ===");
        for (Map.Entry<String, Integer> entry : genreCount.entrySet()) {
            System.out.println(entry.getKey() + ": " + entry.getValue());
        }
    }
    
    // Main execution method
    public void run() {
        System.out.println("=== LIBRARY MANAGEMENT SYSTEM ===");
        System.out.println("Welcome to the Library!");
        
        while (true) {
            displayMainMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: handleBookManagement(); break;
                    case 2: handleMemberManagement(); break;
                    case 3: handleBorrowing(); break;
                    case 4: handleSearchAndReports(); break;
                    case 5: manageFines(); break;
                    case 6:
                        saveDataToFiles();
                        System.out.println("Data saved successfully!");
                        System.out.println("Thank you for using Library Management System!");
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
    
    // Handle book management operations
    private void handleBookManagement() {
        while (true) {
            displayBookMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: addBook(); break;
                    case 2: viewAllBooks(); break;
                    case 3: searchBooks(); break;
                    case 4: /* updateBook(); */ break;
                    case 5: /* removeBook(); */ break;
                    case 6: return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine();
            }
        }
    }
    
    // Handle member management operations
    private void handleMemberManagement() {
        while (true) {
            displayMemberMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: addMember(); break;
                    case 2: viewAllMembers(); break;
                    case 3: /* searchMembers(); */ break;
                    case 4: /* updateMember(); */ break;
                    case 5: /* toggleMemberStatus(); */ break;
                    case 6: return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine();
            }
        }
    }
    
    // Handle borrowing operations
    private void handleBorrowing() {
        while (true) {
            displayBorrowingMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: borrowBook(); break;
                    case 2: returnBook(); break;
                    case 3: /* extendLoan(); */ break;
                    case 4: /* viewMemberBooks(); */ break;
                    case 5: viewOverdueBooks(); break;
                    case 6: return;
                    default:
                        System.out.println("Invalid choice. Please try again.");
                }
            } catch (InputMismatchException e) {
                System.out.println("Invalid input. Please enter a number.");
                scanner.nextLine();
            }
        }
    }
    
    // Handle search and reports
    private void handleSearchAndReports() {
        System.out.println("\n=== Search & Reports ===");
        System.out.println("1. Search Books");
        System.out.println("2. Generate Library Report");
        System.out.println("3. Back to Main Menu");
        System.out.print("Choose an option (1-3): ");
        
        try {
            int choice = scanner.nextInt();
            
            switch (choice) {
                case 1: searchBooks(); break;
                case 2: generateReports(); break;
                case 3: return;
                default:
                    System.out.println("Invalid choice.");
            }
        } catch (InputMismatchException e) {
            System.out.println("Invalid input. Please enter a number.");
            scanner.nextLine();
        }
    }
    
    // Main method
    public static void main(String[] args) {
        LibraryManagementSystem library = new LibraryManagementSystem();
        library.run();
    }
}
