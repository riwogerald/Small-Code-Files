#include <GL/glut.h>
#include <cmath>
#include <iostream>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

class EllipseRenderer {
private:
    float centerX, centerY;
    float semiMajorAxis, semiMinorAxis;
    
public:
    // Constructor
    EllipseRenderer(float cx = -2.0f, float cy = 2.0f, float a = 6.0f, float b = 5.0f)
        : centerX(cx), centerY(cy), semiMajorAxis(a), semiMinorAxis(b) {}
    
    // Method to draw the ellipse
    void drawEllipse() const {
        glBegin(GL_LINE_LOOP);
        for (int i = 0; i < 360; ++i) {
            float theta = static_cast<float>(i) * M_PI / 180.0f;
            float x = centerX + semiMajorAxis * std::cos(theta);
            float y = centerY + semiMinorAxis * std::sin(theta);
            glVertex2f(x, y);
        }
        glEnd();
    }
    
    // Setters for ellipse parameters
    void setCenter(float cx, float cy) {
        centerX = cx;
        centerY = cy;
    }
    
    void setAxes(float a, float b) {
        semiMajorAxis = a;
        semiMinorAxis = b;
    }
    
    // Getters
    float getCenterX() const { return centerX; }
    float getCenterY() const { return centerY; }
    float getSemiMajorAxis() const { return semiMajorAxis; }
    float getSemiMinorAxis() const { return semiMinorAxis; }
};

// Global ellipse object
EllipseRenderer ellipse;

// OpenGL display function
void display() {
    glClear(GL_COLOR_BUFFER_BIT);
    
    glColor3f(1.0f, 1.0f, 1.0f); // Set color to white
    
    // Draw the ellipse
    ellipse.drawEllipse();
    
    glFlush();
}

// OpenGL initialization function
void init() {
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f); // Set clear color to black
    gluOrtho2D(-10.0, 10.0, -10.0, 10.0); // Set the coordinate system
    
    std::cout << "Ellipse Parameters:" << std::endl;
    std::cout << "Center: (" << ellipse.getCenterX() << ", " << ellipse.getCenterY() << ")" << std::endl;
    std::cout << "Semi-major axis: " << ellipse.getSemiMajorAxis() << std::endl;
    std::cout << "Semi-minor axis: " << ellipse.getSemiMinorAxis() << std::endl;
}

// Keyboard callback for interaction
void keyboard(unsigned char key, int x, int y) {
    switch (key) {
        case 'q':
        case 'Q':
        case 27: // ESC key
            std::cout << "Exiting program..." << std::endl;
            exit(0);
            break;
        case 'r':
        case 'R':
            // Reset ellipse to default parameters
            ellipse.setCenter(-2.0f, 2.0f);
            ellipse.setAxes(6.0f, 5.0f);
            glutPostRedisplay();
            std::cout << "Ellipse reset to default parameters" << std::endl;
            break;
        default:
            break;
    }
}

// Main function
int main(int argc, char** argv) {
    try {
        // Initialize GLUT
        glutInit(&argc, argv);
        glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
        glutInitWindowSize(500, 500);
        glutInitWindowPosition(100, 100);
        glutCreateWindow("C++ Ellipse Renderer");
        
        // Set callback functions
        glutDisplayFunc(display);
        glutKeyboardFunc(keyboard);
        
        // Initialize OpenGL
        init();
        
        std::cout << "C++ Ellipse Renderer" << std::endl;
        std::cout << "Press 'R' to reset ellipse" << std::endl;
        std::cout << "Press 'Q' or ESC to quit" << std::endl;
        
        // Start the main loop
        glutMainLoop();
        
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    
    return 0;
}