# Advanced Text Analyzer

A comprehensive C++ console application for analyzing text with multiple features including statistics, frequency analysis, readability metrics, and search functionality.

## Overview

This program implements a sophisticated text analysis system that provides detailed insights into text composition, readability, and structure. It's designed for writers, researchers, students, and anyone interested in text analytics and linguistic analysis.

## Features

- **Basic Statistics**: Character, word, sentence, and paragraph counting
- **Character Frequency Analysis**: Distribution of characters with sorting capabilities
- **Word Frequency Analysis**: Most common words with customizable top-N display
- **Readability Metrics**: Flesch Reading Ease Score and reading level assessment
- **Text Search**: Case-insensitive search with context display
- **Complete Analysis Report**: Comprehensive overview of all metrics
- **Interactive Menu System**: User-friendly navigation through analysis options

## Technical Specifications

- **Language**: C++11 or later
- **Data Structures**: STL containers (map, vector, string)
- **Algorithms**: Sorting, searching, string processing
- **Standard Libraries**: `<iostream>`, `<string>`, `<map>`, `<algorithm>`, `<cctype>`, `<sstream>`, `<vector>`, `<iomanip>`

## Prerequisites

- C++ compiler with C++11 support or later
- Standard C++ library support

## Compilation

```bash
g++ -std=c++11 text_analyzer.cpp -o text_analyzer
```

With additional optimization and warnings:
```bash
g++ -std=c++11 -O2 -Wall -Wextra text_analyzer.cpp -o text_analyzer
```

## Usage

1. Compile and run the program:
   ```bash
   ./text_analyzer
   ```

2. Choose from the menu options:
   - **Option 1**: Enter or change text for analysis
   - **Option 2**: View basic text statistics
   - **Option 3**: Show character frequency distribution
   - **Option 4**: Display word frequency analysis
   - **Option 5**: View readability metrics
   - **Option 6**: Search for specific terms
   - **Option 7**: Generate complete analysis report
   - **Option 8**: Exit the program

## Analysis Features

### Basic Statistics
- Total character count
- Word count
- Sentence count
- Paragraph count
- Alphabetic character count
- Numeric character count
- Whitespace character count
- Punctuation mark count

### Character Frequency Analysis
- Frequency distribution of all characters
- Sorted by frequency (descending order)
- Special handling for spaces and non-printable characters
- Case-insensitive analysis

### Word Frequency Analysis
- Most frequently used words
- Customizable top-N display (default: 10)
- Automatic punctuation removal
- Case-insensitive word matching
- Sorted by frequency (descending order)

### Readability Metrics
- **Average words per sentence**
- **Average syllables per word**
- **Flesch Reading Ease Score**: Industry-standard readability metric
- **Reading Level Classification**:
  - Very Easy (90-100)
  - Easy (80-89)
  - Fairly Easy (70-79)
  - Standard (60-69)
  - Fairly Difficult (50-59)
  - Difficult (30-49)
  - Very Difficult (0-29)

### Text Search Functionality
- Case-insensitive search
- Multiple occurrence detection
- Position reporting
- Context display (30 characters before/after)
- Shows up to 3 contexts for found terms

## Sample Output

### Basic Statistics Example
```
=== BASIC TEXT STATISTICS ===
Total characters: 1250
Total words: 245
Total sentences: 12
Total paragraphs: 3
Alphabetic characters: 1020
Numeric characters: 15
Whitespace characters: 180
Punctuation marks: 35
```

### Word Frequency Example
```
=== TOP 10 MOST FREQUENT WORDS ===
Word            Frequency
-------------------------
the             15
and             12
of              10
to              8
in              7
a               6
is              5
that            4
for             4
with            3
```

### Readability Metrics Example
```
=== READABILITY METRICS ===
Average words per sentence: 20.42
Average syllables per word: 1.65
Flesch Reading Ease Score: 68.3 (Standard)
```

## Class Structure

### TextAnalyzer Class
```cpp
class TextAnalyzer {
private:
    std::string text;                        // Input text
    std::map<char, int> charFrequency;       // Character frequency map
    std::map<std::string, int> wordFrequency; // Word frequency map
    
public:
    void setText(const std::string& inputText);
    void analyzeText();
    void displayBasicStats() const;
    void displayCharacterFrequency() const;
    void displayWordFrequency(int topN = 10) const;
    void displayReadabilityMetrics() const;
    void searchText(const std::string& searchTerm) const;
};
```

## Key Algorithms

### Syllable Estimation Algorithm
1. Count vowel groups (consecutive vowels count as one)
2. Subtract 1 for silent 'e' at word end
3. Ensure minimum of 1 syllable per word
4. Sum syllables across all words

### Flesch Reading Ease Formula
```
Score = 206.835 - (1.015 × avg_words_per_sentence) - (84.6 × avg_syllables_per_word)
```

### Word Cleaning Algorithm
1. Extract words from text using stringstream
2. Remove punctuation characters
3. Convert to lowercase
4. Store non-empty cleaned words

## Educational Value

This program demonstrates:

### String Processing Concepts
- Text parsing and tokenization
- Character and string manipulation
- Case conversion and comparison
- Pattern searching and matching

### Data Structure Usage
- STL map for frequency counting
- STL vector for sorting operations
- Pair structures for key-value storage
- String stream processing

### Algorithm Implementation
- Sorting algorithms with custom comparators
- Frequency analysis algorithms
- Statistical calculation methods
- Search algorithms with context

### C++ Programming Techniques
- Object-oriented design patterns
- STL container manipulation
- Lambda functions for sorting
- Stream manipulation and formatting

## Possible Extensions

### Enhanced Analytics
- **Sentiment Analysis**: Detect positive/negative sentiment
- **Keyword Density**: Calculate keyword frequency percentages
- **Text Complexity**: Additional readability formulas (Gunning Fog, SMOG)
- **Language Detection**: Identify text language

### Advanced Features
- **File I/O Support**: Load/save text from/to files
- **Batch Processing**: Analyze multiple texts simultaneously
- **Export Functionality**: Generate reports in CSV/JSON format
- **Comparison Mode**: Compare statistics between different texts

### User Interface Improvements
- **Colorized Output**: Different colors for different analysis sections
- **Progress Bars**: Show progress for large text analysis
- **Configuration Options**: Customizable analysis parameters
- **Help System**: Built-in help and usage examples

### Statistical Enhancements
- **Standard Deviation**: Measure variability in sentence/word lengths
- **Percentile Analysis**: Distribution analysis of words and sentences
- **Correlation Analysis**: Relationships between different metrics
- **Trend Analysis**: Changes in metrics across text sections

## Learning Objectives

Perfect for understanding:
- Advanced string processing techniques
- Statistical analysis implementation
- Map and vector data structure usage
- Algorithm design and implementation
- Object-oriented programming principles
- Menu-driven application architecture
- Text analytics and computational linguistics

## Performance Considerations

- **Time Complexity**: O(n) for most operations where n is text length
- **Space Complexity**: O(k) where k is unique characters/words
- **Memory Usage**: Efficient storage using STL containers
- **Scalability**: Suitable for texts up to several megabytes

## Real-World Applications

This tool can be used for:
- **Academic Writing**: Analyzing research papers and essays
- **Content Creation**: Optimizing blog posts and articles for readability
- **Educational Assessment**: Evaluating text complexity for different grade levels
- **SEO Analysis**: Keyword density and content optimization
- **Literary Analysis**: Studying writing patterns and styles

This comprehensive text analyzer serves as an excellent example of practical C++ programming applied to real-world text processing challenges, making it ideal for both learning advanced programming concepts and performing actual text analysis tasks.
