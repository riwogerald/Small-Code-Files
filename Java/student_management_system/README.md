# Student Management System

## Overview

A comprehensive console-based Student Management System built in Java that allows educational institutions to manage student records, track academic performance, and generate reports. This system provides full CRUD (Create, Read, Update, Delete) operations for student data with persistent file storage.

## Features

### Student Management
- **Add New Students**: Register students with personal and academic information
- **View All Students**: Display complete list of enrolled students
- **Search Students**: Find students by ID or name (partial matching supported)
- **Update Student Information**: Modify student details (name, email, GPA, major)
- **Delete Students**: Remove student records with confirmation

### Academic Tracking
- **GPA Management**: Track student Grade Point Averages (0.0-4.0 scale)
- **Academic Standing**: Automatic calculation based on GPA:
  - Summa Cum Laude (3.8+)
  - Magna Cum Laude (3.5-3.7)
  - Cum Laude (3.2-3.4)
  - Good Standing (2.0-3.1)
  - Academic Probation (<2.0)
- **Major Tracking**: Organize students by their field of study

### Course Management
- **Course Enrollment**: Add courses to student records
- **Course Removal**: Remove courses from student enrollment
- **Course History**: Track all courses per student

### Advanced Features
- **Filter by Major**: View all students in a specific major
- **GPA Range Filtering**: Find students within specific GPA ranges
- **Statistics Report**: Comprehensive analytics including:
  - Total student count
  - Average, highest, and lowest GPA
  - GPA distribution analysis
  - Major distribution statistics

### Data Persistence
- **File Storage**: All data saved to `students.txt`
- **Auto-load**: Previous data automatically loaded on program start
- **CSV Format**: Human-readable comma-separated values

## Technical Specifications

### Classes
- **Student**: Core student data model with all attributes and methods
- **StudentManagementSystem**: Main application class with all operations

### Key Methods
- `addStudent()`: Register new students with validation
- `viewAllStudents()`: Display all students sorted by ID
- `searchStudentById()`: Find specific student by ID
- `searchStudentsByName()`: Search by name with partial matching
- `updateStudent()`: Modify student information
- `deleteStudent()`: Remove student with confirmation
- `viewStudentsByMajor()`: Filter students by major
- `viewStudentsByGpaRange()`: Filter by GPA range
- `addCourseToStudent()`: Enroll student in course
- `removeCourseFromStudent()`: Remove course enrollment
- `generateStatisticsReport()`: Generate comprehensive analytics

### Data Model
```java
Student {
    - int studentId (unique identifier)
    - String firstName
    - String lastName  
    - String email
    - double gpa (0.0-4.0)
    - String major
    - List<String> courses
}
```

## Usage Instructions

### Running the Program
1. Compile: `javac StudentManagementSystem.java`
2. Run: `java StudentManagementSystem`

### Menu Navigation
The system uses a numbered menu system:
1. **Add New Student** - Register a new student
2. **View All Students** - See complete student list
3. **Search Student by ID** - Find specific student
4. **Search Students by Name** - Name-based search
5. **Update Student Information** - Modify student data
6. **Delete Student** - Remove student record
7. **View Students by Major** - Filter by academic program
8. **View Students by GPA Range** - Filter by academic performance
9. **Add Course to Student** - Enroll in course
10. **Remove Course from Student** - Drop course
11. **Generate Statistics Report** - View analytics
12. **Save and Exit** - Save data and quit

### Input Validation
- **Student ID**: Must be unique across all students
- **GPA**: Must be between 0.0 and 4.0
- **Email/Phone**: Basic format validation
- **Duplicate Prevention**: System prevents duplicate student IDs

### Sample Data Entry
```
Student ID: 12345
First Name: John
Last Name: Doe
Email: john.doe@university.edu
GPA: 3.75
Major: Computer Science
```

## File Structure
```
student_management_system/
├── StudentManagementSystem.java    # Main application
├── students.txt                    # Data storage (auto-generated)
└── README.md                      # This documentation
```

## Sample Output
```
=== Student Management System ===
Total Students: 150

ID: 12345 | John Doe | Email: john.doe@university.edu | Major: Computer Science | GPA: 3.75 | Standing: Cum Laude | Courses: [Java Programming, Data Structures]

=== Statistics Report ===
Total Students: 150
Average GPA: 3.24
Highest GPA: 4.00
Lowest GPA: 1.85

GPA Distribution:
Excellent (3.5+): 45
Good (3.0-3.4): 62
Average (2.5-2.9): 33
Below Average (<2.5): 10

Major Distribution:
Computer Science: 45
Business Administration: 38
Engineering: 32
Liberal Arts: 35
```

## Key Benefits
- **Easy to Use**: Intuitive menu-driven interface
- **Data Persistence**: No data loss between sessions
- **Comprehensive Analytics**: Detailed reporting capabilities
- **Flexible Search**: Multiple search options
- **Academic Focus**: Built specifically for educational institutions
- **Scalable**: Can handle hundreds of student records
- **Error Handling**: Robust input validation and error management

## Educational Value
This system demonstrates:
- Object-oriented programming principles
- File I/O operations in Java
- Data structures (Lists, Maps)
- Input validation and error handling
- Menu-driven program design
- CRUD operations implementation
- Statistical data processing
- Real-world application development
