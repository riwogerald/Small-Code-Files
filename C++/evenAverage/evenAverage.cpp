#include <iostream>
#include <iomanip>
using namespace std;

int main() {
    int sum = 0;
    int count = 0;
    
    cout << "Computing average of even numbers from 3 to 111..." << endl;
    
    // Counting loop from 3 to 111 (inclusive)
    for (int i = 3; i <= 111; i++) {
        if (i % 2 == 0) {  // Check if number is even
            sum += i;
            count++;
        }
    }
    
    // Calculate and display results
    double average = static_cast<double>(sum) / count;
    
    cout << "\n--- Results ---" << endl;
    cout << "Range: 3 to 111 (inclusive)" << endl;
    cout << "Even numbers found: " << count << endl;
    cout << "Sum of even numbers: " << sum << endl;
    cout << "Average: " << fixed << setprecision(2) << average << endl;
    
    return 0;
}