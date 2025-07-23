import random

class Environment:
    def __init__(self):
        # instantiate locations and conditions
        # 0 indicates Clean and 1 indicates dirty
        self.locations = ["A", "B", "C", "D"]  # Locations
        self.location_condition = {"A": 0, "B": 0, "C": 0, "D": 0}  # Status of environment
        
        # Randomize conditions
        for location in self.locations:
            self.location_condition[location] = random.randint(0, 1)
            
        # Place vacuum cleaner at a random location
        self.vacuum_location = random.choice(self.locations)
        
    def is_dirty(self, location):
        """Check if a location is dirty"""
        return self.location_condition[location] == 1
    
    def clean_location(self, location):
        """Clean a specific location"""
        self.location_condition[location] = 0
    
    def is_all_clean(self):
        """Check if all locations are clean"""
        return all(condition == 0 for condition in self.location_condition.values())
    
    def get_next_location(self, current_location):
        """Get the next location in sequence (circular)"""
        current_index = self.locations.index(current_location)
        next_index = (current_index + 1) % len(self.locations)
        return self.locations[next_index]
    
    def move_vacuum(self, new_location):
        """Move vacuum to a new location"""
        self.vacuum_location = new_location


class Agent:
    def __init__(self, environment):
        self.environment = environment
        self.actions_taken = 0
        self.locations_visited = []
        
        print("Initial environment condition:", environment.location_condition)
        print("Initial vacuum location:", environment.vacuum_location)
        print("=" * 50)
        
        self.clean_environment()
        
        print("=" * 50)
        print("Final environment condition:", environment.location_condition)
        print(f"Total actions taken: {self.actions_taken}")
        print(f"Locations visited: {self.locations_visited}")
    
    def clean_environment(self):
        """Clean the environment using a simple reflex agent"""
        max_iterations = len(self.environment.locations) * 2  # Safety limit
        iterations = 0
        
        while not self.environment.is_all_clean() and iterations < max_iterations:
            current_location = self.environment.vacuum_location
            self.locations_visited.append(current_location)
            
            print(f"Step {iterations + 1}: Vacuum at location {current_location}")
            
            # Check if current location is dirty
            if self.environment.is_dirty(current_location):
                # Clean the location
                self.environment.clean_location(current_location)
                print(f"  Action: SUCK - {current_location} has been cleaned.")
                self.actions_taken += 1
            else:
                print(f"  Observation: {current_location} is already clean.")
            
            # Check if all locations are clean
            if self.environment.is_all_clean():
                print("  All locations are now clean. Task completed!")
                break
            
            # Move to next location
            next_location = self.environment.get_next_location(current_location)
            self.environment.move_vacuum(next_location)
            print(f"  Action: MOVE to {next_location}")
            self.actions_taken += 1
            
            iterations += 1
        
        if iterations >= max_iterations:
            print("Warning: Reached maximum iterations limit!")


class SmartAgent:
    """A more intelligent agent that only visits dirty locations"""
    
    def __init__(self, environment):
        self.environment = environment
        self.actions_taken = 0
        self.locations_visited = []
        
        print("=== SMART AGENT ===")
        print("Initial environment condition:", environment.location_condition)
        print("Initial vacuum location:", environment.vacuum_location)
        print("=" * 50)
        
        self.clean_environment_smart()
        
        print("=" * 50)
        print("Final environment condition:", environment.location_condition)
        print(f"Total actions taken: {self.actions_taken}")
        print(f"Locations visited: {self.locations_visited}")
    
    def get_dirty_locations(self):
        """Get list of all dirty locations"""
        return [loc for loc, condition in self.environment.location_condition.items() if condition == 1]
    
    def find_nearest_dirty_location(self, current_location):
        """Find the nearest dirty location (simplified - just returns first dirty location)"""
        dirty_locations = self.get_dirty_locations()
        if not dirty_locations:
            return None
        
        # For simplicity, just return the first dirty location
        # In a real implementation, you'd calculate actual distances
        return dirty_locations[0]
    
    def clean_environment_smart(self):
        """Clean environment by only visiting dirty locations"""
        step = 1
        
        while not self.environment.is_all_clean():
            current_location = self.environment.vacuum_location
            self.locations_visited.append(current_location)
            
            print(f"Step {step}: Vacuum at location {current_location}")
            
            # If current location is dirty, clean it
            if self.environment.is_dirty(current_location):
                self.environment.clean_location(current_location)
                print(f"  Action: SUCK - {current_location} has been cleaned.")
                self.actions_taken += 1
            
            # Check if all locations are clean
            if self.environment.is_all_clean():
                print("  All locations are now clean. Task completed!")
                break
            
            # Find next dirty location
            next_dirty = self.find_nearest_dirty_location(current_location)
            if next_dirty and next_dirty != current_location:
                self.environment.move_vacuum(next_dirty)
                print(f"  Action: MOVE to {next_dirty}")
                self.actions_taken += 1
            
            step += 1


# Test both agents
if __name__ == "__main__":
    print("Testing Simple Reflex Agent:")
    print("=" * 60)
    
    # Create an environment
    e1 = Environment()
    # Send the simple vacuum agent
    a1 = Agent(e1)
    
    print("\n" + "=" * 60)
    print("Testing Smart Agent on same environment:")
    print("=" * 60)
    
    # Reset the environment to original state for fair comparison
    e2 = Environment()
    # Copy the same initial conditions
    e2.location_condition = e1.location_condition.copy()
    e2.vacuum_location = e1.vacuum_location
    
    # Test smart agent
    a2 = SmartAgent(e2)