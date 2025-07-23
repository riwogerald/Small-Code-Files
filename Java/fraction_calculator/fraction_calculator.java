import javax.swing.JOptionPane;

public class FractionCalculator {
    public static void main(String[] args) {
        String value;
        int n, d;
        float num1, num2, sum;
        
        // Get first fraction
        value = JOptionPane.showInputDialog("Enter the numerator for the first number");
        n = Integer.parseInt(value);
        
        do {
            value = JOptionPane.showInputDialog("Enter the denominator for the first number");
            d = Integer.parseInt(value);
            if (d == 0) {
                JOptionPane.showMessageDialog(null, "The denominator cannot be zero. Please try again.");
            }
        } while (d == 0);
        
        num1 = (float)n / (float)d;
        
        // Get second fraction
        value = JOptionPane.showInputDialog("Enter the numerator for the second number");
        n = Integer.parseInt(value);
        
        do {
            value = JOptionPane.showInputDialog("Enter the denominator for the second number");
            d = Integer.parseInt(value);
            if (d == 0) {
                JOptionPane.showMessageDialog(null, "The denominator cannot be zero. Please try again.");
            }
        } while (d == 0);
        
        num2 = (float)n / (float)d;
        
        sum = num1 + num2;
        JOptionPane.showMessageDialog(null, "Sum of " + num1 + " + " + num2 + " is " + sum);
    }
}