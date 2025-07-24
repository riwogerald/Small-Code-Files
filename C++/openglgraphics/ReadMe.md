# OpenGL Graphics Programs

This project contains four OpenGL graphics programs written in C++ that demonstrate basic 2D graphics operations including drawing shapes, transformations, and reflections.

## Programs Included

1. **Square Drawing** - Renders a blue quadrilateral with custom coordinates
2. **Triangle Drawing** - Displays a red triangle with rotation and a dotted shadow square
3. **Square Scaling** - Shows a red square with rotation and scaling transformations
4. **Square Reflection** - Demonstrates Y-axis reflection of a red square

## Prerequisites

Before compiling and running these programs, you need to have the following installed:

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install build-essential
sudo apt-get install freeglut3-dev
sudo apt-get install libgl1-mesa-dev
sudo apt-get install libglu1-mesa-dev
```

### Linux (CentOS/RHEL/Fedora)
```bash
# For CentOS/RHEL
sudo yum install gcc-c++
sudo yum install freeglut-devel
sudo yum install mesa-libGL-devel
sudo yum install mesa-libGLU-devel

# For Fedora
sudo dnf install gcc-c++
sudo dnf install freeglut-devel
sudo dnf install mesa-libGL-devel
sudo dnf install mesa-libGLU-devel
```

### macOS
```bash
# Install Xcode command line tools
xcode-select --install

# Using Homebrew
brew install freeglut
```

### Windows
- Install Visual Studio with C++ support
- Download and install FreeGLUT from: http://freeglut.sourceforge.net/
- Set up appropriate library paths in your IDE

## Compilation

### Linux/macOS
Navigate to the directory containing the source code and compile using:

```bash
g++ -o opengl_programs main.cpp -lGL -lGLU -lglut
```

If you encounter linking issues, try:
```bash
g++ -o opengl_programs main.cpp -lOpenGL -lGLUT
```

### Alternative compilation with more verbose flags:
```bash
g++ -std=c++11 -Wall -o opengl_programs main.cpp -lGL -lGLU -lglut
```

## Running the Program

After successful compilation, run the executable:

```bash
./opengl_programs
```

You'll see a menu with options:
```
Select which program to run:
1. Square Drawing
2. Triangle Drawing
3. Square Scaling
4. Square Reflection
Enter choice (1-4):
```

Enter a number (1-4) to select which graphics program to run.

## Program Details

### 1. Square Drawing
- **Description**: Draws a blue quadrilateral using custom vertex coordinates
- **Features**: Basic shape rendering with specified coordinates
- **Controls**: None (static display)

### 2. Triangle Drawing
- **Description**: Renders a red triangle with a 48-degree rotation
- **Features**: 
  - Rotated red triangle
  - Dotted line shadow square overlay
  - Demonstrates rotation transformations
- **Controls**: None (static display)

### 3. Square Scaling
- **Description**: Shows a red square with both rotation and scaling applied
- **Features**:
  - 48-degree rotation
  - 2x scale factor
  - Demonstrates combined transformations
- **Controls**: None (static display)

### 4. Square Reflection
- **Description**: Displays a red square and its reflection across the Y-axis
- **Features**:
  - Original square on one side
  - Mirrored square on the other side
  - Demonstrates reflection transformation
- **Controls**: None (static display)

## Window Controls

All programs create an 800x600 pixel window with:
- **Close**: Click the window's close button or press Alt+F4
- **Refresh**: The display is static and doesn't require refreshing

## Troubleshooting

### Common Issues

1. **"GL/glut.h: No such file or directory"**
   - Solution: Install GLUT development libraries (see Prerequisites section)

2. **Linking errors with -lGL, -lGLU, -lglut**
   - Try using `-lOpenGL -lGLUT` instead
   - On some systems, try `-lglut32` for GLUT

3. **Program compiles but crashes on startup**
   - Ensure your graphics drivers support OpenGL
   - Try running with `LIBGL_ALWAYS_SOFTWARE=1 ./opengl_programs` for software rendering

4. **Black screen or no display**
   - Check if your OpenGL context is properly initialized
   - Verify graphics drivers are up to date

### Platform-Specific Notes

- **Linux**: If using a virtual machine, ensure 3D acceleration is enabled
- **macOS**: On newer versions, you might need to use OpenGL compatibility profile
- **Windows**: Make sure GLUT DLLs are in your system PATH or application directory

## File Structure

```
project/
├── main.cpp          # Main source code file
├── README.md         # This file
└── opengl_programs   # Compiled executable (after compilation)
```

## Development Notes

- The code uses legacy OpenGL (immediate mode) for educational purposes
- Each program is contained in its own namespace to avoid conflicts
- All graphics operations use 2D coordinates with orthographic projection
- The programs demonstrate basic OpenGL concepts suitable for learning

## License

This project is provided for educational purposes. Feel free to modify and distribute as needed.

## Contributing

To contribute to this project:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test compilation and functionality
5. Submit a pull request

## Support

If you encounter issues:
1. Check the troubleshooting section above
2. Verify all prerequisites are installed
3. Test with the simplest program first (Square Drawing)
4. Check your OpenGL and graphics driver versions