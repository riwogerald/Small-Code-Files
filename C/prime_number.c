// prime_number.c
#include <stdio.h>
#include <math.h>

int isPrime(int num) {
    if (num <= 1) return 0;
    if (num <= 3) return 1;
    if (num % 2 == 0 || num % 3 == 0) return 0;

    for (int i = 5; i * i <= num; i += 6) {
        if (num % i == 0 || num % (i + 2) == 0) return 0;
    }
    return 1;
}

int main() {
    int num1 = 11;
    int num2 = 25;
    
    printf("%d is %sa prime number\n", num1, isPrime(num1) ? "" : "not ");
    printf("%d is %sa prime number\n", num2, isPrime(num2) ? "" : "not ");
    return 0;
}
