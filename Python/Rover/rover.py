import random

class RoverPerformance:
    def __init__(self):  # Fixed: __init__ not _init_
        self.locations_sampled = {'A': 0, 'B': 0, 'C': 0, 'D': 0}
        self.total_ventures = 0

    def update_sampled_locations(self, location):
        self.locations_sampled[location] += 1

    def calculate_performance(self):
        # Fixed: Avoid division by zero and calculate correctly
        if self.total_ventures == 0:
            return 0.0
        
        total_samples = sum(self.locations_sampled.values())
        # Performance as percentage of total possible samples across all ventures
        max_possible_samples = 4 * self.total_ventures
        return (total_samples / max_possible_samples) * 100

    def print_performance(self):
        performance = self.calculate_performance()
        print(f"Locations Sampled: {self.locations_sampled}")
        print(f"Total Ventures: {self.total_ventures}")
        print(f"Rover's performance: {performance:.2f}%")  # Added decimal formatting
        print()

class MarsEnvironment:
    def __init__(self):  # Fixed: __init__ not _init_
        self.rocks = {'A': 1, 'B': 1, 'C': 1, 'D': 1}

    def gusts_of_wind(self):
        """Simulate wind gusts that randomly distribute rocks"""
        print("Wind gusts are reshaping the environment...")
        for location in self.rocks:
            if random.random() < 0.5:
                self.rocks[location] = 1
            else:
                self.rocks[location] = 0
        
        # Display current rock distribution
        rock_status = [loc for loc, has_rock in self.rocks.items() if has_rock == 1]
        print(f"Rocks now present at: {rock_status if rock_status else 'No locations'}")
        print()

    def has_rocks(self, location):
        return self.rocks[location] == 1

class RoverAgent:
    def __init__(self):  # Fixed: __init__ not _init_
        self.performance = RoverPerformance()
        self.environment = MarsEnvironment()
        self.current_location = None

    def explore_location(self, location):
        self.current_location = location
        print(f"Rover is exploring Location {location}...")
        
        if self.performance.locations_sampled[location] > 0:
            print(f"  Location {location} has been sampled before (Count: {self.performance.locations_sampled[location]})")
            # Still check if there are new rocks, but don't sample again in this venture
            if self.environment.has_rocks(location):
                print(f"  New rocks detected at {location}, but location already sampled in previous venture")
            else:
                print(f"  No rocks at {location}")
        else:
            if self.environment.has_rocks(location):
                self.performance.update_sampled_locations(location)
                print(f"  ✓ Rocks found and sampled at Location {location}!")
            else:
                print(f"  No rocks found at Location {location}")
        print()

    def perform_venture(self):
        # Increment venture counter first
        self.performance.total_ventures += 1
        print(f"==== Exploration Venture {self.performance.total_ventures} ====")
        
        # Generate new environment
        self.environment.gusts_of_wind()
        
        # Explore all locations
        for location in ['A', 'B', 'C', 'D']:
            self.explore_location(location)
        
        # Display performance
        self.performance.print_performance()
        print("-" * 50)

# Test the program
print("Mars Rover Exploration Simulation")
print("=" * 50)

rover = RoverAgent()
rover.perform_venture()
rover.perform_venture()

print("Simulation Complete!")
print(f"Final Summary:")
print(f"Total ventures completed: {rover.performance.total_ventures}")
print(f"Unique locations sampled: {sum(1 for count in rover.performance.locations_sampled.values() if count > 0)}")
print(f"Total samples collected: {sum(rover.performance.locations_sampled.values())}")