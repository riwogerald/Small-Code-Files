#include <GL/glut.h>
#include <GL/gl.h>
#include <GL/glu.h>

// Circle parameters
const int center_x = 3;
const int center_y = 3;
const int radius = 4;

// Window size
const int window_width = 800;
const int window_height = 600;

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

// Function to draw the circle using midpoint circle algorithm
void draw_circle() {
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

// OpenGL initialization
void init() {
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    gluOrtho2D(0, window_width, 0, window_height);
}

// Display function
void display() {
    glClear(GL_COLOR_BUFFER_BIT);
    glPointSize(1.0f);
    draw_circle();
    glFlush();
}

// Main function
int main(int argc, char** argv) {
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowSize(window_width, window_height);
    glutCreateWindow("Circle Drawing");
    glutDisplayFunc(display);
    init();
    glutMainLoop();
    return 0;
}