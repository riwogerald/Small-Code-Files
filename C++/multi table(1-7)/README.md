# Multiplication Table Generator (1-7)

This C++/CLI Windows Forms application creates a visual multiplication table displaying products from 1×1 to 7×7 in a graphical grid format.

## Overview

The program uses C++/CLI (Common Language Infrastructure) to create a Windows Forms application that renders a multiplication table directly onto a PictureBox control using GDI+ graphics. The table displays multiplication results for numbers 1 through 7 in an organized, easy-to-read grid format.

## Features

- **Visual Grid Display**: Clean, organized table with borders and proper spacing
- **Professional Formatting**: Uses different fonts and colors for headers and data
- **Custom Graphics Rendering**: Hand-drawn table using GDI+ for precise control
- **Fixed Window Size**: Non-resizable window optimized for the table display
- **Color-coded Elements**: Headers in dark blue, data in black for better readability

## Technical Specifications

- **Framework**: C++/CLI with Windows Forms
- **Table Size**: 7×7 multiplication table
- **Window Size**: 624×424 pixels
- **Cell Dimensions**: 70×35 pixels per cell
- **Graphics**: Custom GDI+ rendering in PictureBox Paint event

## Visual Layout

```
Multiplication Table
*   1   2   3   4   5   6   7
1   1   2   3   4   5   6   7
2   2   4   6   8   10  12  14
3   3   6   9   12  15  18  21
4   4   8   12  16  20  24  28
5   5   10  15  20  25  30  35
6   6   12  18  24  30  36  42
7   7   14  21  28  35  42  49
```

## Prerequisites

- Visual Studio with C++/CLI support
- .NET Framework
- Windows operating system

## Compilation

This project requires Visual Studio or a compatible C++/CLI compiler:

1. Open Visual Studio
2. Create a new C++/CLI project
3. Replace the default code with the provided source
4. Build the project (Ctrl+Shift+B)

## Key Components

### Classes and Structure
- **MultiplicationTableForm**: Main form class inheriting from System::Windows::Forms::Form
- **PictureBox**: Control used for custom graphics rendering
- **Paint Event Handler**: Custom drawing logic in `pictureBox1_Paint`

### Rendering Features
- **Grid Drawing**: Rectangle borders for each cell
- **Text Centering**: Automatic text alignment within cells
- **Font Management**: Multiple font styles for different elements
- **Resource Cleanup**: Proper disposal of graphics resources

## Main Functions

- `InitializeComponent()`: Sets up the Windows Forms interface
- `pictureBox1_Paint()`: Handles all custom drawing operations
- `main()`: Application entry point with Forms initialization

## Customization Options

You can easily modify:
- **Table Size**: Change the loop limits from 1-7 to any desired range
- **Colors**: Modify brush colors for headers and data
- **Fonts**: Adjust font sizes and styles
- **Cell Dimensions**: Change `cellWidth` and `cellHeight` variables
- **Window Size**: Modify `ClientSize` property

## Learning Objectives

This application demonstrates:
- C++/CLI Windows Forms programming
- Custom graphics rendering with GDI+
- Event-driven programming concepts
- Resource management in managed C++
- Mathematical table generation and display

## Usage

1. Compile and run the application
2. The multiplication table window will appear automatically
3. The table displays all multiplication results from 1×1 to 7×7
4. Close the window to exit the application

## Educational Value

Perfect for:
- Learning basic multiplication facts
- Understanding C++/CLI and Windows Forms
- Graphics programming with GDI+
- Creating educational mathematical tools
