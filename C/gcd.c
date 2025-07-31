// gcd.c
#include <stdio.h>

int gcd(int a, int b) {
    return (b == 0) ? a : gcd(b, a % b);
}

int main() {
    int a = 56, b = 98;
    printf("GCD of %d and %d: %d\n", a, b, gcd(a, b));
    return 0;
}
