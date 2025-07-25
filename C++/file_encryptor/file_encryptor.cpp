#include <iostream>
#include <fstream>
#include <string>

static const char XOR_KEY = 'K'; // Simple XOR key for encryption and decryption

void encrypt(const std::string& inputFile, const std::string& outputFile) {
    std::ifstream in(inputFile, std::ios::binary);
    std::ofstream out(outputFile, std::ios::binary);

    if (!in.is_open() || !out.is_open()) {
        std::cerr << "Error opening files!" << std::endl;
        return;
    }

    char ch;
    while (in.get(ch)) {
        char encrypted = ch ^ XOR_KEY; // Simple XOR encryption
        out.put(encrypted);
    }

    in.close();
    out.close();
    std::cout << "File encrypted successfully!" << std::endl;
}

void decrypt(const std::string& inputFile, const std::string& outputFile) {
    std::ifstream in(inputFile, std::ios::binary);
    std::ofstream out(outputFile, std::ios::binary);

    if (!in.is_open() || !out.is_open()) {
        std::cerr << "Error opening files!" << std::endl;
        return;
    }

    char ch;
    while (in.get(ch)) {
        char decrypted = ch ^ XOR_KEY; // Simple XOR decryption
        out.put(decrypted);
    }

    in.close();
    out.close();
    std::cout << "File decrypted successfully!" << std::endl;
}

int main(int argc, char* argv[]) {
    if (argc != 4) {
        std::cerr << "Usage: " << argv[0] << " <encrypt/decrypt> <input file> <output file>" << std::endl;
        return 1;
    }

    std::string mode = argv[1];
    std::string inputFile = argv[2];
    std::string outputFile = argv[3];

    if (mode == "encrypt") {
        encrypt(inputFile, outputFile);
    } else if (mode == "decrypt") {
        decrypt(inputFile, outputFile);
    } else {
        std::cerr << "Invalid mode! Use 'encrypt' or 'decrypt'." << std::endl;
        return 1;
    }

    return 0;
}
