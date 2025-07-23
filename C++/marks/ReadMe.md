# Mark Analyzer

A C++ console application that analyzes student marks and provides insights into class performance and teaching effectiveness.

## Overview

This program allows educators to input student marks, automatically calculates statistics, and provides feedback on both student performance and teaching quality based on predefined criteria.

## Features

- **Input Validation**: Only accepts marks between 0-100
- **Flexible Input**: Continue entering marks until finished (sentinel-controlled)
- **Automatic Statistics**: Calculates average scores and failure rates
- **Performance Analysis**: Provides recommendations based on class performance
- **User-Friendly Interface**: Clear prompts and visual feedback with emojis

## How It Works

### Input Phase
1. The program prompts for student marks one by one
2. Each mark must be between 0-100 (invalid marks are rejected)
3. Marks below 40 are flagged as failures
4. Enter `-1` to finish inputting marks

### Processing
- Calculates the class average
- Counts total failures (marks < 40)
- Computes failure rate as a percentage

### Analysis & Output
The program provides different feedback based on performance metrics:

#### Teaching Quality Assessment
- **Average < 45**: Suggests lecturer training on teaching methodology
- **Failure rate > 50%**: Indicates poor teaching performance
- **Average ≥ 70**: Celebrates excellent class performance
- **Average ≥ 45 AND failure rate ≤ 50%**: Acknowledges satisfactory teaching

## Sample Usage

```
=== MARK ANALYZER ===
Enter student marks (0-100)
Enter -1 when finished
Invalid marks will be ignored
=====================
Enter mark 1: 85
✓ Recorded (PASS)
Enter mark 2: 32
✓ Recorded (FAIL)
Enter mark 3: 78
✓ Recorded (PASS)
Enter mark 4: -1

📊 STATISTICS
=============
Students: 3
Average: 65.00
Failures: 1 (33.3%)

📝 ANALYSIS
===========
👍 Satisfactory teaching performance!
```

## Compilation & Running

### Prerequisites
- C++ compiler (g++, clang++, Visual Studio, etc.)
- Standard C++ libraries support

### Compile
```bash
g++ -o marks marks.cpp
```

### Run
```bash
./marks
```

## Technical Details

### Key Components

- **Sentinel Loop**: Uses `-1` as a termination signal
- **Input Validation**: Rejects marks outside 0-100 range
- **Type Safety**: Proper casting for floating-point calculations
- **Formatted Output**: Uses `setprecision()` for clean decimal display

### Error Handling

- Invalid marks (< 0 or > 100) are ignored with warning messages
- Program exits gracefully if no valid marks are entered
- Robust input handling prevents crashes from unexpected input

### Performance Criteria

| Metric | Threshold | Action |
|--------|-----------|---------|
| Class Average | < 45 | Recommend lecturer training |
| Failure Rate | > 50% | Flag poor teaching |
| Class Average | ≥ 70 | Celebrate excellent performance |
| Combined Good Performance | Average ≥ 45 AND Failures ≤ 50% | Acknowledge satisfactory teaching |

## Code Structure

```cpp
main()
├── Input Loop (sentinel-controlled)
│   ├── Input validation
│   ├── Mark processing
│   └── Pass/Fail classification
├── Statistics Calculation
│   ├── Average computation
│   └── Failure rate calculation
└── Analysis & Reporting
    ├── Performance statistics
    └── Teaching recommendations
```

## Potential Enhancements

- Save results to file
- Support for multiple classes/subjects
- Grade distribution analysis
- Graphical output of statistics
- Custom pass/fail thresholds
- Student name tracking

## License

This is educational software provided as-is for learning purposes.