// Fibonacci.java
public class Fibonacci {
    public static void printFibonacci(int n) {
        int t1 = 0, t2 = 1, nextTerm;
        System.out.print("Fibonacci Series: " + t1 + ", " + t2);

        for (int i = 2; i < n; i++) {
            nextTerm = t1 + t2;
            System.out.print(", " + nextTerm);
            t1 = t2;
            t2 = nextTerm;
        }
        System.out.println();
    }

    public static void main(String[] args) {
        printFibonacci(10);
    }
}
