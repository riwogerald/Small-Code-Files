import random

class RoverPerformance:
    def __init__(self):  # Fixed: __init__ not _init_
        self.total_samples = 0
        self.locations_sampled = set()

    def sample_location(self, location):
        if location in self.locations_sampled:
            return False
        self.locations_sampled.add(location)
        self.total_samples += 1
        return True

    def get_performance_percentage(self, num_locations):
        percentage = (self.total_samples / num_locations) * 100
        return f"Sampling percentage: {percentage:.2f}%\nTotal samples: {self.total_samples}\nLocations sampled: {', '.join(sorted(self.locations_sampled))}"

class MarsEnvironment:
    def __init__(self, locations):  # Fixed: __init__ not _init_
        self.locations = locations
        self.rocks_present = {location: False for location in locations}

    def generate_environment(self):
        # Reset environment before generating new rock distribution
        for location in self.locations:
            self.rocks_present[location] = False
        
        # Generate new rock distribution
        for location in self.locations:
            if random.random() < 0.5:  # 50% chance of rocks being present
                self.rocks_present[location] = True

    def is_rock_present(self, location):
        return self.rocks_present[location]
    
    def display_environment(self):
        """Display current rock distribution for debugging"""
        print("Current environment:")
        for location, has_rock in self.rocks_present.items():
            status = "Rock present" if has_rock else "No rock"
            print(f"  Location {location}: {status}")

class RoverAgent:
    def __init__(self, environment, performance):  # Fixed: __init__ not _init_
        self.environment = environment
        self.performance = performance

    def explore(self, num_ventures, num_locations_per_venture):
        print(f"Starting exploration: {num_ventures} ventures, {num_locations_per_venture} locations per venture\n")
        
        for venture in range(num_ventures):
            print(f"--- Venture {venture + 1} ---")
            self.environment.generate_environment()
            self.environment.display_environment()
            
            for check in range(num_locations_per_venture):
                location = random.choice(list(self.environment.locations))
                print(f"Checking location {location}...", end=" ")
                
                if self.environment.is_rock_present(location):
                    sampled = self.performance.sample_location(location)
                    if sampled:
                        print(f"Rock found! Successfully sampled at location {location}")
                    else:
                        print(f"Rock found, but location {location} already sampled previously")
                else:
                    print(f"No rock found at location {location}")
            print()  # Empty line between ventures


# Example usage
locations = ["A", "B", "C", "D"]

performance = RoverPerformance()
environment = MarsEnvironment(locations)
rover = RoverAgent(environment, performance)

# Fixed parameter name for clarity
rover.explore(num_ventures=2, num_locations_per_venture=4)

print("=" * 50)
print("FINAL ROVER PERFORMANCE:")
print(performance.get_performance_percentage(len(locations)))
print("=" * 50)