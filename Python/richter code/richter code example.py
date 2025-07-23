def evaluate_richter_scale(value):
    """Evaluates the magnitude description based on the Richter scale value."""
    if value > 7.5:
        print(f"Magnitude: {value} Catastrophic")
    elif 6.5 <= value <= 7.5:
        print(f"Magnitude: {value} Disaster")
    elif 5.5 <= value < 6.5:
        print(f"Magnitude: {value} Serious Damage")
    elif 5.0 <= value < 5.5:
        print(f"Magnitude: {value} Some Damage")
    else:
        print(f"Magnitude: {value} Little or No Damage")

def get_category(value):
    """Returns the category character based on the Richter scale value."""
    if value > 9.9:
        return 'h'
    elif value > 7.9:
        return 'g'
    elif value > 6.9:
        return 'f'
    elif value > 5.9:
        return 'e'
    elif value > 4.9:
        return 'd'
    elif value > 3.9:
        return 'c'
    elif value > 2.0:
        return 'b'
    else:
        return 'a'

def main():
    # Get the first Richter scale value
    x = float(input("What is the measure of the first Richter Scale value? "))
    evaluate_richter_scale(x)

    # Loop to ask for the next value
    while True:
        y = float(input("What is the next value (-1 to quit)? "))
        
        if y == -1:
            break  # Exit loop if user inputs -1
        
        category = get_category(y)
        print(f"Richter scale value {y} falls under category '{category}'.")

if __name__ == "__main__":
    main()
