// PrimeNumber.java
public class PrimeNumber {
    public static boolean isPrime(int num) {
        if (num <= 1) return false;
        if (num <= 3) return true;
        if (num % 2 == 0 || num % 3 == 0) return false;
        
        for (int i = 5; i * i <= num; i += 6) {
            if (num % i == 0 || num % (i + 2) == 0) {
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        int testNum = 11;
        System.out.println("Is " + testNum + " a prime number? " + isPrime(testNum));
        
        int testNum2 = 25;
        System.out.println("Is " + testNum2 + " a prime number? " + isPrime(testNum2));
    }
}
