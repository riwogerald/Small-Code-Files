#include <iostream>
#include <string>
#include <map>
#include <algorithm>
#include <cctype>
#include <sstream>
#include <vector>
#include <iomanip>

class TextAnalyzer {
private:
    std::string text;
    std::map<char, int> charFrequency;
    std::map<std::string, int> wordFrequency;
    
public:
    void setText(const std::string& inputText) {
        text = inputText;
        analyzeText();
    }
    
    void analyzeText() {
        charFrequency.clear();
        wordFrequency.clear();
        
        // Character frequency analysis
        for (char c : text) {
            if (std::isalnum(c) || std::isspace(c)) {
                charFrequency[std::tolower(c)]++;
            }
        }
        
        // Word frequency analysis
        std::stringstream ss(text);
        std::string word;
        while (ss >> word) {
            // Clean word (remove punctuation)
            std::string cleanWord;
            for (char c : word) {
                if (std::isalnum(c)) {
                    cleanWord += std::tolower(c);
                }
            }
            if (!cleanWord.empty()) {
                wordFrequency[cleanWord]++;
            }
        }
    }
    
    void displayBasicStats() const {
        std::cout << "\n=== BASIC TEXT STATISTICS ===" << std::endl;
        std::cout << "Total characters: " << text.length() << std::endl;
        std::cout << "Total words: " << countWords() << std::endl;
        std::cout << "Total sentences: " << countSentences() << std::endl;
        std::cout << "Total paragraphs: " << countParagraphs() << std::endl;
        std::cout << "Alphabetic characters: " << countAlphabetic() << std::endl;
        std::cout << "Numeric characters: " << countNumeric() << std::endl;
        std::cout << "Whitespace characters: " << countWhitespace() << std::endl;
        std::cout << "Punctuation marks: " << countPunctuation() << std::endl;
    }
    
    void displayCharacterFrequency() const {
        std::cout << "\n=== CHARACTER FREQUENCY ANALYSIS ===" << std::endl;
        
        // Sort by frequency (descending)
        std::vector<std::pair<char, int>> sortedChars(charFrequency.begin(), charFrequency.end());
        sort(sortedChars.begin(), sortedChars.end(), 
             [](const std::pair<char, int>& a, const std::pair<char, int>& b) {
                 return a.second > b.second;
             });
        
        std::cout << std::left << std::setw(12) << "Character" << "Frequency" << std::endl;
        std::cout << std::string(20, '-') << std::endl;
        
        for (const auto& pair : sortedChars) {
            if (pair.first == ' ') {
                std::cout << std::left << std::setw(12) << "[space]" << pair.second << std::endl;
            } else {
                std::cout << std::left << std::setw(12) << pair.first << pair.second << std::endl;
            }
        }
    }
    
    void displayWordFrequency(int topN = 10) const {
        std::cout << "\n=== TOP " << topN << " MOST FREQUENT WORDS ===" << std::endl;
        
        // Sort by frequency (descending)
        std::vector<std::pair<std::string, int>> sortedWords(wordFrequency.begin(), wordFrequency.end());
        sort(sortedWords.begin(), sortedWords.end(), 
             [](const std::pair<std::string, int>& a, const std::pair<std::string, int>& b) {
                 return a.second > b.second;
             });
        
        std::cout << std::left << std::setw(15) << "Word" << "Frequency" << std::endl;
        std::cout << std::string(25, '-') << std::endl;
        
        int count = 0;
        for (const auto& pair : sortedWords) {
            if (count >= topN) break;
            std::cout << std::left << std::setw(15) << pair.first << pair.second << std::endl;
            count++;
        }
    }
    
    void displayReadabilityMetrics() const {
        std::cout << "\n=== READABILITY METRICS ===" << std::endl;
        
        int words = countWords();
        int sentences = countSentences();
        int syllables = estimateSyllables();
        
        if (sentences > 0 && words > 0) {
            double avgWordsPerSentence = static_cast<double>(words) / sentences;
            double avgSyllablesPerWord = static_cast<double>(syllables) / words;
            
            // Flesch Reading Ease Score
            double fleschScore = 206.835 - (1.015 * avgWordsPerSentence) - (84.6 * avgSyllablesPerWord);
            
            std::cout << "Average words per sentence: " << std::fixed << std::setprecision(2) 
                      << avgWordsPerSentence << std::endl;
            std::cout << "Average syllables per word: " << std::fixed << std::setprecision(2) 
                      << avgSyllablesPerWord << std::endl;
            std::cout << "Flesch Reading Ease Score: " << std::fixed << std::setprecision(1) 
                      << fleschScore << " (" << getReadingLevel(fleschScore) << ")" << std::endl;
        }
    }
    
    void searchText(const std::string& searchTerm) const {
        std::cout << "\n=== SEARCH RESULTS FOR: \"" << searchTerm << "\" ===" << std::endl;
        
        std::string lowerText = text;
        std::string lowerSearch = searchTerm;
        std::transform(lowerText.begin(), lowerText.end(), lowerText.begin(), ::tolower);
        std::transform(lowerSearch.begin(), lowerSearch.end(), lowerSearch.begin(), ::tolower);
        
        size_t pos = 0;
        int count = 0;
        std::vector<size_t> positions;
        
        while ((pos = lowerText.find(lowerSearch, pos)) != std::string::npos) {
            positions.push_back(pos);
            count++;
            pos += lowerSearch.length();
        }
        
        std::cout << "Found " << count << " occurrence(s)" << std::endl;
        
        if (count > 0) {
            std::cout << "Positions: ";
            for (size_t i = 0; i < positions.size(); ++i) {
                std::cout << positions[i];
                if (i < positions.size() - 1) std::cout << ", ";
            }
            std::cout << std::endl;
            
            // Show context for first few occurrences
            int contextSize = 30;
            for (size_t i = 0; i < std::min(positions.size(), size_t(3)); ++i) {
                size_t start = (positions[i] >= contextSize) ? positions[i] - contextSize : 0;
                size_t end = std::min(positions[i] + searchTerm.length() + contextSize, text.length());
                
                std::cout << "Context " << (i + 1) << ": \"..." 
                          << text.substr(start, end - start) << "...\"" << std::endl;
            }
        }
    }

private:
    int countWords() const {
        std::stringstream ss(text);
        std::string word;
        int count = 0;
        while (ss >> word) count++;
        return count;
    }
    
