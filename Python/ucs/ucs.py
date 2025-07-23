class Environment:
    def __init__(self):
        self.my_graph = {
            "1": set(["2", "4"]),
            "2": set(["1", "3", "5"]),
            "3": set(["2", "6"]),
            "4": set(["1", "5", "7"]),
            "5": set(["2", "4", "6", "8"]),
            "6": set(["3", "5", "9"]),
            "7": set(["4", "8"]),
            "8": set(["5", "7", "9"]),
            "9": set(["6", "8"])
        }
        self.state = "1"  # Starting point
        self.goal = "9"   # End point
        self.cost = {
            "['1', '2']": 3,
            "['1', '4']": 5,
            "['2', '1']": 3,
            "['2', '3']": 5,
            "['2', '5']": 7,
            "['3', '2']": 5,
            "['3', '6']": 9,
            "['4', '1']": 5,
            "['4', '5']": 9,
            "['4', '7']": 11,
            "['5', '2']": 7,
            "['5', '4']": 9,
            "['5', '8']": 13,
            "['5', '6']": 11,
            "['6', '3']": 9,
            "['6', '5']": 11,
            "['6', '9']": 15,
            "['7', '4']": 11,
            "['7', '8']": 15,
            "['8', '5']": 13,
            "['8', '7']": 15,
            "['8', '9']": 17,
            "['9', '6']": 15,
            "['9', '8']": 17
        }


class Agent:
    def __init__(self, environment):
        self.environment = environment
        print("UCS cheapest path:", self.ucs(environment.my_graph, environment.state, environment.goal))

    def get_cost(self, path_to_cost):
        """Calculate the total cost of a path"""
        path_cost = 0
        for i in range(len(path_to_cost) - 1):
            edge = [path_to_cost[i], path_to_cost[i + 1]]
            edge_key = str(edge)
            if edge_key in self.environment.cost:
                path_cost += self.environment.cost[edge_key]
            else:
                return float('inf')  # Invalid path
        return path_cost

    def ucs(self, graph, start, goal):
        """Uniform Cost Search implementation using priority queue"""
        from heapq import heappush, heappop
        
        # Priority queue: (cost, path)
        priority_queue = [(0, [start])]
        visited = set()
        
        while priority_queue:
            current_cost, path = heappop(priority_queue)
            current_node = path[-1]
            
            # Skip if we've already visited this node with a better cost
            if current_node in visited:
                continue
                
            visited.add(current_node)
            
            # Check if we reached the goal
            if current_node == goal:
                print(f"UCS path and cost: {path}, {current_cost}")
                return path
            
            # Explore neighbors
            for neighbor in graph[current_node]:
                if neighbor not in visited:
                    new_path = path + [neighbor]
                    new_cost = self.get_cost(new_path)
                    if new_cost != float('inf'):
                        heappush(priority_queue, (new_cost, new_path))
        
        return []  # No path found


# Alternative implementation using the original stack-based approach (fixed)
class AgentStackBased:
    def __init__(self, environment):
        self.environment = environment
        print("UCS cheapest path (stack-based):", self.ucs(environment.my_graph, environment.state, environment.goal))

    def get_cost(self, path_to_cost):
        """Calculate the total cost of a path"""
        path_cost = 0
        for i in range(len(path_to_cost) - 1):
            edge = [path_to_cost[i], path_to_cost[i + 1]]
            edge_key = str(edge)
            if edge_key in self.environment.cost:
                path_cost += self.environment.cost[edge_key]
            else:
                return float('inf')  # Invalid path
        return path_cost

    def ucs(self, graph, start, goal):
        """Uniform Cost Search using stack (less efficient but closer to original)"""
        all_paths = []
        stack = [(start, [start])]
        
        while stack:
            vertex, path = stack.pop()
            
            # Explore all unvisited neighbors
            for next_node in graph[vertex]:
                if next_node not in path:  # Avoid cycles
                    new_path = path + [next_node]
                    
                    if next_node == goal:
                        path_cost = self.get_cost(new_path)
                        all_paths.append((new_path, path_cost))
                        print(f"Found path: {new_path}, cost: {path_cost}")
                    else:
                        stack.append((next_node, new_path))
        
        # Find the path with minimum cost
        if all_paths:
            best_path = min(all_paths, key=lambda x: x[1])
            return best_path[0]
        return []


# Test the implementations
if __name__ == "__main__":
    print("=== Testing Priority Queue Based UCS ===")
    envt = Environment()
    agent1 = Agent(envt)
    
    print("\n=== Testing Stack Based UCS ===")
    agent2 = AgentStackBased(envt)