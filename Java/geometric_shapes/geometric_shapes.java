// Abstract base class for geometric shapes
abstract class GeometricShape {
    protected String name;
    
    // Constructor
    public GeometricShape(String name) {
        this.name = name;
    }
    
    // Abstract methods that must be implemented by subclasses
    public abstract double calculateArea();
    public abstract double calculatePerimeter();
    public abstract void describeShape();
    
    // Concrete method available to all subclasses
    public String getName() {
        return name;
    }
    
    public void setName(String name) {
        this.name = name;
    }
    
    // Method to display basic information
    public void displayInfo() {
        System.out.println("Shape: " + name);
        System.out.println("Area: " + String.format("%.2f", calculateArea()));
        System.out.println("Perimeter: " + String.format("%.2f", calculatePerimeter()));
    }
}

// Concrete class for Circle
class Circle extends GeometricShape {
    private double radius;
    
    // Constructor
    public Circle(double radius) {
        super("Circle");
        this.radius = radius;
    }
    
    // Getter and setter for radius
    public double getRadius() {
        return radius;
    }
    
    public void setRadius(double radius) {
        this.radius = radius;
    }
    
    // Calculate diameter
    public double getDiameter() {
        return 2 * radius;
    }
    
    // Calculate circumference (perimeter for circle)
    public double getCircumference() {
        return calculatePerimeter();
    }
    
    // Implementation of abstract methods
    @Override
    public double calculateArea() {
        return Math.PI * radius * radius;
    }
    
    @Override
    public double calculatePerimeter() {
        return 2 * Math.PI * radius;
    }
    
    @Override
    public void describeShape() {
        System.out.println("\n=== Circle Description ===");
        System.out.println("Radius: " + String.format("%.2f", radius));
        System.out.println("Diameter: " + String.format("%.2f", getDiameter()));
        System.out.println("Area: " + String.format("%.2f", calculateArea()));
        System.out.println("Circumference: " + String.format("%.2f", getCircumference()));
        System.out.println("========================");
    }
}

// Concrete class for Rectangle
class Rectangle extends GeometricShape {
    private double length;
    private double width;
    
    // Constructor
    public Rectangle(double length, double width) {
        super("Rectangle");
        this.length = length;
        this.width = width;
    }
    
    // Getters and setters
    public double getLength() {
        return length;
    }
    
    public void setLength(double length) {
        this.length = length;
    }
    
    public double getWidth() {
        return width;
    }
    
    public void setWidth(double width) {
        this.width = width;
    }
    
    // Implementation of abstract methods
    @Override
    public double calculateArea() {
        return length * width;
    }
    
    @Override
    public double calculatePerimeter() {
        return 2 * (length + width);
    }
    
    @Override
    public void describeShape() {
        System.out.println("\n=== Rectangle Description ===");
        System.out.println("Length: " + String.format("%.2f", length));
        System.out.println("Width: " + String.format("%.2f", width));
        System.out.println("Area: " + String.format("%.2f", calculateArea()));
        System.out.println("Perimeter: " + String.format("%.2f", calculatePerimeter()));
        System.out.println("============================");
    }
}

// Main application class
public class GeometricShapesApp {
    public static void main(String[] args) {
        System.out.println("=== Geometric Shapes Application ===\n");
        
        // Create concrete objects
        Circle circle = new Circle(5.0);
        Rectangle rectangle = new Rectangle(8.0, 6.0);
        
        // Demonstrate inheritance - calling methods from superclass and subclass
        System.out.println("1. INHERITANCE DEMONSTRATION:");
        System.out.println("------------------------------");
        
        // Circle using inherited and overridden methods
        System.out.println("Circle object:");
        circle.displayInfo(); // inherited method
        circle.describeShape(); // overridden method
        
        // Rectangle using inherited and overridden methods
        System.out.println("Rectangle object:");
        rectangle.displayInfo(); // inherited method
        rectangle.describeShape(); // overridden method
        
        // Demonstrate polymorphism using superclass references
        System.out.println("\n2. POLYMORPHISM DEMONSTRATION:");
        System.out.println("--------------------------------");
        
        // Array of GeometricShape references pointing to different objects
        GeometricShape[] shapes = {
            new Circle(3.5),
            new Rectangle(10.0, 4.0),
            new Circle(7.2),
            new Rectangle(5.5, 5.5)
        };
        
        // Process shapes polymorphically
        System.out.println("Processing shapes using superclass references:");
        for (int i = 0; i < shapes.length; i++) {
            System.out.println("\nShape " + (i + 1) + ":");
            // Polymorphic method calls - runtime decides which implementation to use
            shapes[i].displayInfo();
            shapes[i].describeShape();
        }
        
        // Demonstrate method overriding with different behaviors
        System.out.println("\n3. METHOD OVERRIDING DEMONSTRATION:");
        System.out.println("-----------------------------------");
        
        GeometricShape shape1 = new Circle(4.0);
        GeometricShape shape2 = new Rectangle(6.0, 3.0);
        
        System.out.println("Same method call, different behaviors:");
        System.out.println("Shape 1 area: " + String.format("%.2f", shape1.calculateArea()));
        System.out.println("Shape 2 area: " + String.format("%.2f", shape2.calculateArea()));
        
        // Demonstrate runtime type checking
        System.out.println("\n4. RUNTIME TYPE IDENTIFICATION:");
        System.out.println("--------------------------------");
        
        for (GeometricShape shape : shapes) {
            System.out.print("Shape type: " + shape.getClass().getSimpleName());
            
            // Type checking and casting
            if (shape instanceof Circle) {
                Circle c = (Circle) shape;
                System.out.println(" - Radius: " + String.format("%.2f", c.getRadius()));
            } else if (shape instanceof Rectangle) {
                Rectangle r = (Rectangle) shape;
                System.out.println(" - Dimensions: " + String.format("%.2f", r.getLength()) + 
                                 " x " + String.format("%.2f", r.getWidth()));
            }
        }
        
        // Demonstrate comparing shapes
        System.out.println("\n5. SHAPE COMPARISON:");
        System.out.println("--------------------");
        
        GeometricShape largestShape = findLargestShape(shapes);
        System.out.println("Largest shape by area:");
        largestShape.describeShape();
    }
    
    // Utility method demonstrating polymorphism
    public static GeometricShape findLargestShape(GeometricShape[] shapes) {
        if (shapes.length == 0) return null;
        
        GeometricShape largest = shapes[0];
        for (GeometricShape shape : shapes) {
            if (shape.calculateArea() > largest.calculateArea()) {
                largest = shape;
            }
        }
        return largest;
    }
}