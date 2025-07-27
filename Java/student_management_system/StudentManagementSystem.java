import java.util.*;
import java.io.*;

// Student class to represent individual students
class Student {
    private int studentId;
    private String firstName;
    private String lastName;
    private String email;
    private double gpa;
    private String major;
    private List<String> courses;
    
    // Constructor
    public Student(int studentId, String firstName, String lastName, String email, double gpa, String major) {
        this.studentId = studentId;
        this.firstName = firstName;
        this.lastName = lastName;
        this.email = email;
        this.gpa = gpa;
        this.major = major;
        this.courses = new ArrayList<>();
    }
    
    // Getters and setters
    public int getStudentId() { return studentId; }
    public void setStudentId(int studentId) { this.studentId = studentId; }
    
    public String getFirstName() { return firstName; }
    public void setFirstName(String firstName) { this.firstName = firstName; }
    
    public String getLastName() { return lastName; }
    public void setLastName(String lastName) { this.lastName = lastName; }
    
    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }
    
    public double getGpa() { return gpa; }
    public void setGpa(double gpa) { this.gpa = gpa; }
    
    public String getMajor() { return major; }
    public void setMajor(String major) { this.major = major; }
    
    public List<String> getCourses() { return courses; }
    public void setCourses(List<String> courses) { this.courses = courses; }
    
    // Method to add a course
    public void addCourse(String course) {
        if (!courses.contains(course)) {
            courses.add(course);
        }
    }
    
    // Method to remove a course
    public boolean removeCourse(String course) {
        return courses.remove(course);
    }
    
    // Method to get full name
    public String getFullName() {
        return firstName + " " + lastName;
    }
    
    // Method to get academic standing based on GPA
    public String getAcademicStanding() {
        if (gpa >= 3.8) return "Summa Cum Laude";
        else if (gpa >= 3.5) return "Magna Cum Laude";
        else if (gpa >= 3.2) return "Cum Laude";
        else if (gpa >= 2.0) return "Good Standing";
        else return "Academic Probation";
    }
    
    @Override
    public String toString() {
        return String.format("ID: %d | %s | Email: %s | Major: %s | GPA: %.2f | Standing: %s | Courses: %s",
                studentId, getFullName(), email, major, gpa, getAcademicStanding(), courses);
    }
    
    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        Student student = (Student) obj;
        return studentId == student.studentId;
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(studentId);
    }
}

// Main StudentManagementSystem class
public class StudentManagementSystem {
    private List<Student> students;
    private Scanner scanner;
    private final String DATA_FILE = "students.txt";
    
    public StudentManagementSystem() {
        students = new ArrayList<>();
        scanner = new Scanner(System.in);
        loadStudentsFromFile();
    }
    
    // Main menu method
    public void displayMenu() {
        System.out.println("\n=== Student Management System ===");
        System.out.println("1. Add New Student");
        System.out.println("2. View All Students");
        System.out.println("3. Search Student by ID");
        System.out.println("4. Search Students by Name");
        System.out.println("5. Update Student Information");
        System.out.println("6. Delete Student");
        System.out.println("7. View Students by Major");
        System.out.println("8. View Students by GPA Range");
        System.out.println("9. Add Course to Student");
        System.out.println("10. Remove Course from Student");
        System.out.println("11. Generate Statistics Report");
        System.out.println("12. Save and Exit");
        System.out.print("Choose an option (1-12): ");
    }
    
    // Method to add a new student
    public void addStudent() {
        System.out.println("\n=== Add New Student ===");
        
        System.out.print("Enter Student ID: ");
        int id = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        // Check if student ID already exists
        if (findStudentById(id) != null) {
            System.out.println("Error: Student with ID " + id + " already exists!");
            return;
        }
        
        System.out.print("Enter First Name: ");
        String firstName = scanner.nextLine();
        
        System.out.print("Enter Last Name: ");
        String lastName = scanner.nextLine();
        
        System.out.print("Enter Email: ");
        String email = scanner.nextLine();
        
        System.out.print("Enter GPA (0.0-4.0): ");
        double gpa = scanner.nextDouble();
        scanner.nextLine(); // consume newline
        
        if (gpa < 0.0 || gpa > 4.0) {
            System.out.println("Error: GPA must be between 0.0 and 4.0");
            return;
        }
        
        System.out.print("Enter Major: ");
        String major = scanner.nextLine();
        
        Student student = new Student(id, firstName, lastName, email, gpa, major);
        students.add(student);
        
        System.out.println("Student added successfully!");
        System.out.println(student);
    }
    
