// GCD.java
public class GCD {
    public static int gcd(int a, int b) {
        return (b == 0) ? a : gcd(b, a % b);
    }

    public static void main(String[] args) {
        System.out.println("GCD of 56 and 98: " + gcd(56, 98));
    }
}

