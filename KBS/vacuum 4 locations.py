import random

class Environment(object):
    def __init__(self):
        # instantiate locations and conditions
        # 0 indicates Clean and 1 indicates dirty
        self.location = ["A", "B", "C", "D"]  # Locations
        self.locationcondition = {"A": 0, "B": 0, "C": 0, "D": 0}  # Status of environment
        # Randomize conditions
        self.locationcondition["A"] = random.randint(0, 1)
        self.locationcondition["B"] = random.randint(0, 1)
        self.locationcondition["C"] = random.randint(0, 1)
        self.locationcondition["D"] = random.randint(0, 1)
        # Place vacuum cleaner at a random location
        self.vacuumlocation = random.choice(self.location)


class Agent(Environment):
    def __init__(self, environment):
        print("Environment condition:", environment.locationcondition)
        print("Vacuum location:", environment.vacuumlocation)
        count = 0
        # Set the number of iterations to the number of locations to clean all of them
        while count < len(environment.location):
            if environment.locationcondition[environment.vacuumlocation] == 1:
                # Suck and mark clean
                environment.locationcondition[environment.vacuumlocation] = 0
                print(environment.vacuumlocation, "has been cleaned.")
            else:
                print(environment.vacuumlocation, "is cleaned.")
            new_index = environment.location.index(environment.vacuumlocation) + 1
            if new_index == len(environment.location):
                new_index = 0
            environment.vacuumlocation = environment.location[new_index]
            count += 1
        # Done with cleaning
        print("Final environment condition:", environment.locationcondition)

# Create an environment
e1 = Environment()
# Send the vacuum agent
a1 = Agent(e1)