    // Method to view all students
    public void viewAllStudents() {
        System.out.println("\n=== All Students ===");
        if (students.isEmpty()) {
            System.out.println("No students found in the system.");
            return;
        }
        
        // Sort students by ID for better display
        students.sort((s1, s2) -> Integer.compare(s1.getStudentId(), s2.getStudentId()));
        
        for (Student student : students) {
            System.out.println(student);
        }
        System.out.println("\nTotal Students: " + students.size());
    }
    
    // Method to find student by ID
    public Student findStudentById(int id) {
        return students.stream()
                .filter(student -> student.getStudentId() == id)
                .findFirst()
                .orElse(null);
    }
    
    // Method to search student by ID
    public void searchStudentById() {
        System.out.print("\nEnter Student ID to search: ");
        int id = scanner.nextInt();
        
        Student student = findStudentById(id);
        if (student != null) {
            System.out.println("\nStudent Found:");
            System.out.println(student);
        } else {
            System.out.println("Student with ID " + id + " not found.");
        }
    }
    
    // Method to search students by name
    public void searchStudentsByName() {
        System.out.print("\nEnter name to search (first or last): ");
        scanner.nextLine(); // consume newline
        String name = scanner.nextLine().toLowerCase();
        
        List<Student> foundStudents = students.stream()
                .filter(student -> student.getFirstName().toLowerCase().contains(name) ||
                                 student.getLastName().toLowerCase().contains(name))
                .collect(ArrayList::new, (list, student) -> list.add(student), ArrayList::addAll);
        
        if (foundStudents.isEmpty()) {
            System.out.println("No students found with name containing: " + name);
        } else {
            System.out.println("\nStudents found:");
            foundStudents.forEach(System.out::println);
        }
    }
    
    // Method to update student information
    public void updateStudent() {
        System.out.print("\nEnter Student ID to update: ");
        int id = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        Student student = findStudentById(id);
        if (student == null) {
            System.out.println("Student with ID " + id + " not found.");
            return;
        }
        
        System.out.println("Current information:");
        System.out.println(student);
        
        System.out.println("\nWhat would you like to update?");
        System.out.println("1. First Name");
        System.out.println("2. Last Name");
        System.out.println("3. Email");
        System.out.println("4. GPA");
        System.out.println("5. Major");
        System.out.print("Choose option (1-5): ");
        
        int choice = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        switch (choice) {
            case 1:
                System.out.print("Enter new First Name: ");
                student.setFirstName(scanner.nextLine());
                break;
            case 2:
                System.out.print("Enter new Last Name: ");
                student.setLastName(scanner.nextLine());
                break;
            case 3:
                System.out.print("Enter new Email: ");
                student.setEmail(scanner.nextLine());
                break;
            case 4:
                System.out.print("Enter new GPA: ");
                double newGpa = scanner.nextDouble();
                if (newGpa >= 0.0 && newGpa <= 4.0) {
                    student.setGpa(newGpa);
                } else {
                    System.out.println("Invalid GPA. Must be between 0.0 and 4.0");
                    return;
                }
                break;
            case 5:
                System.out.print("Enter new Major: ");
                student.setMajor(scanner.nextLine());
                break;
            default:
                System.out.println("Invalid choice.");
                return;
        }
        
        System.out.println("Student updated successfully!");
        System.out.println(student);
    }
    
    // Method to delete student
    public void deleteStudent() {
        System.out.print("\nEnter Student ID to delete: ");
        int id = scanner.nextInt();
        
        Student student = findStudentById(id);
        if (student == null) {
            System.out.println("Student with ID " + id + " not found.");
            return;
        }
        
        System.out.println("Student to be deleted:");
        System.out.println(student);
        System.out.print("Are you sure you want to delete this student? (y/n): ");
        scanner.nextLine(); // consume newline
        String confirmation = scanner.nextLine();
        
        if (confirmation.toLowerCase().equals("y")) {
            students.remove(student);
            System.out.println("Student deleted successfully!");
        } else {
            System.out.println("Deletion cancelled.");
        }
    }
    
