import random

class Environment(object):
    def __init__(self):
        self.locations = ["A", "B", "C", "D"]  # locations
        self.location_conditions = {location: random.randint(0, 1) for location in self.locations}
        self.vacuum_location = random.choice(self.locations)
        self.modes = ["T", "L"]
        
        # Assume past record is all locations cleaned with T
        self.cleaning_method = {location: "T" for location in self.locations}
        
        # Randomly generate initial cleaning history
        for location in self.locations:
            self.cleaning_method[location] = random.choice(self.modes)

class Agent(Environment):
    def __init__(self, environment):
        super().__init__()
        
        # Randomly generate status
        print("Environment condition:", environment.location_conditions)
        print("Vacuum location:", environment.vacuum_location)
        print("Cleaning method:", environment.cleaning_method)
        
        count = 0
        while count < 4:
            if environment.location_conditions[environment.vacuum_location] == 1:
                print(environment.vacuum_location, "is dirty.")
                
                # Check status, suck, mark clean, record status
                if environment.cleaning_method[environment.vacuum_location] == "T":
                    environment.cleaning_method[environment.vacuum_location] = "L"
                else:
                    environment.cleaning_method[environment.vacuum_location] = "T"
                
                environment.location_conditions[environment.vacuum_location] = 0
                print(environment.vacuum_location, "has been cleaned.")
            else:
                print(environment.vacuum_location, "is clean.")
            
            new_index = environment.locations.index(environment.vacuum_location) + 1
            if new_index == 4:
                new_index = 0
            
            environment.vacuum_location = environment.locations[new_index]
            count += 1
        
        print("New conditions:", environment.location_conditions)
        print("Updated history:", environment.cleaning_method)

e1 = Environment()
a1 = Agent(e1)
