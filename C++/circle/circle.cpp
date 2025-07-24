#include <GL/glut.h>
#include <iostream>

// Circle parameters
int center_x = 3;
int center_y = 3;
int radius = 4;

// Window size
int window_width = 800;
int window_height = 600;

// Current algorithm selector (0 = Mid-point, 1 = Bresenham's)
int current_algorithm = 0;

// Function to draw symmetric points of the circle
void draw_symmetric_points(int x, int y) {
    glVertex2f(center_x + x, center_y + y);
    glVertex2f(center_x - x, center_y + y);
    glVertex2f(center_x + x, center_y - y);
    glVertex2f(center_x - x, center_y - y);
    glVertex2f(center_x + y, center_y + x);
    glVertex2f(center_x - y, center_y + x);
    glVertex2f(center_x + y, center_y - x);
    glVertex2f(center_x - y, center_y - x);
}

// Mid-point circle drawing algorithm
void draw_circle_midpoint() {
    glBegin(GL_POINTS);
    glColor3f(1.0f, 0.0f, 0.0f);  // Red color
    
    int x = 0;
    int y = radius;
    int d = 1 - radius;
    
    draw_symmetric_points(x, y);
    
    while (y > x) {
        if (d < 0) {
            d += 2 * x + 3;
        } else {
            d += 2 * (x - y) + 5;
            y--;
        }
        x++;
        draw_symmetric_points(x, y);
    }
    
    glEnd();
}

// Bresenham's circle drawing algorithm
void draw_circle_bresenham() {
    glBegin(GL_POINTS);
    glColor3f(0.0f, 1.0f, 0.0f);  // Green color
    
    int x = 0;
    int y = radius;
    int d = 3 - 2 * radius;
    
    draw_symmetric_points(x, y);
    
    while (y >= x) {
        if (d <= 0) {
            d += 4 * x + 6;
        } else {
            d += 4 * (x - y) + 10;
            y--;
        }
        x++;
        draw_symmetric_points(x, y);
    }
    
    glEnd();
}

// OpenGL initialization
void init() {
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    gluOrtho2D(0, window_width, 0, window_height);
}

// Display function
void display() {
    glClear(GL_COLOR_BUFFER_BIT);
    glPointSize(2.0f);
    
    if (current_algorithm == 0) {
        draw_circle_midpoint();
    } else {
        draw_circle_bresenham();
    }
    
    glFlush();
}

// Keyboard callback function
void keyboard(unsigned char key, int x, int y) {
    switch (key) {
        case '1':
            current_algorithm = 0;
            std::cout << "Switched to Mid-point Circle Algorithm (Red)" << std::endl;
            glutPostRedisplay();
            break;
        case '2':
            current_algorithm = 1;
            std::cout << "Switched to Bresenham's Circle Algorithm (Green)" << std::endl;
            glutPostRedisplay();
            break;
        case '+':
            if (radius < 50) {
                radius++;
                std::cout << "Radius increased to: " << radius << std::endl;
                glutPostRedisplay();
            }
            break;
        case '-':
            if (radius > 1) {
                radius--;
                std::cout << "Radius decreased to: " << radius << std::endl;
                glutPostRedisplay();
            }
            break;
        case 'q':
        case 'Q':
        case 27: // ESC key
            exit(0);
            break;
    }
}

// Mouse callback function
void mouse(int button, int state, int x, int y) {
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN) {
        // Convert window coordinates to OpenGL coordinates
        center_x = x;
        center_y = window_height - y;
        std::cout << "Center moved to: (" << center_x << ", " << center_y << ")" << std::endl;
        glutPostRedisplay();
    }
}

// Function to print instructions
void print_instructions() {
    std::cout << "=== Circle Drawing Algorithms ===" << std::endl;
    std::cout << "Controls:" << std::endl;
    std::cout << "  1 - Mid-point Circle Algorithm (Red)" << std::endl;
    std::cout << "  2 - Bresenham's Circle Algorithm (Green)" << std::endl;
    std::cout << "  + - Increase radius" << std::endl;
    std::cout << "  - - Decrease radius" << std::endl;
    std::cout << "  Left Click - Move circle center" << std::endl;
    std::cout << "  Q or ESC - Quit" << std::endl;
    std::cout << "=================================" << std::endl;
}

// Main function
int main(int argc, char** argv) {
    print_instructions();
    
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowSize(window_width, window_height);
    glutInitWindowPosition(100, 100);
    glutCreateWindow("Circle Drawing Algorithms - C++");
    
    glutDisplayFunc(display);
    glutKeyboardFunc(keyboard);
    glutMouseFunc(mouse);
    
    init();
    
    std::cout << "Starting with Mid-point Circle Algorithm (Red)" << std::endl;
    
    glutMainLoop();
    
    return 0;
}