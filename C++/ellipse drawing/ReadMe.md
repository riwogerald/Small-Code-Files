# C++ Ellipse Renderer

A simple OpenGL application that renders an ellipse using C++ and GLUT. This program demonstrates basic OpenGL graphics programming concepts including coordinate systems, parametric equations, and interactive controls.

## What It Does

The program creates a 500x500 pixel window with a black background and draws a white ellipse. The ellipse is generated using parametric equations:
- `x = centerX + a * cos(θ)`
- `y = centerY + b * sin(θ)`

Where `a` and `b` are the semi-major and semi-minor axes, and `θ` ranges from 0 to 360 degrees.

## Default Parameters

- **Center**: (-2, 2)
- **Semi-major axis**: 6.0
- **Semi-minor axis**: 5.0
- **Coordinate system**: -10 to +10 in both X and Y directions

## Prerequisites

Before compiling, you need to have the following installed:

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install freeglut3-dev libglu1-mesa-dev
```

### Linux (Red Hat/CentOS/Fedora)
```bash
sudo yum install freeglut-devel mesa-libGLU-devel
# or for newer versions:
sudo dnf install freeglut-devel mesa-libGLU-devel
```

### macOS
```bash
# Install Xcode command line tools first
xcode-select --install

# Then install via Homebrew
brew install freeglut
```

### Windows
- Install Visual Studio with C++ support
- Download and install freeglut libraries
- Or use MSYS2/MinGW with the appropriate packages

## How to Compile and Run

1. **Save the code** as `ellipse_renderer.cpp`

2. **Compile** using g++:
   ```bash
   g++ ellipse_renderer.cpp -lGL -lGLU -lglut -o ellipse_renderer
   ```

3. **Run** the executable:
   ```bash
   ./ellipse_renderer
   ```

## Controls

Once the program is running, you can use these keyboard controls:

- **R** - Reset ellipse to default parameters
- **Q** or **ESC** - Quit the program

## How It Works

### Code Structure

The program uses object-oriented design with an `EllipseRenderer` class that encapsulates:

- **Private members**: Center coordinates and axis lengths
- **Public methods**: Drawing function, parameter setters/getters
- **Constructor**: Initializes with default values

### Rendering Process

1. **Initialization**: Sets up the OpenGL context and coordinate system
2. **Display Loop**: 
   - Clears the screen
   - Sets drawing color to white
   - Calls the ellipse drawing method
   - Flushes the graphics pipeline
3. **Ellipse Generation**: Creates 360 points around the ellipse using trigonometric functions

### Mathematical Foundation

The ellipse is drawn by calculating points using the parametric form:
```
For θ from 0° to 360°:
  x = centerX + a × cos(θ)
  y = centerY + b × sin(θ)
```

These points are connected using OpenGL's `GL_LINE_LOOP` primitive to form a continuous ellipse outline.

## Troubleshooting

### Common Issues

**"GL/glut.h: No such file or directory"**
- Install the GLUT development libraries (see Prerequisites)

**"undefined reference to 'glutInit'"**
- Make sure you're linking with `-lGL -lGLU -lglut`

**Black screen appears but no ellipse**
- Check that your graphics drivers support OpenGL
- Verify the coordinate system matches your ellipse parameters

**Program compiles but won't run**
- Ensure you have a display/X11 session if running on Linux
- Check that OpenGL is properly installed on your system

## Extending the Program

You can easily modify this program to:
- Change ellipse parameters by modifying the constructor call
- Add multiple ellipses by creating additional `EllipseRenderer` objects
- Implement animation by updating parameters in a timer callback
- Add mouse controls for interactive ellipse manipulation
- Change colors or add filled ellipses instead of just outlines

## License

This is a simple educational example - feel free to use and modify as needed.