class environment(object):
    mygraph = {
        "1": set(["2", "3"]),
        "2": set(["1", "4"]),
        "3": set(["1", "4"]),
        "4": set(["2", "3"])
    }
    state = "2"
    goal = "4"

class agent(environment):
    # This function looks through all available paths and returns
    def bfs(graph, start, goal):
        stack = [(start, [start])]
        p = []
        while stack:
            (vertex, path) = stack.pop(0)
            for next in graph[vertex] - set(path):
                if next == goal:
                    p.append(path + [next])
                else:
                    stack.append((next, path + [next]))
        return p

    def __init__(self, environment):
        print("bfs-shortest paths:")
        shortest_paths = agent.bfs(environment.mygraph, environment.state, environment.goal)
        for path in shortest_paths:
            print(path)

e1 = environment()
a1 = agent(e1)