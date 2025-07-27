# Geometric Shapes Application

## Overview

A comprehensive Java application demonstrating object-oriented programming principles through geometric shapes. This program showcases inheritance, polymorphism, abstract classes, and method overriding using Circle and Rectangle classes that extend an abstract GeometricShape base class.

## Features

### Abstract Base Class Design
- **GeometricShape**: Abstract base class defining common interface
- **Abstract Methods**: Forces subclasses to implement specific behaviors
- **Concrete Methods**: Shared functionality available to all shapes
- **Encapsulation**: Protected and private members with public interfaces

### Shape Implementations
- **Circle Class**: Complete circle implementation with radius-based calculations
- **Rectangle Class**: Complete rectangle implementation with length/width
- **Area Calculations**: Mathematical formulas for each shape type
- **Perimeter Calculations**: Boundary measurements for each shape
- **Shape Descriptions**: Detailed information display for each shape

### Polymorphism Demonstrations
- **Method Overriding**: Different implementations of abstract methods
- **Runtime Type Resolution**: Correct method calls based on object type
- **Polymorphic Arrays**: Process different shapes through common interface
- **Dynamic Method Dispatch**: Automatic selection of appropriate methods

## Technical Specifications

### Class Hierarchy

#### GeometricShape (Abstract Base Class)
```java
abstract class GeometricShape {
    protected String name;
    
    // Abstract methods (must be implemented)
    public abstract double calculateArea();
    public abstract double calculatePerimeter();
    public abstract void describeShape();
    
    // Concrete methods (inherited by all)
    public String getName();
    public void displayInfo();
}
```

#### Circle Class
```java
class Circle extends GeometricShape {
    private double radius;
    
    // Specific to circles
    public double getDiameter();
    public double getCircumference();
    
    // Overridden abstract methods
    public double calculateArea();        // π × r²
    public double calculatePerimeter();   // 2 × π × r
    public void describeShape();
}
```

#### Rectangle Class
```java
class Rectangle extends GeometricShape {
    private double length;
    private double width;
    
    // Overridden abstract methods
    public double calculateArea();        // length × width
    public double calculatePerimeter();   // 2 × (length + width)
    public void describeShape();
}
```

## Key Demonstrations

### 1. Inheritance Demonstration
- Shows how subclasses inherit from abstract base class
- Demonstrates shared functionality through inheritance
- Displays method calls from both parent and child classes

### 2. Polymorphism Demonstration
- Uses superclass references to point to different object types
- Processes arrays of different shapes using common interface
- Shows runtime method resolution

### 3. Method Overriding Demonstration
- Same method names with different implementations
- Shows how each shape calculates area differently
- Demonstrates behavior customization through overriding

### 4. Runtime Type Identification
- Uses `instanceof` operator for type checking
- Demonstrates safe type casting
- Shows how to access subclass-specific methods

### 5. Utility Method Example
- `findLargestShape()` method processes shapes polymorphically
- Compares shapes using common interface methods
- Returns largest shape by area

## Usage Instructions

### Running the Program
1. Compile: `javac geometric_shapes.java`
2. Run: `java GeometricShapesApp`

### Program Output Structure
The program automatically demonstrates all features:

1. **Inheritance Examples**: Direct method calls on objects
2. **Polymorphic Processing**: Array processing with mixed shape types
3. **Method Overriding**: Same method, different behaviors
4. **Type Identification**: Runtime type checking and casting
5. **Shape Comparison**: Finding largest shape by area

## Sample Output

```
=== Geometric Shapes Application ===

1. INHERITANCE DEMONSTRATION:
Circle object:
Shape: Circle
Area: 78.54
Perimeter: 31.42

=== Circle Description ===
Radius: 5.00
Diameter: 10.00
Area: 78.54
Circumference: 31.42
========================

2. POLYMORPHISM DEMONSTRATION:
Processing shapes using superclass references:

Shape 1:
Shape: Circle
Area: 38.48
Perimeter: 21.99

Shape 2:  
Shape: Rectangle
Area: 40.00
Perimeter: 28.00

3. METHOD OVERRIDING DEMONSTRATION:
Same method call, different behaviors:
Shape 1 area: 50.27
Shape 2 area: 18.00

4. RUNTIME TYPE IDENTIFICATION:
Shape type: Circle - Radius: 3.50
Shape type: Rectangle - Dimensions: 10.00 x 4.00

5. SHAPE COMPARISON:
Largest shape by area:
=== Rectangle Description ===
Length: 10.00
Width: 4.00
Area: 40.00
Perimeter: 28.00
============================
```

## Mathematical Formulas

### Circle Calculations
- **Area**: π × radius²
- **Perimeter (Circumference)**: 2 × π × radius
- **Diameter**: 2 × radius

### Rectangle Calculations
- **Area**: length × width  
- **Perimeter**: 2 × (length + width)

## Educational Value

This application demonstrates key OOP concepts:

### Object-Oriented Principles
- **Abstraction**: Abstract base class defines interface
- **Inheritance**: Subclasses inherit common functionality
- **Polymorphism**: Same interface, different implementations
- **Encapsulation**: Private data with public interfaces

### Advanced Java Concepts
- **Abstract Classes**: Cannot be instantiated directly
- **Method Overriding**: `@Override` annotation usage
- **Runtime Type Checking**: `instanceof` operator
- **Type Casting**: Safe downcasting with validation
- **Polymorphic Arrays**: Mixed object types in collections

### Design Patterns
- **Template Method Pattern**: Abstract methods define algorithm structure
- **Strategy Pattern**: Different calculation strategies per shape
- **Factory Pattern**: Could be extended for shape creation

## File Structure
```
geometric_shapes/
├── geometric_shapes.java       # Complete application
└── README.md                  # This documentation
```

## Possible Extensions

### Additional Shapes
- Triangle class with base and height
- Polygon class for n-sided shapes
- Ellipse class for oval shapes
- Complex shapes with multiple components

### Enhanced Features
- Color and fill properties
- 3D shape extensions (volume calculations)
- Shape drawing capabilities
- Shape transformation methods (scale, rotate, translate)
- Shape comparison and sorting utilities

### Advanced OOP Features
- Interface implementations
- Generic type parameters
- Builder pattern for shape creation
- Visitor pattern for shape operations

## Key Learning Outcomes
- Understanding abstract classes vs interfaces
- Mastering inheritance hierarchies
- Implementing polymorphism effectively
- Using method overriding appropriately
- Runtime type identification techniques
- Designing extensible object hierarchies
