import random
import os
from time import sleep

def clear_screen():
    """Clear the terminal screen for better presentation."""
    os.system('cls' if os.name == 'nt' else 'clear')

def display_dice_face(number):
    """Display visual representation for dice numbers 1-6, and text for 7-25."""
    dice_faces = {
        1: [
            "┌─────┐",
            "│     │",
            "│  ●  │",
            "│     │",
            "└─────┘"
        ],
        2: [
            "┌─────┐",
            "│ ●   │",
            "│     │",
            "│   ● │",
            "└─────┘"
        ],
        3: [
            "┌─────┐",
            "│ ●   │",
            "│  ●  │",
            "│   ● │",
            "└─────┘"
        ],
        4: [
            "┌─────┐",
            "│ ● ● │",
            "│     │",
            "│ ● ● │",
            "└─────┘"
        ],
        5: [
            "┌─────┐",
            "│ ● ● │",
            "│  ●  │",
            "│ ● ● │",
            "└─────┘"
        ],
        6: [
            "┌─────┐",
            "│ ● ● │",
            "│ ● ● │",
            "│ ● ● │",
            "└─────┘"
        ]
    }
    
    if number in dice_faces:
        # Display visual dice face
        for line in dice_faces[number]:
            print(line)
    else:
        # Display number in a box for values 7-25
        print("┌─────────┐")
        print(f"│   {number:2d}    │")
        print("│         │")
        print("│  SIDES  │")
        print("└─────────┘")

def animate_roll():
    """Create a simple rolling animation."""
    print("Rolling", end="")
    for _ in range(3):
        sleep(0.3)
        print(".", end="", flush=True)
    print("\n")
    sleep(0.5)

def get_dice_type():
    """Allow user to choose dice type."""
    while True:
        try:
            print("Choose your dice:")
            print("1. Standard 6-sided die")
            print("2. Custom die (7-25 sides)")
            choice = input("Enter choice (1 or 2): ").strip()
            
            if choice == "1":
                return 6
            elif choice == "2":
                sides = int(input("Enter number of sides (7-25): "))
                if 7 <= sides <= 25:
                    return sides
                else:
                    print("Please enter a number between 7 and 25.")
            else:
                print("Please enter 1 or 2.")
        except ValueError:
            print("Please enter a valid number.")

def main():
    """Main game loop."""
    print("🎲 Welcome to the Enhanced Dice Roller! 🎲")
    print("=" * 40)
    
    dice_sides = get_dice_type()
    roll_count = 0
    total_sum = 0
    
    while True:
        clear_screen()
        print(f"🎲 {dice_sides}-Sided Dice Roller 🎲")
        print("=" * 30)
        
        if roll_count > 0:
            print(f"Rolls: {roll_count} | Average: {total_sum/roll_count:.1f}")
            print("-" * 30)
        
        animate_roll()
        
        # Generate random number
        result = random.randint(1, dice_sides)
        roll_count += 1
        total_sum += result
        
        # Display the result
        print(f"You rolled: {result}")
        print()
        display_dice_face(result)
        print()
        
        # Get user input for next action
        while True:
            choice = input("Press 'y' to roll again, 'c' to change dice, or 'n' to exit: ").lower().strip()
            if choice in ['y', 'c', 'n']:
                break
            print("Please enter 'y', 'c', or 'n'.")
        
        if choice == 'n':
            print(f"\nThanks for playing! You made {roll_count} rolls.")
            if roll_count > 1:
                print(f"Total sum: {total_sum} | Average roll: {total_sum/roll_count:.2f}")
            break
        elif choice == 'c':
            dice_sides = get_dice_type()
            roll_count = 0
            total_sum = 0

if __name__ == "__main__":
    main()