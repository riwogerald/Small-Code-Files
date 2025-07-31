// palindrome.c
#include <stdio.h>
#include <string.h>
#include <ctype.h>

int isPalindrome(char str[]) {
    int l = 0;
    int h = strlen(str) - 1;
    
    while (h > l) {
        // Convert to lowercase for case-insensitive comparison
        if (tolower(str[l]) != tolower(str[h])) {
            return 0;
        }
        l++;
        h--;
    }
    return 1;
}

int main() {
    char str[] = "radar";
    char str2[] = "Madam";
    
    if (isPalindrome(str)) {
        printf("'%s' is a palindrome\n", str);
    } else {
        printf("'%s' is not a palindrome\n", str);
    }
    
    if (isPalindrome(str2)) {
        printf("'%s' is a palindrome\n", str2);
    } else {
        printf("'%s' is not a palindrome\n", str2);
    }
    
    return 0;
}
