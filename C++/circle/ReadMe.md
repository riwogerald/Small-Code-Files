# Circle Drawing Algorithms in C++

This program demonstrates two fundamental computer graphics algorithms for drawing circles: the **Mid-point Circle Algorithm** and **Bresenham's Circle Algorithm**. Both algorithms are implemented using OpenGL for visualization and provide an interactive way to understand how circles are rasterized on a pixel grid.

## Table of Contents
- [Overview](#overview)
- [Algorithms Explained](#algorithms-explained)
- [Installation](#installation)
- [Usage](#usage)
- [Program Structure](#program-structure)
- [Mathematical Background](#mathematical-background)
- [Performance Comparison](#performance-comparison)
- [Troubleshooting](#troubleshooting)

## Overview

Circle drawing in computer graphics is a fundamental operation that requires efficient algorithms to plot pixels on a discrete grid. This program implements two classic approaches:

1. **Mid-point Circle Algorithm** - Also known as Bresenham's Circle Algorithm (original version)
2. **Bresenham's Circle Algorithm** - An optimized version with integer arithmetic

Both algorithms use the concept of **8-fold symmetry** to draw a complete circle by calculating only one octant and mirroring the points to the other seven octants.

## Algorithms Explained

### Mid-point Circle Algorithm

The mid-point algorithm uses a decision parameter to determine whether the next pixel should be drawn horizontally or diagonally. 

**Key Features:**
- Uses the circle equation: `x² + y² = r²`
- Decision parameter: `d = 1 - r`
- Updates based on whether `d < 0` or `d ≥ 0`
- Highly efficient with only integer arithmetic

**How it works:**
1. Start at point (0, r)
2. Calculate decision parameter `d = 1 - r`
3. For each x from 0 to r:
   - If `d < 0`: move horizontally (increment x only)
   - If `d ≥ 0`: move diagonally (increment x, decrement y)
   - Update decision parameter accordingly
4. Plot all 8 symmetric points for each calculated point

### Bresenham's Circle Algorithm

This is an optimized version that uses a different decision parameter formulation.

**Key Features:**
- Decision parameter: `d = 3 - 2r`
- Different update formulas for better performance
- Also uses only integer arithmetic
- Slightly different pixel selection criteria

**How it works:**
1. Start at point (0, r)
2. Calculate decision parameter `d = 3 - 2r`
3. For each point while `y ≥ x`:
   - If `d ≤ 0`: move horizontally
   - If `d > 0`: move diagonally
   - Update decision parameter with different formulas
4. Plot all 8 symmetric points

### 8-fold Symmetry

Both algorithms exploit the symmetry of a circle. For any point (x, y) on the circle, these 8 points are also on the circle:
- (x, y), (-x, y), (x, -y), (-x, -y)
- (y, x), (-y, x), (y, -x), (-y, -x)

This means we only need to calculate 1/8 of the circle and mirror the rest.

## Installation

### Prerequisites
- C++ compiler (g++, clang++, or MSVC)
- OpenGL libraries
- GLUT (OpenGL Utility Toolkit)

### Platform-specific Installation

#### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install build-essential
sudo apt-get install freeglut3-dev

# Compile the program
g++ -o circle_algorithms circle_algorithms.cpp -lGL -lGLU -lglut

# Run the program
./circle_algorithms
```

#### Windows (MinGW)
```bash
# Install MinGW and GLUT libraries
# Download freeglut from: http://freeglut.sourceforge.net/

# Compile
g++ -o circle_algorithms.exe circle_algorithms.cpp -lopengl32 -lglu32 -lfreeglut

# Run
circle_algorithms.exe
```

#### macOS
```bash
# Install Xcode command line tools
xcode-select --install

# Compile (OpenGL and GLUT are included in macOS)
g++ -o circle_algorithms circle_algorithms.cpp -framework OpenGL -framework GLUT

# Run
./circle_algorithms
```

## Usage

### Controls

| Key/Action | Function |
|------------|----------|
| `1` | Switch to Mid-point Circle Algorithm (Red) |
| `2` | Switch to Bresenham's Circle Algorithm (Green) |
| `+` | Increase circle radius |
| `-` | Decrease circle radius |
| Left Click | Move circle center to mouse position |
| `Q` or `ESC` | Quit the program |

### Getting Started

1. **Launch the program** - You'll see a red circle drawn with the mid-point algorithm
2. **Compare algorithms** - Press `2` to switch to Bresenham's algorithm (green circle)
3. **Adjust parameters** - Use `+`/`-` to change radius, click to move center
4. **Observe differences** - Notice any visual differences between the two algorithms

### Visual Indicators

- **Red Circle**: Mid-point Circle Algorithm
- **Green Circle**: Bresenham's Circle Algorithm
- **Console Output**: Real-time feedback on current settings and actions

## Program Structure

```
circle_algorithms.cpp
├── Global Variables
│   ├── Circle parameters (center_x, center_y, radius)
│   ├── Window dimensions
│   └── Algorithm selector
├── Core Functions
│   ├── draw_symmetric_points() - Plots 8 symmetric points
│   ├── draw_circle_midpoint() - Mid-point algorithm implementation
│   └── draw_circle_bresenham() - Bresenham's algorithm implementation
├── OpenGL Functions
│   ├── init() - OpenGL initialization
│   ├── display() - Main rendering function
│   ├── keyboard() - Keyboard input handler
│   └── mouse() - Mouse input handler
└── Main Function
    └── GLUT setup and main loop
```

### Key Functions Explained

#### `draw_symmetric_points(int x, int y)`
This function takes a single point (x, y) and plots all 8 symmetric points around the circle center. This is the core of the 8-fold symmetry optimization.

#### `draw_circle_midpoint()` and `draw_circle_bresenham()`
These functions implement the respective algorithms. They:
1. Initialize starting conditions
2. Loop through the first octant
3. Calculate decision parameters
4. Call `draw_symmetric_points()` for each calculated point

## Mathematical Background

### Circle Equation
The standard circle equation is: `(x - h)² + (y - k)² = r²`
Where (h, k) is the center and r is the radius.

### Decision Parameters

**Mid-point Algorithm:**
- Initial: `d = 1 - r`
- If `d < 0`: `d = d + 2x + 3`
- If `d ≥ 0`: `d = d + 2(x-y) + 5`

**Bresenham's Algorithm:**
- Initial: `d = 3 - 2r`
- If `d ≤ 0`: `d = d + 4x + 6`
- If `d > 0`: `d = d + 4(x-y) + 10`

### Why These Work
Both algorithms are based on the mathematical property that for a circle, we can determine the next pixel by evaluating whether the actual circle passes above or below the midpoint between potential pixel locations.

## Performance Comparison

| Aspect | Mid-point | Bresenham's |
|--------|-----------|-------------|
| **Arithmetic** | Integer only | Integer only |
| **Accuracy** | High | High |
| **Speed** | Fast | Slightly faster |
| **Memory** | Low | Low |
| **Complexity** | Simple | Simple |

Both algorithms are extremely efficient with O(r) time complexity, where r is the radius.

## Applications

These algorithms are fundamental in:
- **Computer Graphics**: Basic shape rendering
- **Game Development**: Sprite rendering, collision detection
- **Image Processing**: Shape analysis, feature detection  
- **CAD Software**: Technical drawing applications
- **Scientific Visualization**: Data plotting and analysis

## Educational Value

This program demonstrates:
- **Discrete Mathematics**: How continuous curves are approximated on pixel grids
- **Algorithm Optimization**: Integer-only arithmetic for performance
- **Symmetry in Mathematics**: Exploiting geometric properties
- **Computer Graphics Fundamentals**: Rasterization concepts

## Troubleshooting

### Common Issues

1. **Compilation Errors**
   ```
   Error: GL/glut.h not found
   ```
   **Solution**: Install GLUT development libraries for your platform

2. **Linking Errors**
   ```
   undefined reference to 'glVertex2f'
   ```
   **Solution**: Make sure you're linking OpenGL libraries correctly

3. **Window Doesn't Appear**
   **Solution**: Check if your system supports OpenGL and GLUT

4. **Circle Appears Distorted**
   **Solution**: This is normal on low radius values due to pixelation

### Performance Tips

- For very large circles, you might notice performance differences
- The algorithms are optimized for typical graphics applications
- Modern GPUs handle circle drawing differently, but these algorithms remain educational

## Further Reading

- [Computer Graphics: Principles and Practice](https://www.pearson.com/us/higher-education/program/Hughes-Computer-Graphics-Principles-and-Practice-3rd-Edition/PGM94572.html)
- [Bresenham's Original Paper (1965)](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
- [OpenGL Programming Guide](https://www.opengl.org/documentation/books/)

## License

This educational program is provided as-is for learning purposes. Feel free to modify and distribute for educational use.

---

*Happy coding and circle drawing! 🔴🟢*