    int countSentences() const {
        int count = 0;
        for (char c : text) {
            if (c == '.' || c == '!' || c == '?') count++;
        }
        return count;
    }
    
    int countParagraphs() const {
        int count = 1;
        for (size_t i = 0; i < text.length() - 1; ++i) {
            if (text[i] == '\n' && text[i + 1] == '\n') count++;
        }
        return count;
    }
    
    int countAlphabetic() const {
        int count = 0;
        for (char c : text) {
            if (std::isalpha(c)) count++;
        }
        return count;
    }
    
    int countNumeric() const {
        int count = 0;
        for (char c : text) {
            if (std::isdigit(c)) count++;
        }
        return count;
    }
    
    int countWhitespace() const {
        int count = 0;
        for (char c : text) {
            if (std::isspace(c)) count++;
        }
        return count;
    }
    
    int countPunctuation() const {
        int count = 0;
        for (char c : text) {
            if (std::ispunct(c)) count++;
        }
        return count;
    }
    
    int estimateSyllables() const {
        int syllables = 0;
        std::stringstream ss(text);
        std::string word;
        
        while (ss >> word) {
            int wordSyllables = 0;
            bool prevVowel = false;
            
            for (char c : word) {
                char lower = std::tolower(c);
                if (lower == 'a' || lower == 'e' || lower == 'i' || 
                    lower == 'o' || lower == 'u' || lower == 'y') {
                    if (!prevVowel) wordSyllables++;
                    prevVowel = true;
                } else {
                    prevVowel = false;
                }
            }
            
            // Adjust for silent 'e'
            if (word.length() > 0 && std::tolower(word.back()) == 'e') {
                wordSyllables--;
            }
            
            // Every word has at least one syllable
            if (wordSyllables <= 0) wordSyllables = 1;
            
            syllables += wordSyllables;
        }
        
        return syllables;
    }
    
    std::string getReadingLevel(double fleschScore) const {
        if (fleschScore >= 90) return "Very Easy";
        else if (fleschScore >= 80) return "Easy";
        else if (fleschScore >= 70) return "Fairly Easy";
        else if (fleschScore >= 60) return "Standard";
        else if (fleschScore >= 50) return "Fairly Difficult";
        else if (fleschScore >= 30) return "Difficult";
        else return "Very Difficult";
    }
};

void displayMenu() {
    std::cout << "\n========================================" << std::endl;
    std::cout << "         TEXT ANALYZER TOOL" << std::endl;
    std::cout << "========================================" << std::endl;
    std::cout << "1. Enter/Change Text" << std::endl;
    std::cout << "2. Display Basic Statistics" << std::endl;
    std::cout << "3. Show Character Frequency" << std::endl;
    std::cout << "4. Show Word Frequency" << std::endl;
    std::cout << "5. Display Readability Metrics" << std::endl;
    std::cout << "6. Search Text" << std::endl;
    std::cout << "7. Complete Analysis Report" << std::endl;
    std::cout << "8. Exit" << std::endl;
    std::cout << "========================================" << std::endl;
    std::cout << "Choose an option (1-8): ";
}

int main() {
    TextAnalyzer analyzer;
    int choice;
    std::string inputText, searchTerm;
    
    std::cout << "Welcome to the Advanced Text Analyzer!" << std::endl;
    
    while (true) {
        displayMenu();
        std::cin >> choice;
        std::cin.ignore(); // Clear the input buffer
        
        switch (choice) {
            case 1:
                std::cout << "\nEnter the text to analyze (end with a line containing only 'END'):" << std::endl;
                inputText.clear();
                {
                    std::string line;
                    while (std::getline(std::cin, line) && line != "END") {
                        inputText += line + "\n";
                    }
                }
                analyzer.setText(inputText);
                std::cout << "Text has been loaded and analyzed!" << std::endl;
                break;
                
            case 2:
                analyzer.displayBasicStats();
                break;
                
            case 3:
                analyzer.displayCharacterFrequency();
                break;
                
            case 4: {
                int topN;
                std::cout << "How many top words to display? (default 10): ";
                std::cin >> topN;
                if (topN <= 0) topN = 10;
                analyzer.displayWordFrequency(topN);
                break;
            }
            
            case 5:
                analyzer.displayReadabilityMetrics();
                break;
                
            case 6:
                std::cout << "Enter search term: ";
                std::getline(std::cin, searchTerm);
                analyzer.searchText(searchTerm);
                break;
                
            case 7:
                std::cout << "\n" << std::string(50, '=') << std::endl;
                std::cout << "           COMPLETE ANALYSIS REPORT" << std::endl;
                std::cout << std::string(50, '=') << std::endl;
                analyzer.displayBasicStats();
                analyzer.displayReadabilityMetrics();
                analyzer.displayWordFrequency(5);
                analyzer.displayCharacterFrequency();
                break;
                
            case 8:
                std::cout << "Thank you for using Text Analyzer!" << std::endl;
                return 0;
                
            default:
                std::cout << "Invalid choice! Please try again." << std::endl;
                break;
        }
        
        std::cout << "\nPress Enter to continue...";
        std::cin.get();
    }
    
    return 0;
}
