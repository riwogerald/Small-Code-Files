#include <iostream>
#include <string>
#include <vector>
#include <random>
#include <algorithm>
#include <cctype>
#include <iomanip>

class PasswordGenerator {
private:
    std::random_device rd;
    std::mt19937 gen;
    
    // Character sets
    const std::string lowercase = "abcdefghijklmnopqrstuvwxyz";
    const std::string uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const std::string digits = "0123456789";
    const std::string symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    const std::string ambiguous = "0O1lI"; // Characters that can be confused
    
public:
    PasswordGenerator() : gen(rd()) {}
    
    struct PasswordConfig {
        int length = 12;
        bool includeLowercase = true;
        bool includeUppercase = true;
        bool includeDigits = true;
        bool includeSymbols = false;
        bool excludeAmbiguous = false;
        int minNumbers = 1;
        int minSymbols = 0;
        bool noRepeatingChars = false;
    };
    
    std::string generatePassword(const PasswordConfig& config) {
        std::string charset = buildCharset(config);
        
        if (charset.empty()) {
            throw std::runtime_error("No character types selected!");
        }
        
        std::string password;
        std::uniform_int_distribution<> dis(0, charset.length() - 1);
        
        // Ensure minimum requirements are met
        password += ensureMinimumRequirements(config);
        
        // Fill remaining length
        while (password.length() < config.length) {
            char nextChar = charset[dis(gen)];
            
            if (config.noRepeatingChars && 
                std::find(password.begin(), password.end(), nextChar) != password.end()) {
                continue;
            }
            
            password += nextChar;
        }
        
        // Shuffle the password to randomize position of required characters
        std::shuffle(password.begin(), password.end(), gen);
        
        return password;
    }
    
    std::vector<std::string> generateMultiple(const PasswordConfig& config, int count) {
        std::vector<std::string> passwords;
        passwords.reserve(count);
        
        for (int i = 0; i < count; ++i) {
            passwords.push_back(generatePassword(config));
        }
        
        return passwords;
    }
    
    double calculateStrength(const std::string& password) {
        double score = 0;
        int charsetSize = 0;
        
        bool hasLower = false, hasUpper = false, hasDigit = false, hasSymbol = false;
        
        for (char c : password) {
            if (std::islower(c)) hasLower = true;
            else if (std::isupper(c)) hasUpper = true;
            else if (std::isdigit(c)) hasDigit = true;
            else hasSymbol = true;
        }
        
        if (hasLower) charsetSize += 26;
        if (hasUpper) charsetSize += 26;
        if (hasDigit) charsetSize += 10;
        if (hasSymbol) charsetSize += 32;
        
        // Calculate entropy: log2(charsetSize^length)
        double entropy = password.length() * std::log2(charsetSize);
        
        // Convert to strength score (0-100)
        score = std::min(100.0, (entropy / 128.0) * 100.0);
        
        return score;
    }
    
    std::string getStrengthLabel(double strength) {
        if (strength < 25) return "Very Weak";
        if (strength < 50) return "Weak";
        if (strength < 75) return "Good";
        if (strength < 90) return "Strong";
        return "Very Strong";
    }

private:
    std::string buildCharset(const PasswordConfig& config) {
        std::string charset;
        
        if (config.includeLowercase) charset += lowercase;
        if (config.includeUppercase) charset += uppercase;
        if (config.includeDigits) charset += digits;
        if (config.includeSymbols) charset += symbols;
        
        if (config.excludeAmbiguous) {
            for (char c : ambiguous) {
                charset.erase(std::remove(charset.begin(), charset.end(), c), charset.end());
            }
        }
        
        return charset;
    }
    
    std::string ensureMinimumRequirements(const PasswordConfig& config) {
        std::string required;
        std::uniform_int_distribution<> lowerDis(0, lowercase.length() - 1);
        std::uniform_int_distribution<> upperDis(0, uppercase.length() - 1);
        std::uniform_int_distribution<> digitDis(0, digits.length() - 1);
        std::uniform_int_distribution<> symbolDis(0, symbols.length() - 1);
        
        // Add required numbers
        for (int i = 0; i < config.minNumbers && config.includeDigits; ++i) {
            required += digits[digitDis(gen)];
        }
        
        // Add required symbols
        for (int i = 0; i < config.minSymbols && config.includeSymbols; ++i) {
            required += symbols[symbolDis(gen)];
        }
        
        return required;
    }
};

