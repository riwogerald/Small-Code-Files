// LCM.java
public class LCM {
    public static int lcm(int a, int b) {
        return (a * b) / gcd(a, b);
    }

    public static int gcd(int a, int b) {
        return (b == 0) ? a : gcd(b, a % b);
    }

    public static void main(String[] args) {
        System.out.println("LCM of 12 and 15: " + lcm(12, 15));
    }
}
