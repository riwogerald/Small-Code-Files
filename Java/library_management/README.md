# Library Management System

## Overview

A comprehensive console-based Library Management System built in Java that handles all aspects of library operations including book cataloging, member management, borrowing/returning processes, and fine management. This system provides a complete solution for small to medium-sized libraries with data persistence and detailed reporting capabilities.

## Features

### Book Management
- **Add New Books**: Catalog books with ISBN, title, author, genre, and publication year
- **View All Books**: Display complete book inventory separated by availability status
- **Search Books**: Multi-criteria search by title, author, genre, or ISBN
- **Book Status Tracking**: Monitor available vs. borrowed books
- **Overdue Detection**: Automatic identification of overdue books
- **Loan Extension**: Extend borrowing periods for existing loans

### Member Management
- **Member Registration**: Add new library members with contact information
- **Member Profiles**: Track member details, borrowing history, and fines
- **Active/Inactive Status**: Manage member account status
- **Borrowing Limits**: Enforce maximum books per member (5 books)
- **Fine Tracking**: Monitor outstanding fines and payment history

### Borrowing & Returning System
- **Book Borrowing**: Process book loans with customizable loan periods
- **Book Returns**: Handle returns with automatic fine calculation
- **Due Date Management**: Track and enforce return dates
- **Overdue Processing**: Calculate fines for late returns ($1/day)
- **Transaction History**: Complete audit trail of all borrowing activities

### Fine Management
- **Automatic Fine Calculation**: $1 per day for overdue books
- **Fine Payment Processing**: Handle partial and full fine payments
- **Fine Waiving**: Administrative fine forgiveness options
- **Outstanding Fine Reports**: Track all unpaid fines

### Advanced Features
- **Comprehensive Reports**: Library statistics and analytics
- **Genre Distribution**: Track book collection by genre
- **Member Analytics**: Active vs. inactive member statistics
- **Financial Reporting**: Total outstanding fines and collection data
- **Search & Filter**: Multiple search options for books and members

## Technical Specifications

### Classes

#### Book Class
```java
Book {
    - String isbn (unique identifier)
    - String title
    - String author
    - String genre
    - int publicationYear
    - boolean isAvailable
    - LocalDate dueDate
    - String borrowedBy (member ID)
}
```

#### Member Class
```java
Member {
    - String memberId (unique identifier)
    - String name
    - String email
    - String phone
    - LocalDate membershipDate
    - List<String> borrowedBooks
    - double fineAmount
    - boolean isActive
}
```

#### LibraryManagementSystem Class
- Main application controller
- Handles all library operations
- Manages data persistence
- Provides comprehensive reporting

### Key Methods

#### Book Operations
- `addBook()`: Add new books to catalog
- `viewAllBooks()`: Display complete inventory
- `searchBooks()`: Multi-criteria book search
- `borrowBook()`: Process book loans
- `returnBook()`: Handle book returns
- `viewOverdueBooks()`: Show overdue items

#### Member Operations
- `addMember()`: Register new members
- `viewAllMembers()`: Display member list
- `searchMembers()`: Find members by criteria
- `updateMember()`: Modify member information

#### Administrative Functions
- `generateReports()`: Create comprehensive statistics
- `manageFines()`: Handle fine operations
- `saveDataToFiles()`: Persist data to files
- `loadDataFromFiles()`: Load existing data

## Usage Instructions

### Running the Program
1. Compile: `javac LibraryManagementSystem.java`
2. Run: `java LibraryManagementSystem`

### Main Menu Navigation
1. **Book Management** - Catalog and book operations
2. **Member Management** - Member registration and management
3. **Borrowing & Returning** - Loan processing
4. **Search & Reports** - Search and analytics
5. **Fine Management** - Fine processing and payment
6. **Save & Exit** - Save data and quit

### Book Management Submenu
1. **Add New Book** - Catalog new books
2. **View All Books** - Display inventory
3. **Search Books** - Find books by criteria
4. **Update Book Information** - Modify book details
5. **Remove Book** - Delete books from catalog

### Member Management Submenu
1. **Add New Member** - Register new members
2. **View All Members** - Display member list
3. **Search Members** - Find members
4. **Update Member Information** - Modify member details
5. **Deactivate/Activate Member** - Manage member status

### Borrowing Operations
1. **Borrow Book** - Process new loans
2. **Return Book** - Handle book returns
3. **Extend Loan** - Extend borrowing period
4. **View Member's Borrowed Books** - Check member loans
5. **View Overdue Books** - Show overdue items

## Data Persistence

### File Storage
- **books.txt**: Book catalog information
- **members.txt**: Member information and borrowing history
- **Automatic Save**: Data saved on program exit
- **Automatic Load**: Previous data loaded on startup