void displayMenu() {
    std::cout << "\n========================================\n";
    std::cout << "       PASSWORD GENERATOR TOOL\n";
    std::cout << "========================================\n";
    std::cout << "1. Generate Single Password\n";
    std::cout << "2. Generate Multiple Passwords\n";
    std::cout << "3. Check Password Strength\n";
    std::cout << "4. Custom Password Configuration\n";
    std::cout << "5. Exit\n";
    std::cout << "========================================\n";
    std::cout << "Choose an option (1-5): ";
}

void displayPasswordWithStrength(const std::string& password, PasswordGenerator& generator) {
    double strength = generator.calculateStrength(password);
    std::string strengthLabel = generator.getStrengthLabel(strength);
    
    std::cout << "\nGenerated Password: " << password << "\n";
    std::cout << "Length: " << password.length() << " characters\n";
    std::cout << "Strength: " << std::fixed << std::setprecision(1) 
              << strength << "% (" << strengthLabel << ")\n";
    
    // Visual strength bar
    std::cout << "Strength Bar: [";
    int bars = static_cast<int>(strength / 5);
    for (int i = 0; i < 20; ++i) {
        if (i < bars) std::cout << "=";
        else std::cout << " ";
    }
    std::cout << "]\n";
}

int main() {
    PasswordGenerator generator;
    int choice;
    
    std::cout << "Welcome to the Advanced Password Generator!\n";
    
    while (true) {
        displayMenu();
        std::cin >> choice;
        
        switch (choice) {
            case 1: {
                // Generate single password with default settings
                PasswordGenerator::PasswordConfig config;
                std::string password = generator.generatePassword(config);
                displayPasswordWithStrength(password, generator);
                break;
            }
            
            case 2: {
                // Generate multiple passwords
                int count;
                std::cout << "How many passwords to generate? ";
                std::cin >> count;
                
                PasswordGenerator::PasswordConfig config;
                auto passwords = generator.generateMultiple(config, count);
                
                std::cout << "\nGenerated " << count << " passwords:\n";
                std::cout << "--------------------------------\n";
                for (size_t i = 0; i < passwords.size(); ++i) {
                    std::cout << (i + 1) << ". " << passwords[i] << "\n";
                }
                break;
            }
            
            case 3: {
                // Check password strength
                std::string password;
                std::cout << "Enter password to check: ";
                std::cin >> password;
                
                double strength = generator.calculateStrength(password);
                std::string strengthLabel = generator.getStrengthLabel(strength);
                
                std::cout << "\nPassword Analysis:\n";
                std::cout << "Password: " << password << "\n";
                std::cout << "Length: " << password.length() << " characters\n";
                std::cout << "Strength: " << std::fixed << std::setprecision(1) 
                          << strength << "% (" << strengthLabel << ")\n";
                break;
            }
            
            case 4: {
                // Custom configuration
                PasswordGenerator::PasswordConfig config;
                
                std::cout << "\n--- Custom Password Configuration ---\n";
                std::cout << "Password length (4-128): ";
                std::cin >> config.length;
                config.length = std::max(4, std::min(128, config.length));
                
                char choice;
                std::cout << "Include lowercase letters? (y/n): ";
                std::cin >> choice;
                config.includeLowercase = (choice == 'y' || choice == 'Y');
                
                std::cout << "Include uppercase letters? (y/n): ";
                std::cin >> choice;
                config.includeUppercase = (choice == 'y' || choice == 'Y');
                
                std::cout << "Include digits? (y/n): ";
                std::cin >> choice;
                config.includeDigits = (choice == 'y' || choice == 'Y');
                
                std::cout << "Include symbols? (y/n): ";
                std::cin >> choice;
                config.includeSymbols = (choice == 'y' || choice == 'Y');
                
                std::cout << "Exclude ambiguous characters (0,O,1,l,I)? (y/n): ";
                std::cin >> choice;
                config.excludeAmbiguous = (choice == 'y' || choice == 'Y');
                
                try {
                    std::string password = generator.generatePassword(config);
                    displayPasswordWithStrength(password, generator);
                } catch (const std::exception& e) {
                    std::cout << "Error: " << e.what() << "\n";
                }
                break;
            }
            
            case 5:
                std::cout << "Thank you for using Password Generator!\n";
                return 0;
                
            default:
                std::cout << "Invalid choice! Please try again.\n";
                break;
        }
        
        std::cout << "\nPress Enter to continue...";
        std::cin.ignore();
        std::cin.get();
    }
    
    return 0;
}
