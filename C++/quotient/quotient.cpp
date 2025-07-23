#include <iostream>
using namespace std;

int main() {
    int dividend, divisor;
    
    // Read input from user
    cout << "Enter the first number (dividend): ";
    cin >> dividend;
    
    cout << "Enter the second number (divisor): ";
    cin >> divisor;
    
    // Check for division by zero
    if (divisor == 0) {
        cout << "Error: Division by zero is not allowed!" << endl;
        return 1;
    }
    
    // Compute quotient and remainder
    int quotient = dividend / divisor;
    int remainder = dividend % divisor;
    
    // Display results
    cout << "\n--- Results ---" << endl;
    cout << "Dividend: " << dividend << endl;
    cout << "Divisor: " << divisor << endl;
    cout << "Quotient: " << quotient << endl;
    cout << "Remainder: " << remainder << endl;
    cout << "\nVerification: " << dividend << " = " << divisor 
         << " × " << quotient << " + " << remainder << endl;
    
    return 0;
}