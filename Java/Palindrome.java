// Palindrome.java
public class Palindrome {
    public static boolean isPalindrome(String str) {
        String cleaned = str.toLowerCase().replaceAll("\\s+", "");
        return cleaned.equals(new StringBuilder(cleaned).reverse().toString());
    }

    public static void main(String[] args) {
        String testString = "radar";
        System.out.println("Is '" + testString + "' a palindrome? " + isPalindrome(testString));
        
        String testString2 = "A man a plan a canal Panama";
        System.out.println("Is '" + testString2 + "' a palindrome? " + isPalindrome(testString2));
    }
}
