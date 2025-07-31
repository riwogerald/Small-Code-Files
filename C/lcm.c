// lcm.c
#include <stdio.h>

int gcd(int a, int b) {
    return (b == 0) ? a : gcd(b, a % b);
}

int lcm(int a, int b) {
    return (a * b) / gcd(a, b);
}

int main() {
    int a = 12, b = 15;
    printf("LCM of %d and %d: %d\n", a, b, lcm(a, b));
    return 0;
}
