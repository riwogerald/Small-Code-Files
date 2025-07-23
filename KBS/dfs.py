class Environment:
    def __init__(self):
        self.mygraph = {
            "1": set(["2", "3"]),
            "2": set(["1", "4"]),
            "3": set(["1", "4"]),
            "4": set(["2", "3"])
        }
        self.state = "2"
        self.goal = "4"

class Agent:
    def __init__(self, environment):
        print("bfs-shortest paths:")
        shortest_paths = self.bfs(environment.mygraph, environment.state, environment.goal)
        for path in shortest_paths:
            print(path)
    
    # This function finds all shortest paths using BFS
    def bfs(self, graph, start, goal):
        if start == goal:
            return [[start]]
        
        queue = [(start, [start])]
        shortest_paths = []
        visited_at_level = {start: 0}
        current_level = 0
        
        while queue:
            vertex, path = queue.pop(0)
            
            # If we've found paths and current path is longer, stop searching
            if shortest_paths and len(path) > len(shortest_paths[0]):
                break
            
            for neighbor in graph[vertex]:
                if neighbor not in path:  # Avoid cycles
                    new_path = path + [neighbor]
                    
                    if neighbor == goal:
                        # Found a path to goal
                        if not shortest_paths or len(new_path) == len(shortest_paths[0]):
                            shortest_paths.append(new_path)
                        elif len(new_path) < len(shortest_paths[0]):
                            shortest_paths = [new_path]
                    else:
                        # Continue exploring if we haven't visited this node at a shorter path length
                        if neighbor not in visited_at_level or visited_at_level[neighbor] >= len(new_path):
                            visited_at_level[neighbor] = len(new_path)
                            queue.append((neighbor, new_path))
        
        return shortest_paths

# Create environment and agent
e1 = Environment()
a1 = Agent(e1)