    // Method to view students by major
    public void viewStudentsByMajor() {
        System.out.print("\nEnter major to filter by: ");
        scanner.nextLine(); // consume newline
        String major = scanner.nextLine();
        
        List<Student> studentsInMajor = students.stream()
                .filter(student -> student.getMajor().equalsIgnoreCase(major))
                .collect(ArrayList::new, (list, student) -> list.add(student), ArrayList::addAll);
        
        if (studentsInMajor.isEmpty()) {
            System.out.println("No students found in major: " + major);
        } else {
            System.out.println("\nStudents in " + major + ":");
            studentsInMajor.forEach(System.out::println);
            System.out.println("Total: " + studentsInMajor.size());
        }
    }
    
    // Method to view students by GPA range
    public void viewStudentsByGpaRange() {
        System.out.print("\nEnter minimum GPA: ");
        double minGpa = scanner.nextDouble();
        System.out.print("Enter maximum GPA: ");
        double maxGpa = scanner.nextDouble();
        
        List<Student> studentsInRange = students.stream()
                .filter(student -> student.getGpa() >= minGpa && student.getGpa() <= maxGpa)
                .collect(ArrayList::new, (list, student) -> list.add(student), ArrayList::addAll);
        
        if (studentsInRange.isEmpty()) {
            System.out.println("No students found in GPA range " + minGpa + " - " + maxGpa);
        } else {
            System.out.println("\nStudents with GPA between " + minGpa + " and " + maxGpa + ":");
            studentsInRange.sort((s1, s2) -> Double.compare(s2.getGpa(), s1.getGpa())); // Sort by GPA descending
            studentsInRange.forEach(System.out::println);
            System.out.println("Total: " + studentsInRange.size());
        }
    }
    
    // Method to add course to student
    public void addCourseToStudent() {
        System.out.print("\nEnter Student ID: ");
        int id = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        Student student = findStudentById(id);
        if (student == null) {
            System.out.println("Student with ID " + id + " not found.");
            return;
        }
        
        System.out.print("Enter course name to add: ");
        String course = scanner.nextLine();
        
        student.addCourse(course);
        System.out.println("Course added successfully!");
        System.out.println("Updated student info:");
        System.out.println(student);
    }
    
    // Method to remove course from student
    public void removeCourseFromStudent() {
        System.out.print("\nEnter Student ID: ");
        int id = scanner.nextInt();
        scanner.nextLine(); // consume newline
        
        Student student = findStudentById(id);
        if (student == null) {
            System.out.println("Student with ID " + id + " not found.");
            return;
        }
        
        if (student.getCourses().isEmpty()) {
            System.out.println("This student has no courses enrolled.");
            return;
        }
        
        System.out.println("Current courses: " + student.getCourses());
        System.out.print("Enter course name to remove: ");
        String course = scanner.nextLine();
        
        if (student.removeCourse(course)) {
            System.out.println("Course removed successfully!");
        } else {
            System.out.println("Course not found in student's enrollment.");
        }
        
        System.out.println("Updated student info:");
        System.out.println(student);
    }
    
