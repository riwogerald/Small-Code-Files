#include <iostream>
#include <vector>
#include <algorithm>
#include <random>
#include <chrono>
#include <thread>

class MemoryGame {
private:
    std::vector<int> sequence;
    std::mt19937 rng;
    int score;
public:
    MemoryGame() : rng(std::random_device{}()), score(0) {}

    void start() {
        std::cout << "Welcome to the Memory Game!\n";
        int round = 1;

        while (true) {
            std::cout << "\nRound " << round << "\n";
            addRandomNumber();
            displaySequence();
            if (!playerTurn()) {
                std::cout << "Game Over! Your final score is " << score << ".\n";
                break;
            }
            ++score;
            ++round;
        }
    }

private:
    void addRandomNumber() {
        std::uniform_int_distribution<int> dist(0, 9);
        sequence.push_back(dist(rng));
    }

    void displaySequence() {
        std::cout << "Memorize this sequence: ";
        for (int num : sequence) {
            std::cout << num << " ";
        }
        std::cout << "\r";
        std::this_thread::sleep_for(std::chrono::seconds(3));
        std::cout << std::string(50, ' ') << "\r";
    }

    bool playerTurn() {
        std::cout << "Enter the sequence: ";
        for (int num : sequence) {
            int input;
            std::cin >> input;
            if (input != num) return false;
        }
        return true;
    }
};

int main() {
    MemoryGame game;
    game.start();
    return 0;
}

