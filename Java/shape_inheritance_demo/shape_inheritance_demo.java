// Abstract base class Shape
abstract class Shape {
    // Abstract method that must be implemented by subclasses
    public abstract void describe();
}

// Point class inheriting from Shape
class Point extends Shape {
    protected double x;
    protected double y;
    
    // Constructor for Point
    public Point(double x, double y) {
        this.x = x;
        this.y = y;
    }
    
    // Getter methods
    public double getX() {
        return x;
    }
    
    public double getY() {
        return y;
    }
    
    // Override the abstract describe method
    @Override
    public void describe() {
        System.out.println("Point: (" + x + ", " + y + ")");
    }
}

// Circle class extending Point
class Circle extends Point {
    private double radius;
    
    // Constructor for Circle - center point and radius
    public Circle(double centerX, double centerY, double radius) {
        super(centerX, centerY); // Call Point constructor for center coordinates
        this.radius = radius;
    }
    
    // Getter method for radius
    public double getRadius() {
        return radius;
    }
    
    // Override the describe method
    @Override
    public void describe() {
        System.out.println("Circle: center (" + x + ", " + y + "), radius = " + radius);
    }
}

// Test class to demonstrate functionality
public class ShapeTester {
    public static void main(String[] args) {
        System.out.println("=== Shape Inheritance and Polymorphism Demo ===\n");
        
        // Create instances of Point and Circle
        Point point1 = new Point(3.5, 2.8);
        Circle circle1 = new Circle(1.0, 4.5, 2.3);
        
        // Direct method calls
        System.out.println("Direct method calls:");
        point1.describe();
        circle1.describe();
        
        System.out.println("\n=== Demonstrating Polymorphism ===");
        
        // Polymorphic behavior - Shape references pointing to different objects
        Shape[] shapes = {
            new Point(0.0, 0.0),
            new Point(-2.5, 3.7),
            new Circle(5.0, -1.5, 4.0),
            new Circle(0.0, 0.0, 1.0)
        };
        
        // Polymorphic method calls - the correct describe() method is called
        // based on the actual object type, not the reference type
        System.out.println("Polymorphic method calls using Shape array:");
        for (int i = 0; i < shapes.length; i++) {
            System.out.print("Shape " + (i + 1) + " - ");
            shapes[i].describe(); // Calls the appropriate describe() method
        }
        
        // Another example of polymorphism using a method parameter
        System.out.println("\nPolymorphic method parameter example:");
        displayShape(new Point(10.0, 20.0));
        displayShape(new Circle(7.5, 8.2, 3.6));
    }
    
    // Method demonstrating polymorphism through parameter passing
    public static void displayShape(Shape shape) {
        System.out.print("Displaying shape: ");
        shape.describe(); // Polymorphic call - actual method depends on object type
    }
}