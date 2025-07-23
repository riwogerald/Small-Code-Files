def reverse_number(num):
  sign = '-' if num < 0 else ''
  reversed_str = ''.join(reversed(str(abs(num))))
  return int(sign + reversed_str)

num = int(input("Enter a Number: "))
print("Reversed Number", reverse_number(num))