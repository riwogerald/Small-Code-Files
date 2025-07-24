#include <GL/glut.h>
#include <GL/gl.h>
#include <GL/glu.h>
#include <iostream>

// ================================ SQUARE DRAWING ================================

namespace SquareDrawing {
    // Square coordinates
    float vertices[][2] = {{-3, 5}, {0, -4}, {3, 7}, {6, -2}};
    
    // Window size
    const int window_width = 800;
    const int window_height = 600;
    
    // Function to draw the square
    void draw_square() {
        glBegin(GL_QUADS);
        glColor3f(0.0f, 0.0f, 1.0f);  // Blue color
        for (int i = 0; i < 4; i++) {
            glVertex2f(vertices[i][0], vertices[i][1]);
        }
        glEnd();
    }
    
    // OpenGL initialization
    void init() {
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        gluOrtho2D(-window_width/2, window_width/2, -window_height/2, window_height/2);
    }
    
    // Display function
    void display() {
        glClear(GL_COLOR_BUFFER_BIT);
        glPointSize(1.0f);
        draw_square();
        glFlush();
    }
    
    void run() {
        glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
        glutInitWindowSize(window_width, window_height);
        glutCreateWindow("Square Drawing");
        glutDisplayFunc(display);
        init();
        glutMainLoop();
    }
}

// =============================== TRIANGLE DRAWING ===============================

namespace TriangleDrawing {
    // Triangle vertices
    float vertices[][2] = {{0.0f, 0.5f}, {-0.5f, -0.5f}, {0.5f, -0.5f}};
    
    // Window size
    const int window_width = 800;
    const int window_height = 600;
    
    // Rotation angle in degrees
    const float rotation_angle = 48.0f;
    
    // Function to draw the triangle
    void draw_triangle() {
        glBegin(GL_TRIANGLES);
        glColor3f(1.0f, 0.0f, 0.0f);  // Red color
        for (int i = 0; i < 3; i++) {
            glVertex2f(vertices[i][0], vertices[i][1]);
        }
        glEnd();
    }
    
    // Function to draw the shadow square
    void draw_shadow_square() {
        glBegin(GL_LINE_LOOP);
        glColor3f(1.0f, 0.0f, 0.0f);  // Red color
        glLineStipple(1, 0xAAAA);  // Dotted line pattern
        glEnable(GL_LINE_STIPPLE);
        glVertex2f(-0.5f, 0.5f);
        glVertex2f(-0.5f, -0.5f);
        glVertex2f(0.5f, -0.5f);
        glVertex2f(0.5f, 0.5f);
        glEnd();
        glDisable(GL_LINE_STIPPLE);
    }
    
    // Function to rotate the triangle
    void rotate_triangle() {
        glTranslatef(0.0f, 0.0f, 0.0f);
        glRotatef(rotation_angle, 0.0f, 0.0f, 1.0f);
    }
    
    // OpenGL initialization
    void init() {
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        gluOrtho2D(-window_width/2, window_width/2, -window_height/2, window_height/2);
    }
    
    // Display function
    void display() {
        glClear(GL_COLOR_BUFFER_BIT);
        glPointSize(1.0f);
        draw_triangle();
        rotate_triangle();
        draw_shadow_square();
        glFlush();
    }
    
    void run() {
        glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
        glutInitWindowSize(window_width, window_height);
        glutCreateWindow("Triangle Rotation");
        glutDisplayFunc(display);
        init();
        glutMainLoop();
    }
}

// ============================= ROTATION BY FACTOR OF 2 =============================

namespace SquareScaling {
    // Square vertices
    float vertices[][2] = {{-0.5f, 0.5f}, {-0.5f, -0.5f}, {0.5f, -0.5f}, {0.5f, 0.5f}};
    
    // Window size
    const int window_width = 800;
    const int window_height = 600;
    
    // Rotation angle in degrees
    const float rotation_angle = 48.0f;
    
    // Scale factor
    const float scale_factor = 2.0f;
    
    // Function to draw the square
    void draw_square() {
        glBegin(GL_QUADS);
        glColor3f(1.0f, 0.0f, 0.0f);  // Red color
        for (int i = 0; i < 4; i++) {
            glVertex2f(vertices[i][0], vertices[i][1]);
        }
        glEnd();
    }
    
    // Function to rotate the square
    void rotate_square() {
        glRotatef(rotation_angle, 0.0f, 0.0f, 1.0f);
    }
    
    // Function to scale the square
    void scale_square() {
        glScalef(scale_factor, scale_factor, 1.0f);
    }
    
    // OpenGL initialization
    void init() {
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        gluOrtho2D(-window_width/2, window_width/2, -window_height/2, window_height/2);
    }
    
    // Display function
    void display() {
        glClear(GL_COLOR_BUFFER_BIT);
        glPointSize(1.0f);
        draw_square();
        rotate_square();
        scale_square();
        draw_square();
        glFlush();
    }
    
    void run() {
        glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
        glutInitWindowSize(window_width, window_height);
        glutCreateWindow("Square Scaling");
        glutDisplayFunc(display);
        init();
        glutMainLoop();
    }
}

// ========================== REFLECT ACROSS THE Y AXIS ==========================

namespace SquareReflection {
    // Square vertices
    float vertices[][2] = {{-0.5f, 0.5f}, {-0.5f, -0.5f}, {0.5f, -0.5f}, {0.5f, 0.5f}};
    
    // Window size
    const int window_width = 800;
    const int window_height = 600;
    
    // Function to draw the square
    void draw_square() {
        glBegin(GL_QUADS);
        glColor3f(1.0f, 0.0f, 0.0f);  // Red color
        for (int i = 0; i < 4; i++) {
            glVertex2f(vertices[i][0], vertices[i][1]);
        }
        glEnd();
    }
    
    // Function to reflect the square across the Y-axis
    void reflect_square() {
        glScalef(-1.0f, 1.0f, 1.0f);
    }
    
    // OpenGL initialization
    void init() {
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        gluOrtho2D(-window_width/2, window_width/2, -window_height/2, window_height/2);
    }
    
    // Display function
    void display() {
        glClear(GL_COLOR_BUFFER_BIT);
        glPointSize(1.0f);
        draw_square();
        reflect_square();
        draw_square();
        glFlush();
    }
    
    void run() {
        glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
        glutInitWindowSize(window_width, window_height);
        glutCreateWindow("Square Reflection");
        glutDisplayFunc(display);
        init();
        glutMainLoop();
    }
}

// ================================ MAIN FUNCTION ================================

int main(int argc, char** argv) {
    glutInit(&argc, argv);
    
    std::cout << "Select which program to run:" << std::endl;
    std::cout << "1. Square Drawing" << std::endl;
    std::cout << "2. Triangle Drawing" << std::endl;
    std::cout << "3. Square Scaling" << std::endl;
    std::cout << "4. Square Reflection" << std::endl;
    std::cout << "Enter choice (1-4): ";
    
    int choice;
    std::cin >> choice;
    
    switch(choice) {
        case 1:
            SquareDrawing::run();
            break;
        case 2:
            TriangleDrawing::run();
            break;
        case 3:
            SquareScaling::run();
            break;
        case 4:
            SquareReflection::run();
            break;
        default:
            std::cout << "Invalid choice. Running Square Drawing by default." << std::endl;
            SquareDrawing::run();
            break;
    }
    
    return 0;
}