### Data Formats

#### Books File Format
```
ISBN,Title,Author,Genre,PublicationYear,IsAvailable,BorrowedBy,DueDate
978-0-123456-78-9,Java Programming,John Smith,Technology,2023,false,M001,2024-02-15
```

#### Members File Format
```
MemberID,Name,Email,Phone,MembershipDate,FineAmount,IsActive,BorrowedBooks
M001,Jane Doe,jane@email.com,555-1234,2024-01-01,5.00,true,978-0-123456-78-9
```

## Sample Usage Scenarios

### Adding a New Book
```
=== Add New Book ===
Enter ISBN: 978-0-123456-78-9
Enter Title: Java Programming Guide
Enter Author: John Smith
Enter Genre: Technology
Enter Publication Year: 2023
Book added successfully!
```

### Borrowing a Book
```
=== Borrow Book ===
Enter Member ID: M001
Enter Book ISBN: 978-0-123456-78-9
Enter loan period (days) [default: 14]: 14
Book borrowed successfully!
Due date: 2024-02-15
```

### Processing a Return
```
=== Return Book ===
Enter Book ISBN: 978-0-123456-78-9
Book is overdue by 3 days.
Fine of $3.00 has been added to member's account.
Book returned successfully!
Member M001 now owes $3.00 in fines.
```

## Sample Reports

### Library Statistics
```
=== Library Report ===

=== Book Statistics ===
Total Books: 1,250
Available Books: 1,100
Borrowed Books: 150
Overdue Books: 25

=== Member Statistics ===
Total Members: 500
Active Members: 485
Inactive Members: 15

=== Financial Statistics ===
Total Outstanding Fines: $450.00

=== Genre Distribution ===
Fiction: 320
Technology: 180
Science: 150
History: 120
Biography: 95
Other: 385
```

### Overdue Books Report
```
=== Overdue Books ===
Total overdue books: 3

ISBN: 978-0-123456-78-9 | Java Programming by John Smith | Genre: Technology | Year: 2023 | Status: Borrowed by M001 (Due: 2024-01-20)
  Borrower: Jane Doe (M001)
  Days overdue: 5
  Potential fine: $5.00
```

## Business Rules

### Borrowing Rules
- Maximum 5 books per member
- Default loan period: 14 days
- Customizable loan periods available
- No borrowing with outstanding fines
- Active membership required

### Fine System
- $1.00 per day for overdue books
- Fines calculated on return
- Payment required before new borrowing
- Administrative fine waiving available

### Member Management
- Unique member IDs required
- Contact information mandatory
- Automatic membership date recording
- Status tracking (active/inactive)

## File Structure
```
library_management/
├── LibraryManagementSystem.java    # Main application
├── books.txt                       # Book catalog (auto-generated)
├── members.txt                     # Member data (auto-generated)
└── README.md                      # This documentation
```

## Key Benefits

### For Librarians
- **Efficient Operations**: Streamlined book and member management
- **Automated Processing**: Automatic fine calculation and overdue tracking
- **Comprehensive Reporting**: Detailed analytics and statistics
- **Data Persistence**: No data loss between sessions
- **Search Capabilities**: Quick access to books and member information

### For Library Management
- **Financial Tracking**: Monitor fines and collection statistics
- **Usage Analytics**: Track borrowing patterns and popular genres
- **Member Management**: Comprehensive member tracking and communication
- **Inventory Control**: Complete book catalog with status tracking

### Technical Benefits
- **Object-Oriented Design**: Clean, maintainable code structure
- **Data Integrity**: Robust validation and error handling
- **Scalability**: Handles growing collections and membership
- **User-Friendly Interface**: Intuitive menu-driven operation

## Educational Value

This system demonstrates:
- **Advanced OOP Concepts**: Multiple classes with complex relationships
- **Date/Time Handling**: LocalDate for due dates and membership tracking
- **Collections Framework**: Lists and Maps for data management
- **File I/O Operations**: Reading and writing structured data
- **Business Logic Implementation**: Real-world library operations
- **Data Validation**: Input validation and error handling
- **Reporting Systems**: Data analysis and presentation
- **Menu-Driven Interface**: Complex navigation systems
- **Financial Calculations**: Fine calculation and payment processing
- **Search Algorithms**: Multi-criteria search implementation

## System Requirements
- Java 8 or higher (for LocalDate support)
- Console/Terminal environment
- File system access for data persistence
- Sufficient memory for book and member collections

This Library Management System provides a complete foundation for understanding complex business applications while demonstrating advanced Java programming concepts and best practices.
