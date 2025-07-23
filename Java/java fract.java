import javax.swing.JOptionPane;

public class FractionSum {
    public static void main(String[] args) {
        float num1 = getFraction("first");
        float num2 = getFraction("second");

        float sum = num1 + num2;
        JOptionPane.showMessageDialog(null, "Sum of " + num1 + " + " + num2 + " is " + sum);
    }

    public static float getFraction(String order) {
        int numerator = Integer.parseInt(JOptionPane.showInputDialog("Enter the numerator for the " + order + " number"));
        int denominator;

        // Loop to ensure the denominator is not zero or negative
        while (true) {
            denominator = Integer.parseInt(JOptionPane.showInputDialog("Enter the denominator for the " + order + " number"));
            if (denominator > 0) {
                break;
            }
            JOptionPane.showMessageDialog(null, "The denominator cannot be zero or negative. Please try again.");
        }

        return (float) numerator / (float) denominator;
    }
}
