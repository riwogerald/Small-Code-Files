# Circle Drawing using Midpoint Algorithm

This C++ program demonstrates circle drawing using the **Midpoint Circle Algorithm** (also known as Bresenham's Circle Algorithm) with OpenGL graphics.

## Overview

The program creates a graphical window and draws a circle using discrete points calculated by the midpoint algorithm. This algorithm is efficient for raster graphics as it uses only integer arithmetic and avoids expensive floating-point operations.

## Features

- **Midpoint Circle Algorithm**: Implements the classic computer graphics algorithm for drawing circles
- **8-fold Symmetry**: Takes advantage of circle symmetry to draw 8 points for each calculated point
- **OpenGL Rendering**: Uses OpenGL for graphics display with GLUT for window management
- **Configurable Parameters**: Easy to modify circle center, radius, and window dimensions

## Algorithm Details

The midpoint circle algorithm works by:
1. Starting at point (0, radius) 
2. Using a decision parameter to determine whether to increment x or decrement y
3. Drawing 8 symmetric points for each calculated point
4. Continuing until the curve completes one octant

## Technical Specifications

- **Circle Center**: (3, 3)
- **Radius**: 4 units
- **Window Size**: 800x600 pixels
- **Point Color**: Red (RGB: 1.0, 0.0, 0.0)
- **Background**: Black

## Prerequisites

- OpenGL development libraries
- GLUT (OpenGL Utility Toolkit)
- C++ compiler with OpenGL support

## Compilation

```bash
g++ circle_midpoint.cpp -o circle_midpoint -lGL -lGLU -lglut
```

## Usage

Run the compiled executable to display the circle in a graphics window:

```bash
./circle_midpoint
```

## Key Functions

- `draw_symmetric_points()`: Draws 8 symmetric points for a given (x,y) coordinate
- `draw_circle()`: Main algorithm implementation
- `display()`: OpenGL display callback function
- `init()`: OpenGL initialization and viewport setup

## Learning Objectives

This program is ideal for understanding:
- Computer graphics algorithms
- Circle rasterization techniques
- OpenGL programming basics
- Integer-based geometric algorithms
- Symmetric point plotting

## Modifications

You can easily modify:
- Circle center by changing `center_x` and `center_y`
- Circle radius by changing the `radius` constant
- Colors by modifying the `glColor3f()` values
- Window size by adjusting `window_width` and `window_height`
