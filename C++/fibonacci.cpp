// fibonacci.cpp
#include <iostream>
using namespace std;

void printFibonacci(int n) {
    int t1 = 0, t2 = 1, nextTerm = 0;

    cout << "Fibonacci Series: " << t1 << ", " << t2 << ", ";

    for (int i = 1; i <= n - 2; ++i) {
        nextTerm = t1 + t2;
        t1 = t2;
        t2 = nextTerm;

        cout << nextTerm << ", ";
    }
    cout << endl;
}

int main() {
    int n = 10;
    printFibonacci(n);
    return 0;
}
