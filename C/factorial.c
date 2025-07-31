// factorial.c
#include <stdio.h>

int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

int main() {
    int num = 5;
    printf("Factorial of %d: %d\n", num, factorial(num));
    return 0;
}
