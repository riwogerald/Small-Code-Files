#include <iostream>
#include <string>
#include <iomanip>
using namespace std;

class Student {
private:
    string registrationNumber;
    string name;
    static float feetobepaid;  // Static field for fixed fee
    float feesPaid;
    float balance;

public:
    // Constructor accepting registration number and name
    Student(string regNum, string studentName) {
        registrationNumber = regNum;
        name = studentName;
        feesPaid = 0.0;
        balance = feetobepaid;  // Initial balance equals total fee to be paid
    }
    
    // Getter and Setter methods for registration number
    string getRegistrationNumber() const {
        return registrationNumber;
    }
    
    void setRegistrationNumber(string regNum) {
        registrationNumber = regNum;
    }
    
    // Getter and Setter methods for name
    string getName() const {
        return name;
    }
    
    void setName(string studentName) {
        name = studentName;
    }
    
    // Getter and Setter methods for feetobepaid (static)
    static float getFeetobepaid() {
        return feetobepaid;
    }
    
    static void setFeetobepaid(float fee) {
        feetobepaid = fee;
    }
    
    // Getter methods for feesPaid and balance
    float getFeesPaid() const {
        return feesPaid;
    }
    
    float getBalance() const {
        return balance;
    }
    
    // Additional method to make a payment (optional but useful)
    void makePayment(float amount) {
        if (amount > 0 && amount <= balance) {
            feesPaid += amount;
            balance -= amount;
        }
    }
};

// Initialize static member
float Student::feetobepaid = 5000.0;  // Default fee amount

class StudentApp {
public:
    static void displayStudentInfo() {
        // Create two Student objects
        Student student1("REG001", "John Doe");
        Student student2("REG002", "Jane Smith");
        
        // Make some payments for demonstration
        student1.makePayment(2000.0);
        student2.makePayment(3500.0);
        
        // Display information in dialog box format
        cout << "========================================" << endl;
        cout << "          STUDENT INFORMATION          " << endl;
        cout << "========================================" << endl;
        cout << endl;
        
        // Student 1 information
        cout << "STUDENT 1:" << endl;
        cout << "----------" << endl;
        cout << "Name: " << student1.getName() << endl;
        cout << "Registration Number: " << student1.getRegistrationNumber() << endl;
        cout << "Total Fee: $" << fixed << setprecision(2) << Student::getFeetobepaid() << endl;
        cout << "Fees Paid: $" << fixed << setprecision(2) << student1.getFeesPaid() << endl;
        cout << "Fee Balance: $" << fixed << setprecision(2) << student1.getBalance() << endl;
        cout << endl;
        
        // Student 2 information
        cout << "STUDENT 2:" << endl;
        cout << "----------" << endl;
        cout << "Name: " << student2.getName() << endl;
        cout << "Registration Number: " << student2.getRegistrationNumber() << endl;
        cout << "Total Fee: $" << fixed << setprecision(2) << Student::getFeetobepaid() << endl;
        cout << "Fees Paid: $" << fixed << setprecision(2) << student2.getFeesPaid() << endl;
        cout << "Fee Balance: $" << fixed << setprecision(2) << student2.getBalance() << endl;
        cout << endl;
        
        cout << "========================================" << endl;
    }
};

int main() {
    // Run the StudentApp
    StudentApp::displayStudentInfo();
    
    return 0;
}