import heapq
from collections import defaultdict

class SurveillanceRobot:
    def __init__(self):
        # Define the adjacency matrix to represent the graph
        self.graph = [
            # 0   1   2   3   4   5   6   7   8   9   10  11
            [0, 1, 1, 1, 1, 2, 2, 0, 0, 0, 0, 0],  # 0
            [1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0],  # 1
            [1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0],  # 2
            [1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0],  # 3
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0],  # 4
            [2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],  # 5
            [2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],  # 6
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],  # 7
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  # 8
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],  # 9
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],  # 10
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],  # 11
        ]
        
        # Node positions for more accurate heuristic (assuming a logical layout)
        self.positions = {
            0: (0, 0), 1: (0, 1), 2: (0, 2), 3: (0, 3), 4: (0, 4),
            5: (1, 0), 6: (1, 1), 7: (1, 2), 8: (2, 0), 9: (2, 1), 
            10: (2, 2), 11: (2, 3)
        }

    def heuristic(self, current, goal):
        """Manhattan distance heuristic using actual node positions"""
        x1, y1 = self.positions[current]
        x2, y2 = self.positions[goal]
        return abs(x1 - x2) + abs(y1 - y2)

    def get_neighbors(self, node):
        """Get all neighbors of a node with their edge weights"""
        neighbors = []
        for i in range(len(self.graph)):
            if self.graph[node][i] > 0:  # There's an edge
                neighbors.append((i, self.graph[node][i]))
        return neighbors

    def a_star_search(self, start, goal):
        """A* search algorithm - finds optimal path with lowest cost"""
        if start == goal:
            return [start], 0
            
        # Priority queue: (f_score, g_score, node)
        open_set = [(0, 0, start)]
        came_from = {}
        g_score = defaultdict(lambda: float('inf'))
        g_score[start] = 0
        closed_set = set()

        while open_set:
            f_current, g_current, current = heapq.heappop(open_set)
            
            if current in closed_set:
                continue
                
            closed_set.add(current)
            
            if current == goal:
                # Reconstruct path
                path = []
                while current is not None:
                    path.append(current)
                    current = came_from.get(current)
                return list(reversed(path)), g_score[goal]

            for neighbor, weight in self.get_neighbors(current):
                if neighbor in closed_set:
                    continue
                    
                tentative_g_score = g_score[current] + weight
                
                if tentative_g_score < g_score[neighbor]:
                    came_from[neighbor] = current
                    g_score[neighbor] = tentative_g_score
                    f_score = tentative_g_score + self.heuristic(neighbor, goal)
                    heapq.heappush(open_set, (f_score, tentative_g_score, neighbor))

        return None, float('inf')  # No path found

    def dijkstra_search(self, start, goal):
        """Dijkstra's algorithm - guaranteed shortest path by cost"""
        if start == goal:
            return [start], 0
            
        # Priority queue: (cost, node)
        pq = [(0, start)]
        distances = defaultdict(lambda: float('inf'))
        distances[start] = 0
        came_from = {}
        visited = set()

        while pq:
            current_dist, current = heapq.heappop(pq)
            
            if current in visited:
                continue
                
            visited.add(current)
            
            if current == goal:
                # Reconstruct path
                path = []
                while current is not None:
                    path.append(current)
                    current = came_from.get(current)
                return list(reversed(path)), distances[goal]

            for neighbor, weight in self.get_neighbors(current):
                if neighbor in visited:
                    continue
                    
                new_dist = current_dist + weight
                if new_dist < distances[neighbor]:
                    distances[neighbor] = new_dist
                    came_from[neighbor] = current
                    heapq.heappush(pq, (new_dist, neighbor))

        return None, float('inf')  # No path found

    def bfs_shortest_path(self, start, goal):
        """BFS - finds path with minimum number of hops (stopovers)"""
        if start == goal:
            return [start], 0
            
        from collections import deque
        queue = deque([(start, [start])])
        visited = {start}

        while queue:
            current, path = queue.popleft()
            
            for neighbor, _ in self.get_neighbors(current):
                if neighbor == goal:
                    return path + [neighbor], len(path)
                    
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append((neighbor, path + [neighbor]))

        return None, float('inf')  # No path found

    def find_all_paths(self, start, goal, max_length=10):
        """Find all possible paths from start to goal"""
        def dfs_all_paths(current, target, path, all_paths, visited):
            if len(path) > max_length:  # Prevent infinite loops
                return
                
            if current == target:
                all_paths.append(path.copy())
                return
                
            for neighbor, weight in self.get_neighbors(current):
                if neighbor not in visited:
                    visited.add(neighbor)
                    path.append(neighbor)
                    dfs_all_paths(neighbor, target, path, all_paths, visited)
                    path.pop()
                    visited.remove(neighbor)

        all_paths = []
        visited = {start}
        dfs_all_paths(start, goal, [start], all_paths, visited)
        
        # Calculate costs for each path
        paths_with_costs = []
        for path in all_paths:
            cost = 0
            for i in range(len(path) - 1):
                cost += self.graph[path[i]][path[i + 1]]
            paths_with_costs.append((path, cost, len(path) - 1))  # (path, cost, hops)
            
        return paths_with_costs

    def get_reachable_nodes(self, start):
        """DFS to find all nodes reachable from start"""
        visited = set()
        path = []
        
        def dfs(node):
            visited.add(node)
            path.append(node)
            
            for neighbor, _ in self.get_neighbors(node):
                if neighbor not in visited:
                    dfs(neighbor)
        
        dfs(start)
        return path

    def verify_heuristic_consistency(self):
        """Verify that the heuristic is consistent (admissible)"""
        print("\nVerifying heuristic consistency:")
        print("For consistency: h(n) <= cost(n,n') + h(n') for all neighbors n'")
        
        inconsistencies = []
        for node in range(12):
            for neighbor, actual_cost in self.get_neighbors(node):
                for goal in range(12):
                    h_current = self.heuristic(node, goal)
                    h_neighbor = self.heuristic(neighbor, goal)
                    
                    if h_current > actual_cost + h_neighbor:
                        inconsistencies.append(
                            (node, neighbor, goal, h_current, actual_cost + h_neighbor)
                        )
        
        if not inconsistencies:
            print("✓ Heuristic is consistent!")
            return True
        else:
            print("✗ Heuristic inconsistencies found:")
            for node, neighbor, goal, h_curr, cost_plus_h in inconsistencies[:5]:
                print(f"  Node {node}→{neighbor} (goal {goal}): h({node}) = {h_curr} > {cost_plus_h} = cost + h({neighbor})")
            return False

