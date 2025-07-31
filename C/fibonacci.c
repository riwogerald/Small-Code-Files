// fibonacci.c
#include <stdio.h>

void printFibonacci(int n) {
    int t1 = 0, t2 = 1, nextTerm;
    printf("Fibonacci Series: %d, %d", t1, t2);

    for (int i = 2; i < n; i++) {
        nextTerm = t1 + t2;
        t1 = t2;
        t2 = nextTerm;
        printf(", %d", nextTerm);
    }
    printf("\n");
}

int main() {
    printFibonacci(10);
    return 0;
}
