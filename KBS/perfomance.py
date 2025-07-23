import random
import time
import matplotlib.pyplot as plt


class Performance:
    def _init_(self):
        self.score = 0
        self.score_history = []  # List to store score values

    def display(self):
        print("Score:", self.score)
        print("Performance:", (self.score / 48) * 100, "%")

    def add_score(self, score):
        self.score_history.append(score)  # Add score to history


class Environment:
    def _init_(self):
        self.location = ["A", "B"]  # locations
        self.locationcondition = {"A": 0, "B": 0}
        self.locationcondition["A"] = random.randint(0, 1)
        self.locationcondition["B"] = random.randint(0, 1)
        self.vacuumlocation = random.choice(self.location)
        self.mode = ["T", "L"]
        self.cleaningmethod = {"A": "T", "B": "T"}
        self.cleaningmethod["A"] = random.choice(self.mode)
        self.cleaningmethod["B"] = random.choice(self.mode)


class Agent(Environment, Performance):
    def _init_(self, environment, performance):
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
            newindex = environment.location.index(environment.vacuumlocation) + 1
            if newindex == 2:
                newindex = 0
            environment.vacuumlocation = environment.location[newindex]
            count += 1
        print("New conditions:", environment.locationcondition)
        print("Updated history:", environment.cleaningmethod)

        performance.add_score(performance.score)  # Add score to performance history


thescore = Performance()
x = 0
while x < 24:
    e1 = Environment()
    a1 = Agent(e1, thescore)
    x += 1
    time.sleep(1)

thescore.display()

# Plotting the performance
iterations = list(range(1, 25))
scores = thescore.score_history

plt.plot(iterations, scores, marker='o')
plt.xlabel('Iterations')
plt.ylabel('Score')
plt.title('Agent Performance')
plt.grid(True)
plt.show()