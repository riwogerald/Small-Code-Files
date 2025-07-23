import random

totals = [0, 0, 0, 0, 0, 0, 0]

#function that rolls the dice and returns the sum of their rolls
def roll_dice():
   return random.randint(1, 6)
   
for i in range(1000):
    dice_total = roll_dice()
    totals[dice_total] = totals[dice_total] + 1

# Calculate the percentage frequency of each roll
for i in range(1, 7):
    percentage = 100 * totals[i] / 1000
    print(f"{i} was rolled {percentage:.2f}% of the time at" , totals[i],)