    // Method to generate statistics report
    public void generateStatisticsReport() {
        System.out.println("\n=== Statistics Report ===");
        
        if (students.isEmpty()) {
            System.out.println("No students in the system.");
            return;
        }
        
        // Basic statistics
        int totalStudents = students.size();
        double averageGpa = students.stream().mapToDouble(Student::getGpa).average().orElse(0.0);
        double highestGpa = students.stream().mapToDouble(Student::getGpa).max().orElse(0.0);
        double lowestGpa = students.stream().mapToDouble(Student::getGpa).min().orElse(0.0);
        
        System.out.println("Total Students: " + totalStudents);
        System.out.println("Average GPA: " + String.format("%.2f", averageGpa));
        System.out.println("Highest GPA: " + String.format("%.2f", highestGpa));
        System.out.println("Lowest GPA: " + String.format("%.2f", lowestGpa));
        
        // GPA distribution
        long excellentCount = students.stream().filter(s -> s.getGpa() >= 3.5).count();
        long goodCount = students.stream().filter(s -> s.getGpa() >= 3.0 && s.getGpa() < 3.5).count();
        long averageCount = students.stream().filter(s -> s.getGpa() >= 2.5 && s.getGpa() < 3.0).count();
        long belowAverageCount = students.stream().filter(s -> s.getGpa() < 2.5).count();
        
        System.out.println("\nGPA Distribution:");
        System.out.println("Excellent (3.5+): " + excellentCount);
        System.out.println("Good (3.0-3.4): " + goodCount);
        System.out.println("Average (2.5-2.9): " + averageCount);
        System.out.println("Below Average (<2.5): " + belowAverageCount);
        
        // Major distribution
        Map<String, Long> majorCounts = students.stream()
                .collect(HashMap::new,
                        (map, student) -> map.merge(student.getMajor(), 1L, Long::sum),
                        (map1, map2) -> {
                            map2.forEach((key, value) -> map1.merge(key, value, Long::sum));
                            return map1;
                        });
        
        System.out.println("\nMajor Distribution:");
        majorCounts.forEach((major, count) -> 
                System.out.println(major + ": " + count));
    }
    
    // Method to save students to file
    public void saveStudentsToFile() {
        try (PrintWriter writer = new PrintWriter(new FileWriter(DATA_FILE))) {
            for (Student student : students) {
                writer.println(student.getStudentId() + "," + 
                             student.getFirstName() + "," + 
                             student.getLastName() + "," + 
                             student.getEmail() + "," + 
                             student.getGpa() + "," + 
                             student.getMajor() + "," + 
                             String.join(";", student.getCourses()));
            }
            System.out.println("Data saved successfully to " + DATA_FILE);
        } catch (IOException e) {
            System.out.println("Error saving data: " + e.getMessage());
        }
    }
    
    // Method to load students from file
    public void loadStudentsFromFile() {
        try (BufferedReader reader = new BufferedReader(new FileReader(DATA_FILE))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] parts = line.split(",");
                if (parts.length >= 6) {
                    int id = Integer.parseInt(parts[0]);
                    String firstName = parts[1];
                    String lastName = parts[2];
                    String email = parts[3];
                    double gpa = Double.parseDouble(parts[4]);
                    String major = parts[5];
                    
                    Student student = new Student(id, firstName, lastName, email, gpa, major);
                    
                    // Load courses if available
                    if (parts.length > 6 && !parts[6].isEmpty()) {
                        String[] courses = parts[6].split(";");
                        for (String course : courses) {
                            student.addCourse(course);
                        }
                    }
                    
                    students.add(student);
                }
            }
            System.out.println("Loaded " + students.size() + " students from " + DATA_FILE);
        } catch (FileNotFoundException e) {
            System.out.println("No existing data file found. Starting with empty database.");
        } catch (IOException e) {
            System.out.println("Error loading data: " + e.getMessage());
        }
    }
    
    // Main execution method
    public void run() {
        System.out.println("Welcome to the Student Management System!");
        
        while (true) {
            displayMenu();
            
            try {
                int choice = scanner.nextInt();
                
                switch (choice) {
                    case 1: addStudent(); break;
                    case 2: viewAllStudents(); break;
                    case 3: searchStudentById(); break;
                    case 4: searchStudentsByName(); break;
                    case 5: updateStudent(); break;
                    case 6: deleteStudent(); break;
                    case 7: viewStudentsByMajor(); break;
                    case 8: viewStudentsByGpaRange(); break;
                    case 9: addCourseToStudent(); break;
                    case 10: removeCourseFromStudent(); break;
                    case 11: generateStatisticsReport(); break;
                    case 12:
                        saveStudentsToFile();
                        System.out.println("Thank you for using the Student Management System!");
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
    
    // Main method
    public static void main(String[] args) {
        StudentManagementSystem sms = new StudentManagementSystem();
        sms.run();
    }
}
