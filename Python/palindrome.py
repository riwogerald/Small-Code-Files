# palindrome.py

def is_palindrome(s):
    """Check if a string is a palindrome."""
    s = s.lower().replace(" ", "")  # Convert to lowercase and remove spaces
    return s == s[::-1]

if __name__ == "__main__":
    test_string = "radar"
    result = is_palindrome(test_string)
    print(f"'{test_string}' is {'a palindrome' if result else 'not a palindrome'}")
