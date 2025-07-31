# prime_number.py

def is_prime(num):
    """Check if a number is prime."""
    if num <= 1:
        return False
    for i in range(2, int(num**0.5) + 1):
        if num % i == 0:
            return False
    return True

if __name__ == "__main__":
    test_num = 11
    result = is_prime(test_num)
    print(f"{test_num} is {'prime' if result else 'not prime'}")
