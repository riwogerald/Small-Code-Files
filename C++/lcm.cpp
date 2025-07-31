// lcm.cpp
#include <iostream>
using namespace std;

int gcd(int a, int b) {
    return (b == 0) ? a : gcd(b, a % b);
}

int lcm(int a, int b) {
    return (a * b) / gcd(a, b);
}

int main() {
    int a = 12, b = 15;
    cout << "LCM of " << a << " and " << b << " is " << lcm(a, b) << endl;
    return 0;
}
