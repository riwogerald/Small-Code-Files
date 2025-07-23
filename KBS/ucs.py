class environment(object):
    my_graph = {
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
    state = "1"  # Starting point
    goal = "9"  # End point
    cost = {
        str(["1", "2"]): "3",
        str(["1", "4"]): "5",
        str(["2", "1"]): "3",
        str(["2", "3"]): "5",
        str(["2", "5"]): "7",
        str(["3", "2"]): "5",
        str(["3", "6"]): "9",
        str(["4", "1"]): "5",
        str(["4", "5"]): "9",
        str(["4", "7"]): "11",
        str(["5", "2"]): "7",
        str(["5", "4"]): "9",
        str(["5", "8"]): "13",
        str(["5", "6"]): "11",
        str(["6", "3"]): "9",
        str(["6", "5"]): "11",
        str(["6", "9"]): "15",
        str(["7", "4"]): "11",
        str(["7", "8"]): "15",
        str(["8", "5"]): "13",
        str(["8", "7"]): "15",
        str(["8", "9"]): "17",
        str(["9", "6"]): "15",
        str(["9", "8"]): "17"
    }

class agent(environment):
    def get_cost(path_to_cost):
        i = 0
        path_cost = 0
        while i < len(path_to_cost) - 1:
            l = []
            l.append(path_to_cost[i])
            l.append(path_to_cost[i + 1])
            path_cost = path_cost + int(environment.cost[str(l)])
            i += 1
        return path_cost

    def ucs(graph, start, goal):  # uniform cost search
        stack = [(start, [start])]
        p = []
        least_cost = 1000  # maximum limit cost
        while stack:
            (vertex, path) = stack.pop()
            for next in graph[vertex] - set(path):
                if next == goal:
                    path_cost = agent.get_cost(path + [next])
                    print("ucs path and cost", path + [next], path_cost)
                    if path_cost < least_cost:
                        least_cost = path_cost
                        p = path + [next]
                else:
                    stack.append((next, path + [next]))
        return p

    def __init__(self, environment):
        print("ucs cheapest path:", agent.ucs(environment.my_graph, environment.state, environment.goal))

envt = environment()
agent1 = agent(envt)