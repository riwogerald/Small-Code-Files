# fibonacci.py

def fibonacci(n):
    """Generate the first n Fibonacci numbers."""
    if n <= 0:
        return []
    elif n == 1:
        return [0]
    
    fib_sequence = [0, 1]
    for i in range(2, n):
        fib_sequence.append(fib_sequence[-1] + fib_sequence[-2])
    return fib_sequence

def fibonacci_generator(n):
    """Generator function for Fibonacci numbers."""
    a, b = 0, 1
    count = 0
    while count < n:
        yield a
        a, b = b, a + b
        count += 1

if __name__ == "__main__":
    n = 10
    fib_list = fibonacci(n)
    print(f"First {n} Fibonacci numbers: {fib_list}")
    
    print("Using generator:")
    for i, fib in enumerate(fibonacci_generator(n)):
        print(f"F({i}) = {fib}")
