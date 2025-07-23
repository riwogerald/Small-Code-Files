from tkinter import simpledialog, messagebox, Tk

def get_fraction(order):
    numerator = int(simpledialog.askstring("Input", f"Enter the numerator for the {order} number"))
    denominator = 0

    # Loop to ensure the denominator is valid
    while True:
        denominator = int(simpledialog.askstring("Input", f"Enter the denominator for the {order} number"))
        if denominator > 0:
            break
        messagebox.showerror("Error", "The denominator cannot be zero or negative. Please try again.")

    return numerator / denominator

def main():
    root = Tk()  # Initialize Tkinter
    root.withdraw()  # Hide the root window

    num1 = get_fraction("first")
    num2 = get_fraction("second")

    total = num1 + num2
    messagebox.showinfo("Result", f"Sum of {num1} + {num2} is {total}")

if __name__ == "__main__":
    main()
