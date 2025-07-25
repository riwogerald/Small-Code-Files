# Binary Tree Visualizer

A C++ program that implements a binary search tree data structure with level-order traversal visualization.

## Overview

This program demonstrates the implementation of a binary search tree (BST) with insertion functionality and level-by-level visualization. It's designed to help understand tree data structures and their traversal algorithms.

## Features

- **Binary Search Tree Implementation**: Complete BST with insertion capability
- **Level-Order Traversal**: Displays tree structure level by level (breadth-first)
- **Automatic Tree Building**: Constructs a sample tree with predefined values
- **Clean Console Output**: Organized display showing tree levels clearly

## Data Structure Details

### TreeNode Structure
```cpp
struct TreeNode {
    int value;         // Node data
    TreeNode* left;    // Pointer to left child
    TreeNode* right;   // Pointer to right child
};
```

### Binary Tree Class
- **Insertion**: Maintains BST property (left < parent < right)
- **Level-Order Display**: Uses queue-based breadth-first traversal
- **Recursive Insertion**: Helper function for proper node placement

## Sample Output

For the input values `{15, 10, 20, 8, 12, 17, 25}`, the program outputs:
```
Level Order Traversal of the Binary Tree:
15 
10 20 
8 12 17 25 
```

This represents the tree structure:
```
       15
      /  \
    10    20
   / \   / \
  8  12 17 25
```

## Technical Specifications

- **Language**: C++
- **Data Structure**: Binary Search Tree
- **Traversal Method**: Level-order (breadth-first)
- **Memory Management**: Dynamic allocation using pointers
- **Standard Libraries**: `<iostream>`, `<queue>`

## Prerequisites

- C++ compiler (g++, clang++, MSVC)
- Standard C++ library support

## Compilation

```bash
g++ binary_tree_visualizer.cpp -o binary_tree_visualizer
```

With additional flags:
```bash
g++ -std=c++11 -Wall binary_tree_visualizer.cpp -o binary_tree_visualizer
```

## Usage

1. Compile the program
2. Run the executable:
   ```bash
   ./binary_tree_visualizer
   ```
3. View the level-order traversal output

## Algorithm Details

### Insertion Algorithm
1. If tree is empty, create root node
2. If value < current node value, go left
3. If value ≥ current node value, go right
4. Recursively find appropriate position
5. Insert new node as leaf

### Level-Order Traversal Algorithm
1. Use a queue to store nodes to visit
2. Start with root node in queue
3. For each level:
   - Process all nodes currently in queue
   - Add their children to queue
   - Print node values
4. Continue until queue is empty

## Key Concepts Demonstrated

### Data Structures
- **Binary Search Tree**: Ordered tree structure
- **Queue**: FIFO structure for level-order traversal
- **Pointers**: Dynamic memory management
- **Recursive Structures**: Self-referencing data types

### Algorithms
- **Tree Insertion**: Maintaining BST property
- **Breadth-First Search**: Level-order traversal
- **Recursive Programming**: Helper functions

## Educational Value

This program is excellent for learning:

### Computer Science Fundamentals
- Tree data structures
- Binary search tree properties
- Graph traversal algorithms
- Queue-based algorithms

### C++ Programming Concepts
- Struct and class definitions
- Pointer manipulation
- Dynamic memory allocation
- STL container usage (queue)
- Recursive function design

### Algorithm Analysis
- Time complexity of BST operations
- Space complexity of traversal algorithms
- Understanding tree height and balance

## Possible Extensions

### Enhanced Functionality
- **Additional Traversals**: In-order, pre-order, post-order
- **Search Function**: Find specific values in tree
- **Deletion**: Remove nodes while maintaining BST property
- **Tree Statistics**: Height, node count, balance factor

### Visualization Improvements
- **ASCII Art Display**: Visual tree representation
- **Tree Depth Indication**: Show level numbers
- **Color-coded Output**: Different colors for different levels
- **Interactive Mode**: Allow user input for tree building

### Advanced Features
- **Self-Balancing**: AVL or Red-Black tree implementation
- **Multiple Data Types**: Template-based generic tree
- **File I/O**: Save/load tree structures
- **Performance Metrics**: Measure insertion and traversal times

## Learning Objectives

Perfect for understanding:
- Binary tree concepts and terminology
- Tree traversal algorithms
- Queue-based algorithms
- Recursive data structure manipulation
- Memory management with pointers
- Object-oriented design in C++

## Memory Considerations

**Note**: This implementation does not include a destructor or memory cleanup. For production code, consider:
- Implementing destructor to free allocated memory
- Using smart pointers for automatic memory management
- Adding copy constructor and assignment operator

## Tree Properties

The sample tree demonstrates:
- **BST Property**: Left subtree < root < right subtree
- **Complete Levels**: First two levels are completely filled
- **Balanced Structure**: Relatively even distribution of nodes
- **Height**: 3 levels (0-indexed)

This program serves as an excellent introduction to tree data structures and provides a foundation for more advanced tree algorithms and data structure implementations.
