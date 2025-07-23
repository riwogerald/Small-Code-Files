import random
import time
import matplotlib.pyplot as plt


class Performance:
    def __init__(self):  # Fixed: __init__ not _init_
        self.score = 0
        self.score_history = []  # List to store score values

    def display(self):
        print("Score:", self.score)
        print("Performance:", (self.score / 48) * 100, "%")

    def add_score(self, score):
        self.score_history.append(score)  # Add score to history


class Environment:
    def __init__(self):  # Fixed: __init__ not _init_
        self.location = ["A", "B"]  # locations
        self.locationcondition = {"A": 0, "B": 0}
        self.locationcondition["A"] = random.randint(0, 1)
        self.locationcondition["B"] = random.randint(0, 1)
        self.vacuumlocation = random.choice(self.location)
        self.mode = ["T", "L"]
        self.cleaningmethod = {"A": "T", "B": "T"}
        self.cleaningmethod["A"] = random.choice(self.mode)
        self.cleaningmethod["B"] = random.choice(self.mode)


class Agent:  # Fixed: Removed inheritance, made standalone class
    def __init__(self, environment, performance):  # Fixed: __init__ not _init_
        print("Environment condition:", environment.locationcondition)
        print("Vacuum location:", environment.vacuumlocation)
        print("Cleaning method:", environment.cleaningmethod)
        
        count = 0
        while count < 2:
            if environment.locationcondition[environment.vacuumlocation] == 1:
                if environment.cleaningmethod[environment.vacuumlocation] == "T":
                    environment.cleaningmethod[environment.vacuumlocation] = "L"
                else:
                    environment.cleaningmethod[environment.vacuumlocation] = "T"
                environment.locationcondition[environment.vacuumlocation] = 0
                print(environment.vacuumlocation, "has been cleaned")
                performance.score += 1
            else:
                print(environment.vacuumlocation, "is clean")
            
            # Move to next location
            newindex = environment.location.index(environment.vacuumlocation) + 1
            if newindex == 2:
                newindex = 0
            environment.vacuumlocation = environment.location[newindex]
            count += 1
        
        print("New conditions:", environment.locationcondition)
        print("Updated cleaning methods:", environment.cleaningmethod)
        
        performance.add_score(performance.score)  # Add current total score to history


thescore = Performance()
x = 0
while x < 24:
    print(f"\n--- Iteration {x + 1} ---")  # Added iteration counter for clarity
    e1 = Environment()
    a1 = Agent(e1, thescore)
    x += 1
    time.sleep(1)

thescore.display()

# Plotting the performance
iterations = list(range(1, 25))
scores = thescore.score_history

plt.figure(figsize=(10, 6))  # Added figure size for better visualization
plt.plot(iterations, scores, marker='o', linewidth=2, markersize=6)
plt.xlabel('Iterations')
plt.ylabel('Cumulative Score')
plt.title('Agent Performance Over Time')
plt.grid(True, alpha=0.3)
plt.xticks(range(1, 25, 2))  # Show every other iteration on x-axis
plt.tight_layout()
plt.show()