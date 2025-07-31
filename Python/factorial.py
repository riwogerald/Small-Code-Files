# factorial.py

def factorial(n):
    """Calculate the factorial of a number using recursion."""
    if n < 0:
        return None  # Factorial is not defined for negative numbers
    if n == 0 or n == 1:
        return 1
    return n * factorial(n - 1)

def factorial_iterative(n):
    """Calculate the factorial of a number using iteration."""
    if n < 0:
        return None
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result

if __name__ == "__main__":
    test_num = 5
    recursive_result = factorial(test_num)
    iterative_result = factorial_iterative(test_num)
    
    print(f"Factorial of {test_num} (recursive): {recursive_result}")
    print(f"Factorial of {test_num} (iterative): {iterative_result}")
