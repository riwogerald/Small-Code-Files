import random

class RoverPerformance:
    def _init_(self):
        self.locations_sampled = {'A': 0, 'B': 0, 'C': 0, 'D': 0}
        self.total_ventures = 0

    def update_sampled_locations(self, location):
        self.locations_sampled[location] += 1

    def calculate_performance(self):
        total_locations = sum(self.locations_sampled.values())
        return (total_locations / (4 * self.total_ventures)) * 100

    def print_performance(self):
        performance = self.calculate_performance()
        print(f"Locations Sampled {self.locations_sampled}")
        print(f"Rover's performance {performance}%")
        print()

class MarsEnvironment:
    def _init_(self):
        self.rocks = {'A': 1, 'B': 1, 'C': 1, 'D': 1}

    def gusts_of_wind(self):
        for location in self.rocks:
            if random.random() < 0.5:
                self.rocks[location] = 1
            else:
                self.rocks[location] = 0

    def has_rocks(self, location):
        return self.rocks[location] == 1

class RoverAgent:
    def _init_(self):
        self.performance = RoverPerformance()
        self.environment = MarsEnvironment()
        self.current_location = None

    def explore_location(self, location):
        print(f"Rover is in Location {location}")
        if self.performance.locations_sampled[location] > 0:
            print(f"{location} Location has been Sampled before.")
        else:
            if self.environment.has_rocks(location):
                self.performance.update_sampled_locations(location)
                print(f"{location} Rocks Sampled.")
            else:
                print(f"{location} has no Rocks.")
        print()

    def perform_venture(self):
        self.environment.gusts_of_wind()
        self.performance.total_ventures += 1
        print(f"---- Exploration Venture {self.performance.total_ventures-1} ----")
        for location in ['A', 'B', 'C', 'D']:
            self.explore_location(location)
        self.performance.print_performance()

# Test the program
rover = RoverAgent()
rover.perform_venture()
rover.perform_venture()