def multiplication_table():
    # Outer loop for rows
    for i in range(1, 11):  # From 1 to 10 inclusive
        # Inner loop for columns
        for j in range(1, 11):
            print(i * j, end="\t")  # Use tab to format the table horizontally
        print()  # New line after each row

# Call the function to display the multiplication table
multiplication_table()
