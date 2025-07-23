import random

totals = [0, 0, 0, 0, 0, 0, 0]  # Or use [0] * 7

def roll_die():  # More accurate name
    return random.randint(1, 6)
   
for i in range(1000):
    dice_total = roll_die()
    totals[dice_total] = totals[dice_total] + 1

# Calculate the percentage frequency of each roll
for i in range(1, 7):
    percentage = 100 * totals[i] / 1000
    print(f"{i} was rolled {percentage:.2f}% of the time ({totals[i]} times)")