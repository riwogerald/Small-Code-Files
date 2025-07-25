# Memory Game

A console-based memory game implemented in C++ where players must memorize and recall sequences of numbers that grow progressively longer with each round.

## Overview

This program implements a classic memory game where players are shown a sequence of random numbers for a brief period, then must reproduce the sequence from memory. The game becomes increasingly challenging as each round adds another number to the sequence.

## Features

- **Progressive Difficulty**: Each round adds one more number to the sequence
- **Random Number Generation**: Uses modern C++ random number generation
- **Timed Display**: Sequences are shown for exactly 3 seconds
- **Score Tracking**: Tracks the player's progress through rounds
- **Clean Console Interface**: Clear display and input prompts
- **Immediate Feedback**: Game ends immediately on incorrect input

## Game Mechanics

### Sequence Generation
- Numbers range from 0-9
- Each round adds one new random number to the end
- Sequence starts with 1 number in round 1
- Maximum sequence length is limited only by player ability

### Display System
- Sequence is displayed for 3 seconds
- Screen clears automatically after display time
- Player must then input the complete sequence

### Scoring System
- Score equals the number of successfully completed rounds
- Game tracks highest achieved round
- Final score displayed when game ends

## Technical Specifications

- **Language**: C++11 or later
- **Random Engine**: Mersenne Twister (std::mt19937)
- **Threading**: std::thread for timing control
- **Data Structure**: std::vector for sequence storage
- **Standard Libraries**: `<iostream>`, `<vector>`, `<random>`, `<chrono>`, `<thread>`

## Prerequisites

- C++ compiler with C++11 support or later
- Standard threading library support

## Compilation

```bash
g++ -std=c++11 -pthread memory_game.cpp -o memory_game
```

For systems requiring explicit threading support:
```bash
g++ -std=c++11 -pthread memory_game.cpp -o memory_game
```

## Usage

1. Compile the program
2. Run the executable:
   ```bash
   ./memory_game
   ```
3. Follow the game prompts:
   - Watch the sequence display for 3 seconds
   - Enter the numbers in correct order when prompted
   - Continue until you make a mistake

## Sample Gameplay

```
Welcome to the Memory Game!

Round 1
Memorize this sequence: 7 
Enter the sequence: 7

Round 2
Memorize this sequence: 7 3 
Enter the sequence: 7 3

Round 3
Memorize this sequence: 7 3 9 
Enter the sequence: 7 3 8
Game Over! Your final score is 2.
```

## Class Structure

### MemoryGame Class
```cpp
class MemoryGame {
private:
    std::vector<int> sequence;     // Stores the current sequence
    std::mt19937 rng;             // Random number generator
    int score;                    // Current player score
    
public:
    void start();                 // Main game loop
    
private:
    void addRandomNumber();       // Adds random number to sequence
    void displaySequence();       // Shows sequence for 3 seconds
    bool playerTurn();           // Handles player input and validation
};
```

## Key Methods

- **`start()`**: Main game loop managing rounds and scoring
- **`addRandomNumber()`**: Generates and appends random digit (0-9)
- **`displaySequence()`**: Displays sequence with timing control
- **`playerTurn()`**: Captures and validates player input

## Algorithm Details

### Random Number Generation
1. Initialize Mersenne Twister with hardware random seed
2. Use uniform distribution for digits 0-9
3. Append new random number each round

### Sequence Display
1. Print sequence with spaces between numbers
2. Use carriage return to stay on same line
3. Sleep for exactly 3 seconds using std::chrono
4. Clear display by overwriting with spaces

### Input Validation
1. Read player input number by number
2. Compare each input against corresponding sequence element
3. Return false immediately on first mismatch
4. Return true only if entire sequence matches

## Educational Value

This program demonstrates:

### Modern C++ Features
- Smart random number generation with proper seeding
- STL container usage (vector)
- Chrono library for precise timing
- Threading for non-blocking delays

### Game Programming Concepts
- Game loop design
- Progressive difficulty scaling
- User input handling
- State management

### Memory and Cognitive Skills
- Short-term memory training
- Pattern recognition
- Concentration improvement
- Sequential memory development

## Possible Extensions

### Enhanced Gameplay
- **Difficulty Levels**: Adjustable display time and number ranges
- **High Score System**: Persistent score tracking across sessions
- **Multiple Game Modes**: 
  - Colors instead of numbers
  - Audio sequences
  - Mixed character types (letters + numbers)

### Advanced Features
- **Timing Challenges**: Decreasing display time per round
- **Hint System**: Limited hints for struggling players
- **Statistics Tracking**: Average performance, improvement trends
- **Multiplayer Mode**: Turn-based competition

### User Interface Improvements
- **Colorized Output**: Different colors for different elements
- **Better Visualization**: ASCII art or symbols
- **Sound Effects**: Audio feedback for correct/incorrect answers
- **Configuration Menu**: Customizable game parameters

### Technical Enhancements
- **File I/O**: Save/load high scores and settings
- **Command Line Arguments**: Configure game parameters at startup
- **Error Handling**: Robust input validation and error recovery
- **Platform Independence**: Cross-platform compatibility improvements

## Learning Objectives

Perfect for understanding:
- Modern C++ random number generation
- STL container manipulation
- Threading and timing concepts
- Object-oriented game design
- User input handling and validation
- Game loop architecture

## Performance Considerations

- **Memory Usage**: O(n) where n is current round number
- **Time Complexity**: O(n) for input validation per round
- **Threading**: Minimal overhead from sleep operations
- **Scalability**: Can handle very long sequences limited only by memory

## Cognitive Benefits

Playing this game can help improve:
- **Working Memory**: Holding sequences in short-term memory
- **Attention Span**: Maintaining focus during sequence display
- **Pattern Recognition**: Identifying sequence patterns over time
- **Mental Discipline**: Concentration under time pressure

## Difficulty Progression

The game naturally becomes more challenging:
- **Round 1-3**: Easy warm-up (1-3 digits)
- **Round 4-7**: Moderate challenge (4-7 digits)
- **Round 8+**: Difficult (approaching typical digit span limits)

Most players find significant challenge around 7-9 digits, which aligns with psychological research on human digit span capacity.

This memory game serves as both an entertaining challenge and a practical example of C++ programming concepts, making it ideal for learning game development fundamentals while exercising cognitive abilities.
