# sort_list.py

def sort_numbers(numbers):
    """Sort a list of numbers in ascending order."""
    return sorted(numbers)

def bubble_sort(numbers):
    """Sort using bubble sort algorithm."""
    n = len(numbers)
    arr = numbers.copy()
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr

if __name__ == "__main__":
    test_list = [5, 3, 8, 1, 9, 2]
    sorted_list = sort_numbers(test_list)
    bubble_sorted = bubble_sort(test_list)
    print(f"Original: {test_list}")
    print(f"Sorted (built-in): {sorted_list}")
    print(f"Sorted (bubble sort): {bubble_sorted}")