def print_path_info(description, result):
    """Helper function to print path information"""
    if result[0] is None:
        print(f"{description}: No path found")
    else:
        path, cost = result
        hops = len(path) - 1
        print(f"{description}: {' → '.join(map(str, path))} (Cost: {cost}, Hops: {hops})")

# Sample usage and testing
if __name__ == "__main__":
    robot = SurveillanceRobot()
    start_node = 0
    goal_node = 10
    
    print(f"Surveillance Robot Pathfinding: Node {start_node} to Node {goal_node}")
    print("=" * 60)

    # Task (i): Find the path with the fewest stopovers (minimum hops)
    quickest_path = robot.bfs_shortest_path(start_node, goal_node)
    print_path_info("Path with fewest stopovers (BFS)", quickest_path)

    # Task (ii): Find all possible paths
    print(f"\nAll possible paths from {start_node} to {goal_node}:")
    all_paths = robot.find_all_paths(start_node, goal_node)
    if all_paths:
        for i, (path, cost, hops) in enumerate(all_paths):
            print(f"  Path {i+1}: {' → '.join(map(str, path))} (Cost: {cost}, Hops: {hops})")
    else:
        print("  No paths found")

    # Task (iii): Find the cheapest path (lowest energy consumption)
    cheapest_path_dijkstra = robot.dijkstra_search(start_node, goal_node)
    print_path_info("\nCheapest path (Dijkstra)", cheapest_path_dijkstra)
    
    cheapest_path_astar = robot.a_star_search(start_node, goal_node)
    print_path_info("Cheapest path (A*)", cheapest_path_astar)

    # Task (iv): Show all reachable nodes
    reachable = robot.get_reachable_nodes(start_node)
    print(f"\nAll nodes reachable from {start_node}: {reachable}")

    # Task (v): Verify heuristic consistency
    robot.verify_heuristic_consistency()
    
    # Additional analysis
    print(f"\nPath Analysis Summary:")
    print("-" * 30)
    if all_paths:
        costs = [cost for _, cost, _ in all_paths]
        hops = [h for _, _, h in all_paths]
        print(f"Total paths found: {len(all_paths)}")
        print(f"Cost range: {min(costs)} - {max(costs)}")
        print(f"Hops range: {min(hops)} - {max(hops)}")
        
        # Find best paths by different criteria
        min_cost_path = min(all_paths, key=lambda x: x[1])
        min_hops_path = min(all_paths, key=lambda x: x[2])
        
        print(f"\nOptimal paths:")
        print(f"Minimum cost: {' → '.join(map(str, min_cost_path[0]))} (Cost: {min_cost_path[1]})")
        print(f"Minimum hops: {' → '.join(map(str, min_hops_path[0]))} (Hops: {min_hops_path[2]})")