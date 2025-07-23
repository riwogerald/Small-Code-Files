#include <iostream>
#include <iomanip>
using namespace std;

int main() {
    int mark;
    int sum = 0;
    int validMarkCount = 0;
    int failingMarkCount = 0;
    
    cout << "=== MARK ANALYZER ===" << endl;
    cout << "Enter student marks (0-100)" << endl;
    cout << "Enter -1 when finished" << endl;
    cout << "Invalid marks will be ignored" << endl;
    cout << "=====================" << endl;
    
    // Sentinel-controlled loop
    while (true) {
        cout << "Enter mark " << (validMarkCount + 1) << ": ";
        cin >> mark;
        
        // Check for sentinel value
        if (mark == -1) {
            break;
        }
        
        // Validate mark range
        if (mark < 0 || mark > 100) {
            cout << "⚠️  Invalid! Mark must be between 0-100" << endl;
            continue;
        }
        
        // Process valid mark
        sum += mark;
        validMarkCount++;
        
        // Check for failing mark
        if (mark < 40) {
            failingMarkCount++;
            cout << "✓ Recorded (FAIL)" << endl;
        } else {
            cout << "✓ Recorded (PASS)" << endl;
        }
    }
    
    // Validate input
    if (validMarkCount == 0) {
        cout << "\n❌ No valid marks entered!" << endl;
        return 1;
    }
    
    // Calculate statistics
    double average = static_cast<double>(sum) / validMarkCount;
    double failureRate = static_cast<double>(failingMarkCount) / validMarkCount * 100;
    
    // Display results
    cout << "\n📊 STATISTICS" << endl;
    cout << "=============" << endl;
    cout << "Students: " << validMarkCount << endl;
    cout << "Average: " << fixed << setprecision(2) << average << endl;
    cout << "Failures: " << failingMarkCount << " (" << setprecision(1) 
         << failureRate << "%)" << endl;
    
    // Analysis and recommendations
    cout << "\n📝 ANALYSIS" << endl;
    cout << "===========" << endl;
    
    if (average < 45) {
        cout << "📚 The lecturer should be trained on teaching methodology" << endl;
    }
    
    if (failingMarkCount > validMarkCount / 2.0) {
        cout << "💥 The lecturer sucks!" << endl;
    }
    
    if (average >= 70) {
        cout << "🎉 Excellent class performance!" << endl;
    } else if (average >= 45 && failingMarkCount <= validMarkCount / 2.0) {
        cout << "👍 Satisfactory teaching performance!" << endl;
    }
    
    return 0;
}