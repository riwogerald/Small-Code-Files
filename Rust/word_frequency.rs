use std::collections::HashMap;
use std::io;

struct WordFrequencyAnalyzer {
    word_counts: HashMap<String, usize>,
    total_words: usize,
}

impl WordFrequencyAnalyzer {
    fn new() -> Self {
        WordFrequencyAnalyzer {
            word_counts: HashMap::new(),
            total_words: 0,
        }
    }

    fn analyze_text(&mut self, text: &str) {
        let words: Vec<String> = text
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect();

        self.total_words += words.len();

        for word in words {
            *self.word_counts.entry(word).or_insert(0) += 1;
        }
    }

    fn get_most_frequent(&self, n: usize) -> Vec<(&String, &usize)> {
        let mut word_freq: Vec<_> = self.word_counts.iter().collect();
        word_freq.sort_by(|a, b| b.1.cmp(a.1));
        word_freq.into_iter().take(n).collect()
    }

    fn get_word_frequency(&self, word: &str) -> usize {
        *self.word_counts.get(&word.to_lowercase()).unwrap_or(&0)
    }

    fn get_unique_words(&self) -> usize {
        self.word_counts.len()
    }

    fn get_words_with_frequency(&self, frequency: usize) -> Vec<&String> {
        self.word_counts
            .iter()
            .filter(|(_, &count)| count == frequency)
            .map(|(word, _)| word)
            .collect()
    }

    fn calculate_word_percentage(&self, word: &str) -> f64 {
        if self.total_words == 0 {
            return 0.0;
        }
        let count = self.get_word_frequency(word);
        (count as f64 / self.total_words as f64) * 100.0
    }

    fn display_statistics(&self) {
        println!("\n=== Text Analysis Statistics ===");
        println!("Total words: {}", self.total_words);
        println!("Unique words: {}", self.get_unique_words());
        
        if self.total_words > 0 {
            let vocabulary_richness = self.get_unique_words() as f64 / self.total_words as f64;
            println!("Vocabulary richness: {:.2}%", vocabulary_richness * 100.0);
        }

        // Show frequency distribution
        let mut freq_distribution: HashMap<usize, usize> = HashMap::new();
        for &count in self.word_counts.values() {
            *freq_distribution.entry(count).or_insert(0) += 1;
        }

        println!("\nFrequency Distribution:");
        let mut freq_vec: Vec<_> = freq_distribution.iter().collect();
        freq_vec.sort_by(|a, b| b.0.cmp(a.0));
        
        for (&frequency, &word_count) in freq_vec.iter().take(10) {
            println!("  {} words appear {} time(s)", word_count, frequency);
        }
    }

    fn display_top_words(&self, n: usize) {
        println!("\n=== Top {} Most Frequent Words ===", n);
        let top_words = self.get_most_frequent(n);
        
        for (i, (word, count)) in top_words.iter().enumerate() {
            let percentage = self.calculate_word_percentage(word);
            println!("{}. {} - {} occurrences ({:.1}%)", 
                     i + 1, word, count, percentage);
        }
    }
}

fn main() {
    println!("Word Frequency Analyzer");
    println!("Enter text to analyze (type 'END' on a new line to finish):");

    let mut analyzer = WordFrequencyAnalyzer::new();
    let mut text_input = String::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");
        
        let line = line.trim();
        if line == "END" {
            break;
        }
        
        text_input.push_str(line);
        text_input.push(' ');
    }

    if text_input.trim().is_empty() {
        println!("No text entered!");
        return;
    }

    analyzer.analyze_text(&text_input);
    analyzer.display_statistics();
    analyzer.display_top_words(10);

    // Interactive word lookup
    println!("\n=== Word Lookup ===");
    println!("Enter words to check their frequency (type 'quit' to exit):");

    loop {
        print!("word> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let word = input.trim();
        
        if word == "quit" {
            break;
        }

        if word.is_empty() {
            continue;
        }

        let frequency = analyzer.get_word_frequency(word);
        let percentage = analyzer.calculate_word_percentage(word);
        
        if frequency > 0 {
            println!("'{}' appears {} time(s) ({:.1}%)", word, frequency, percentage);
        } else {
            println!("'{}' not found in the text", word);
        }
    }

    println!("Analysis complete!");
}