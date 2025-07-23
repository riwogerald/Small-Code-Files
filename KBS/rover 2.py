import random

class RoverPerformance:
    def _init_(self):
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
        return f"Sampling percentage: {percentage:.2f}%\nTotal samples: {self.total_samples}\nLocations sampled: {', '.join(self.locations_sampled)}"

class MarsEnvironment:
    def _init_(self, locations):
        self.locations = locations
        self.rocks_present = {location: False for location in locations}

    def generate_environment(self):
        for location in self.locations:
            if random.random() < 0.5:  # 50% chance of rocks being present
                self.rocks_present[location] = True

    def is_rock_present(self, location):
        return self.rocks_present[location]

class RoverAgent:
    def _init_(self, environment, performance):
        self.environment = environment
        self.performance = performance

    def explore(self, num_ventures, num_locations):
        for _ in range(num_ventures):
            self.environment.generate_environment()
            for _ in range(num_locations):
                location = random.choice(list(self.environment.locations))
                if self.environment.is_rock_present(location):
                    sampled = self.performance.sample_location(location)
                    if sampled:
                        print(f"Sampled rock at location {location}")
                else:
                    print(f"No rock found at location {location}")


# Example usage
locations = ["A", "B", "C", "D"]

performance = RoverPerformance()
environment = MarsEnvironment(locations)
rover = RoverAgent(environment, performance)

rover.explore(num_ventures=2, num_locations=4)

print("\nRover Performance:")
print(performance.get_performance_percentage(len(locations)))